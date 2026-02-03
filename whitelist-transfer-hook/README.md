# Whitelist Transfer Hook
This example demonstrates how to implement a transfer hook using the SPL Token 2022 Transfer Hook interface to enforce whitelist restrictions on token transfers.
In this example, only whitelisted addresses will be able to transfer tokens that have this transfer hook enabled, providing fine-grained access control over token movements.
---
## Let's walk through the architecture:
For this program, we will have multiple state accounts:
- Whitelist accounts (one per managed user address)

A Whitelist account consists of:
```rust
#[account]
pub struct Whitelist {
    pub user: Pubkey,
    pub is_whitelisted: bool,
    pub bump: u8,
}
```
### In this state account, we will store:
- user: The public key of the user this whitelist entry corresponds to.
- is_whitelisted: A boolean indicating whether the user is allowed to transfer tokens.
- bump: The bump seed used to derive the whitelist PDA.

The account has a fixed size since it stores a single user's status.
---
### The admin will be able to create Whitelist accounts for users (effectively adding them to the whitelist):
```rust
#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is the user address being whitelisted; we only use its key
    pub user: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [b"whitelist", user.key().as_ref()],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}
```
Let's have a closer look at the accounts that we are passing in this context:
- admin: The signer creating the whitelist entry and paying for account creation.
- user: The address being whitelisted (unchecked because we only need its public key for PDA derivation).
- whitelist: The new state account being initialized, derived as a PDA from the seeds `"whitelist"` and the user's public key.
- system_program: Required for account allocation and initialization.

### Functionality for InitializeWhitelist:
```rust
impl<'info> InitializeWhitelist<'info> {
    pub fn initialize_whitelist(&mut self, bumps: InitializeWhitelistBumps) -> Result<()> {
        self.whitelist.set_inner(Whitelist {
            is_whitelisted: true,
            user: self.user.key(),
            bump: bumps.whitelist,
        });
        Ok(())
    }
}
```
This instruction creates the Whitelist account for the specified user and immediately marks them as whitelisted.
---
### The admin will be able to update the whitelist status of existing users:
```rust
#[derive(Accounts)]
pub struct UpdateWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account()]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"whitelist", whitelist.user.as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}
```
Accounts in this context:
- admin: The signer performing the update (marked mutable in case future extensions require fees).
- mint: The token mint associated with this transfer hook.
- whitelist: The existing Whitelist account to modify. The seeds are validated using the stored `user` field inside the account data.

### Functionality for UpdateWhitelist:
```rust
impl<'info> UpdateWhitelist<'info> {
    pub fn update_whitelist(&mut self, is_whitelisted: bool) -> Result<()> {
        self.whitelist.is_whitelisted = is_whitelisted;
        msg!("Whitelist updated: {} is_whitelisted = {}", self.whitelist.user, is_whitelisted);
        Ok(())
    }
}
```
This instruction updates the `is_whitelisted` flag for the user associated with the provided Whitelist PDA. No account reallocation is needed because the account size is fixed.
---
### The system will need to initialize extra account metadata for the transfer hook:
```rust
#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        ).unwrap(),
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}
```
### Functionality for InitializeExtraAccountMetaList:
```rust
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(
            vec![
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"whitelist".to_vec(),
                        },
                        Seed::AccountKey { index: 3 }, // source_token.owner
                    ],
                    false, // is_signer
                    false // is_writable
                ).unwrap(),
            ]
        )
    }
}
```
This defines a single extra account: the Whitelist PDA derived from the source token owner's public key. The token program will automatically append this account to every transfer hook execution.
---
### The transfer hook will validate every token transfer:
```rust
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    #[account(
        address = source_token.owner
    )]
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist", owner.key().as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}
```
Accounts in this context:
- source_token: The originating token account.
- mint: The token mint.
- destination_token: The receiving token account.
- owner: The owner of the source token account (constrained to match `source_token.owner`).
- extra_account_meta_list: The metadata account defining required extra accounts.
- whitelist: The Whitelist account for the source owner, derived from the owner's public key.

### Functionality for TransferHook:
```rust
impl<'info> TransferHook<'info> {
    /// This function is called when the transfer hook is executed.
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        self.check_is_transferring()?;
        msg!("Source token owner: {}", self.source_token.owner);
        msg!("Destination token owner: {}", self.destination_token.owner);
        if self.whitelist.is_whitelisted {
            msg!("Transfer allowed: The user is whitelisted");
        } else {
            panic!("TransferHook: user is not whitelisted");
        }
        Ok(())
    }

    /// Checks if the transfer hook is being executed during a transfer operation.
    fn check_is_transferring(&mut self) -> Result<()> {
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

        if !bool::from(account_extension.transferring) {
            panic!("TransferHook: Not transferring");
        }

        Ok(())
    }
}
```
The hook first verifies that it is being called during an actual transfer by inspecting the `transferring` flag in the source token account's TransferHook extension. It then checks whether the source token owner has `is_whitelisted = true` in their corresponding Whitelist account. If the owner is whitelisted, the transfer proceeds; otherwise, it fails. Non-existent Whitelist accounts will cause deserialization to fail, effectively preventing transfers by uninitialized users.
---
This whitelist transfer hook provides a robust access control mechanism for Token 2022 mints, ensuring that only pre-approved addresses can transfer tokens while maintaining the standard token interface that users and applications expect.
