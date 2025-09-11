use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    instruction::{Seed, Signer},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{
    instructions::{Transfer, CloseAccount},
    state::TokenAccount,
    ID as TOKEN_PROGRAM_ID,
};
use pinocchio_associated_token_account::instructions::Create;

pub struct SignerAccount;

impl SignerAccount {
    pub fn check(account: &AccountInfo) -> ProgramResult {
        if !account.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

pub struct ProgramAccount;

impl ProgramAccount {
    pub fn check(account: &AccountInfo) -> ProgramResult {
        if account.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }

    pub fn init<T>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed],
        space: usize,
    ) -> ProgramResult {
        let rent = Rent::get()?.minimum_balance(space);

        CreateAccount {
            from: payer,
            to: account,
            lamports: rent,
            space: space as u64,
            owner: &crate::ID,
        }.invoke_signed(&[Signer::from(seeds)])?;

        Ok(())
    }

    pub fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult {
        let dest_starting_lamports = destination.lamports();
        let account_lamports = account.lamports();

        // Transfer lamports from account to destination
        *destination.try_borrow_mut_lamports()? = dest_starting_lamports
            .checked_add(account_lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        *account.try_borrow_mut_lamports()? = 0;

        Ok(())
    }
}

pub struct MintInterface;

impl MintInterface {
    pub fn check(mint: &AccountInfo) -> ProgramResult {
        if mint.owner() != &TOKEN_PROGRAM_ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }
}

pub struct AssociatedTokenAccount;

impl AssociatedTokenAccount {
    pub fn check(
        ata: &AccountInfo,
        owner: &AccountInfo,
        mint: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        if ata.owner() != token_program.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // Simple check - if it's an ATA account, it should be owned by the token program
        // More sophisticated checks can be added later if needed

        Ok(())
    }

    pub fn init(
        ata: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        Create {
            funding_account: payer,
            account: ata,
            wallet: owner,
            mint,
            system_program,
            token_program,
        }.invoke()?;

        Ok(())
    }

    pub fn init_if_needed(
        ata: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        if ata.lamports() == 0 {
            Self::init(ata, mint, payer, owner, system_program, token_program)?;
        }
        Ok(())
    }
}