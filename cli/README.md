# Jito Vault Whitelist CLI

## Overview

The Jito Vault Whitelist CLI is a command-line tool that provides access to Jito Vault Whitelist Program.

With this CLI, you can:

- Initialize configuration of Jito Vault Whitelist Program.
- Whitelist configuration
- Mint, Withdraw operation

## Comamnd

### `initialize_config`

Jito's Vault Whitelist Program admin initialize configuration of this program.
The account contains the information like jito vault program address.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist config initialize
```

### `initialize_whitelist`

Vault Manger can initialize whitelist account through this command.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist initialize <VAULT_ADDRESS>
```

### `set_mint_burn_admin`

Vault Manager can set `vault_mint_burn_admin` filed to whitelist pubkey.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist set-mint-burn-admin <VAULT_ADDRESS>
```

### `add_to_whitelist`

Vault Manager can add new user to whitelist.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist add_to_whitelist <VAULT_ADDRESS> <USER_ADDRESS>
```

### `mint`

Whitelist user can mint VRT:

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist mint <USER_KEYPAIR_PATH>  <VAULT_ADDRESS> <AMOUNT_IN> <MIN_AMOUNT_OUT>
```

### `enqueue_withdrawal`

Whitelist user can initiate withdrawal:

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist enqueue-withdrawal <USER_KEYPAIR_PATH> <VAULT_ADDRESS> <AMOUNT>
```

### `burn_withdrawal_ticket`

Whitelist user can burn withdrawal ticket:

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist whitelist burn-withdrawal-ticket <USER_KEYPAIR_PATH> <VAULT_ADDRESS>
```
