# Pinocchio Escrow

A secure, efficient token escrow program built for Solana using the Pinocchio library. This program enables trustless atomic swaps between two different SPL tokens through an escrow mechanism.

## Overview

The escrow program facilitates peer-to-peer token exchanges where:
- A **maker** deposits tokens they want to trade and specifies what they want in return
- A **taker** can fulfill the trade by providing the requested tokens
- The **maker** can refund their deposit if no one takes the trade

## Features

- ✅ **Atomic Swaps**: Guaranteed exchange or refund - no partial states
- ✅ **Multi-token Support**: Works with any SPL token
- ✅ **Maker Protection**: Refund mechanism for unused escrows
- ✅ **Gas Efficient**: Built with Pinocchio for minimal compute units
- ✅ **Security First**: Comprehensive account validation and ownership checks

## Architecture

### Core Components

#### State Management (`src/state.rs`)
```rust
pub struct Escrow {
    pub seed: u64,        // Random seed for PDA derivation
    pub maker: Pubkey,    // Creator of the escrow
    pub mint_a: Pubkey,   // Token being deposited
    pub mint_b: Pubkey,   // Token being requested
    pub receive: u64,     // Amount of token B wanted
    pub bump: [u8;1]      // PDA bump seed
}
```

#### Instructions

1. **Make** (`src/instructions/make.rs`)
   - Creates escrow account with PDA derivation
   - Transfers maker's tokens to vault
   - Sets up trade parameters

2. **Take** (`src/instructions/take.rs`)
   - Validates escrow parameters
   - Transfers vault tokens to taker
   - Transfers taker's tokens to maker
   - Closes escrow and vault accounts

3. **Refund** (`src/instructions/refund.rs`)
   - Allows maker to reclaim deposited tokens
   - Returns tokens from vault to maker
   - Closes escrow and vault accounts

#### Helper Functions (`src/instructions/helpers.rs`)
- **SignerAccount**: Validates transaction signers
- **ProgramAccount**: Manages program-owned account lifecycle
- **MintInterface**: Validates SPL token mints
- **AssociatedTokenAccount**: Handles ATA creation and validation

## Program Flow

### Making an Escrow
```
1. Maker calls Make instruction with:
   - seed (random number for uniqueness)
   - mint_a (token they're offering)
   - mint_b (token they want)
   - amount (how much of token_a to deposit)
   - receive (how much of token_b they want)

2. Program creates escrow PDA account
3. Program creates ATA vault for token_a
4. Maker's token_a is transferred to vault
5. Escrow is live and discoverable
```

### Taking an Escrow
```
1. Taker calls Take instruction
2. Program validates escrow exists and is valid
3. Program transfers vault tokens to taker
4. Taker's token_b is transferred to maker
5. Vault and escrow accounts are closed
6. Trade is complete
```

### Refunding an Escrow
```
1. Maker calls Refund instruction
2. Program validates maker authority
3. Vault tokens are returned to maker
4. Vault and escrow accounts are closed
5. Escrow is cancelled
```

## Account Structure

### Make Instruction Accounts
```
0. [SIGNER, WRITABLE] Maker
1. [WRITABLE] Escrow (PDA)
2. [] Token Mint A
3. [] Token Mint B
4. [WRITABLE] Maker's ATA for Token A
5. [WRITABLE] Vault ATA (PDA)
6. [] System Program
7. [] Token Program
```

### Take Instruction Accounts
```
0. [SIGNER, WRITABLE] Taker
1. [] Maker
2. [WRITABLE] Escrow (PDA)
3. [] Token Mint A
4. [] Token Mint B
5. [WRITABLE] Vault ATA
6. [WRITABLE] Taker's ATA for Token A
7. [WRITABLE] Taker's ATA for Token B
8. [WRITABLE] Maker's ATA for Token B
9. [] System Program
10. [] Token Program
```

### Refund Instruction Accounts
```
0. [SIGNER, WRITABLE] Maker
1. [WRITABLE] Escrow (PDA)
2. [] Token Mint A
3. [WRITABLE] Vault ATA
4. [WRITABLE] Maker's ATA for Token A
5. [] Token Program
```

## Security Features

- **PDA Derivation**: Escrow accounts use Program Derived Addresses for security
- **Ownership Validation**: Strict account ownership checks
- **Signer Verification**: Required signatures for all state changes
- **Mint Validation**: Ensures tokens match expected mints
- **Arithmetic Safety**: Overflow protection for all calculations
- **Account Lifecycle**: Proper initialization and cleanup

## Building and Testing

### Prerequisites
- Rust 1.70+
- Solana CLI tools
- Pinocchio library (included in dependencies)

### Build
```bash
cargo build
cargo build --release  # For production
```

### Dependencies
```toml
[dependencies]
pinocchio = "0.9.2"
pinocchio-associated-token-account = "0.2.0"
pinocchio-system = "0.3.0"
pinocchio-token = "0.4.0"
```

## Program ID
```
DPqMV5ZzwyzwfyF4oWBwFT7jn8FqDXaDZSfYWP71hUmy
```

## Usage Example

### Creating an Escrow (Client-side)
```typescript
// Maker wants to trade 100 USDC for 1 SOL
const makeInstruction = await program.methods
  .make(
    new BN(Math.random() * 1000000), // seed
    new BN(1_000_000_000),          // 1 SOL (in lamports)
    new BN(100_000_000)             // 100 USDC (6 decimals)
  )
  .accounts({
    maker: maker.publicKey,
    escrow: escrowPDA,
    mintA: USDC_MINT,
    mintB: WSOL_MINT,
    makerAtaA: makerUsdcAta,
    vault: vaultPDA,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .instruction();
```

## Error Handling

The program includes comprehensive error handling through custom error types:

- `InvalidEscrowAccount`: Escrow account validation failed
- `InvalidTokenAccount`: Token account validation failed
- `InvalidMint`: Mint validation failed
- `InvalidAuthority`: Authority validation failed
- `InsufficientFunds`: Not enough tokens for operation
- `TokenMismatch`: Token types don't match expected
- `InvalidBump`: PDA bump validation failed
- `InvalidSeed`: Seed validation failed

## Contributing

1. Follow Rust best practices
2. Maintain comprehensive test coverage
3. Update documentation for any API changes
4. Ensure all builds pass without warnings

## License

This project is licensed under the MIT License.

## Audit Status

⚠️ **This code has not been audited. Use at your own risk in production environments.**

For production deployments, consider:
- Professional security audit
- Comprehensive testing on devnet/testnet
- Gradual rollout with monitoring
- Emergency pause mechanisms