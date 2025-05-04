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
cargo r -p jito-vault-whitelist-cli -- vault-whitelist vault-whitelist initialize <WHITELIST_DATA_PATH> <VAULT_ADDRESS>
```

### `set_mint_burn_admin`

Vault Manager can set `vault_mint_burn_admin` filed to whitelist pubkey.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist vault-whitelist set-mint-burn-admin <VAULT_ADDRESS>
```

### `set_merkle_root`

Vault Manager can set new merkle root if whitelist user added or removed.

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist vault-whitelist set-merkle-root <WHITELIST_DATA_PATH> <VAULT_ADDRESS>
```

### `mint`

Whitelist user can mint VRT:

```bash
cargo r -p jito-vault-whitelist-cli -- vault-whitelist vault-whitelist mint <WHITELIST_DATA_PATH> <USER_KEYPAIR_PATH>  <VAULT_ADDRESS> <AMOUNT_IN> <MIN_AMOUNT_OUT>
```

### `enqueue_withdrawal`

Whitelist user can initiate withdrawal:

```bash

```
