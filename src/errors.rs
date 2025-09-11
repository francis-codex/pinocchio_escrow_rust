use pinocchio::program_error::ProgramError;

#[derive(Debug, Clone, Copy)]
pub enum EscrowError {
    InvalidEscrowAccount,
    InvalidTokenAccount,
    InvalidMint,
    InvalidAuthority,
    InsufficientFunds,
    TokenMismatch,
    InvalidBump,
    InvalidSeed,
    EscrowNotReady,
    EscrowExpired,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        match e {
            EscrowError::InvalidEscrowAccount => ProgramError::InvalidAccountData,
            EscrowError::InvalidTokenAccount => ProgramError::InvalidAccountData,
            EscrowError::InvalidMint => ProgramError::InvalidAccountData,
            EscrowError::InvalidAuthority => ProgramError::InvalidAccountOwner,
            EscrowError::InsufficientFunds => ProgramError::InsufficientFunds,
            EscrowError::TokenMismatch => ProgramError::InvalidAccountData,
            EscrowError::InvalidBump => ProgramError::InvalidSeeds,
            EscrowError::InvalidSeed => ProgramError::InvalidSeeds,
            EscrowError::EscrowNotReady => ProgramError::InvalidInstructionData,
            EscrowError::EscrowExpired => ProgramError::InvalidInstructionData,
        }
    }
}