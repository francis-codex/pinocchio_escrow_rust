use pinocchio::{account_info::AccountInfo, program_error::ProgramError, instruction::{Seed, Signer}, pubkey::create_program_address, ProgramResult};
use pinocchio_token::{instructions::{Transfer, CloseAccount}, state::TokenAccount};
use crate::{state::Escrow, instructions::{SignerAccount, MintInterface, AssociatedTokenAccount, ProgramAccount}};

pub struct RefundAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for RefundAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, escrow, mint_a, vault, maker_ata_a, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        SignerAccount::check(maker)?;
        ProgramAccount::check(escrow)?;
        MintInterface::check(mint_a)?;
        AssociatedTokenAccount::check(vault, escrow, mint_a, token_program)?;
        AssociatedTokenAccount::check(maker_ata_a, maker, mint_a, token_program)?;

        // Return the accounts
        Ok(Self {
            maker,
            escrow,
            mint_a,
            vault,
            maker_ata_a,
            token_program,
        })
    }
}

pub struct Refund<'a> {
    pub accounts: RefundAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Refund<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = RefundAccounts::try_from(accounts)?;

        Ok(Self {
            accounts,
        })
    }
}

impl<'a> Refund<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(&mut self) -> ProgramResult {
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;

        // Verify the maker is the correct authority
        if &escrow.maker != self.accounts.maker.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // Check if the escrow account is valid
        let escrow_key = create_program_address(&[b"escrow", self.accounts.maker.key(), &escrow.seed.to_le_bytes(), &escrow.bump], &crate::ID)?;
        if &escrow_key != self.accounts.escrow.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = escrow.bump;
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];
        let signer = Signer::from(&escrow_seeds);

        let vault_data = self.accounts.vault.try_borrow_data()?;
        let vault_account = unsafe { TokenAccount::from_bytes_unchecked(&vault_data) };
        let amount = vault_account.amount();

        // Transfer tokens back to maker
        Transfer {
            from: self.accounts.vault,
            to: self.accounts.maker_ata_a,
            authority: self.accounts.escrow,
            amount,
        }.invoke_signed(&[signer.clone()])?;

        // Close the vault
        CloseAccount {
            account: self.accounts.vault,
            destination: self.accounts.maker,
            authority: self.accounts.escrow,
        }.invoke_signed(&[signer.clone()])?;

        // Close the escrow account
        drop(data);
        ProgramAccount::close(self.accounts.escrow, self.accounts.maker)?;

        Ok(())
    }
}