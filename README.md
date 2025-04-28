# Jito Vault Whitelist Program

A secure permissioned layer built on top of the Jito Vault program, allowing only whitelisted users to mint VRT tokens, withdraw supported tokens, and perform administrative operations.

## Overview

This program extends the functionality of Jito's Vault by implementing a whitelist mechanism that restricts access to critical vault operations.
It ensures that only authorized users can interact with sensitive functions of the vault, providing an additional layer of security and control.

## Instructions

- Initialize Config: Set up initial configuration for the whitelist program
- Initialize Whitelist: Create and configure the whitelist for user access control
- Set Mint Burn Admin: Assign administrative privileges for minting and burning tokens
- Set Merkle Root: Update the Merkle root for whitelist verification
- Mint: Mint new VRT tokens (whitelisted users only)
- Enqueue Withdrawal: Request token withdrawal (whitelisted users only)
- Burn Withdrawal Ticket: Process and complete a withdrawal request
- Close Whitelist: Terminate the whitelist functionality

## Features

- Whitelisted Access Control: Only pre-approved addresses can perform key operations
- Secured Token Operations: Protected mint VRT and withdraw functions
- Administrative Controls: Restricted access to admin operations

## Setup

### Generate IDL and Clients

```bash
make generate-code
```

### Check Lint

```bash
make lint
```

### Test

```bash
make test
```
