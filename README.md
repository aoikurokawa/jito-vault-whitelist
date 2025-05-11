# Jito Vault Whitelist Program

A secure permissioned layer built on top of the Jito Vault program, allowing only whitelisted users to mint VRT tokens, withdraw supported tokens, and perform administrative operations.


## Note
- Jito Vault Whitelist is in active development so all APIs and protocols are subject to change.
- The code hasn't been audited yet. Use at your own risk.

## Overview

This program extends the functionality of Jito's Vault by implementing a whitelist mechanism that restricts access to critical vault operations.
It ensures that only authorized users can interact with sensitive functions of the vault, providing an additional layer of security and control.

## Features

- Whitelisted Access Control: Only pre-approved addresses can perform key operations
- Secured Token Operations: Protected mint VRT and withdraw functions
- Administrative Controls: Restricted access to admin operations

## Program Instructions

The Jito Vault Whitelist Program supports the following instructions:

- Initialize Config: Set up initial configuration
- Initialize Whitelist: Create and configure the whitelist
- Set Mint Burn Admin: Assign administrative privileges
- Add To Whitelist: Add user to whitelist
- Remove from Whitelist: Remove user from whitelist
- Mint: Mint new VRT tokens (whitelisted users only)
- Enqueue Withdrawal: Request token withdrawal (whitelisted users only)
- Burn Withdrawal Ticket: Process withdrawal request
- Close Whitelist: Terminate whitelist functionality

## Program ID

| Network | Program              | Address                                       | Version |
|---------|----------------------|-----------------------------------------------|---------|
| Mainnet | Jito Vault Whitelist | 7BHULFc6NKwtc7f2ap6y7ty1cRfTN5MBMfJQj1rxEUhP  | 0.0.1   |
| Testnet | Jito Vault Whitelist | 7BHULFc6NKwtc7f2ap6y7ty1cRfTN5MBMfJQj1rxEUhP  | 0.0.1   |
| Devnet  | Jito Vault Whitelist | 7BHULFc6NKwtc7f2ap6y7ty1cRfTN5MBMfJQj1rxEUhP  | 0.0.1   |


## SDKs

We provide TypeScript SDKs for interacting with the Jito Vault Whitelist system:

- 📦 @jito-foundation/vault-whitelist-sdk – TypeScript SDK for interacting with the Jito Vault Whitelist program.

## Development Setup

### Prerequisites

- Rust and Cargo installed
- Solana CLI tools

### Getting Started

#### Generate IDL and Clients

```bash
make generate-code
```

#### Check Lint

```bash
make lint
```

#### Run Tests

```bash
make test
```

## CLI Tool

A command-line interface is available for interacting with the Jito Vault Whitelist Program.
For detailed usage instructions and commands, please refer to the [README.md](./cli/README.md)

### Build from source

```bash
cargo build -p jito-vault-whitelist-cli --release
```

### Usage

All commands follow this basic structure:

```bash
jito-vault-whitelist-cli vault-whitelist <SUBCOMMAND> [OPTIONS] [ARGS]
```


## References

- [Jito Vault Program](https://github.com/jito-foundation/restaking)

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

