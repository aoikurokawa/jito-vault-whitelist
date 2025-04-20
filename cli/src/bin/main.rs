use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use clap_markdown::MarkdownOptions;
use env_logger::Env;
use jito_vault_client::programs::JITO_VAULT_ID;
use jito_vault_whitelist_cli::{
    cli_args::{Cli, ProgramCommand},
    cli_config::CliConfig,
    cli_signer::CliSigner,
    vault_whitelist_handler::VaultWhitelistCliHandler,
};
use jito_vault_whitelist_client::programs::JITO_VAULT_WHITELIST_ID;
use solana_cli_config::Config;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

pub fn get_cli_config(args: &Cli) -> Result<CliConfig, anyhow::Error> {
    let cli_config = if let Some(config_file) = &args.config_file {
        let config = Config::load(config_file.as_os_str().to_str().unwrap())?;
        let signer = if let Some(ledger) = &args.ledger {
            CliSigner::new_ledger(ledger)
        } else if let Some(keypair_path) = &args.keypair {
            CliSigner::new_keypair_from_path(keypair_path)
        } else {
            CliSigner::new_keypair_from_path(&config.keypair_path)
        };

        CliConfig {
            rpc_url: config.json_rpc_url,
            commitment: CommitmentConfig::from_str(&config.commitment)?,
            signer: Some(signer),
        }
    } else {
        let config_file = solana_cli_config::CONFIG_FILE
            .as_ref()
            .ok_or_else(|| anyhow!("unable to get config file path"))?;
        if let Ok(config) = Config::load(config_file) {
            let signer = if let Some(ledger) = &args.ledger {
                CliSigner::new_ledger(ledger)
            } else if let Some(keypair_path) = &args.keypair {
                CliSigner::new_keypair_from_path(keypair_path)
            } else {
                CliSigner::new_keypair_from_path(&config.keypair_path)
            };

            let rpc = if let Some(rpc) = &args.rpc_url {
                rpc.to_string()
            } else {
                config.json_rpc_url
            };

            CliConfig {
                rpc_url: rpc,
                commitment: CommitmentConfig::from_str(&config.commitment)?,
                signer: Some(signer),
            }
        } else {
            CliConfig {
                rpc_url: args
                    .rpc_url
                    .as_ref()
                    .ok_or_else(|| anyhow!("RPC URL not provided"))?
                    .to_string(),
                commitment: if let Some(commitment) = &args.commitment {
                    CommitmentConfig::from_str(commitment)?
                } else {
                    CommitmentConfig::confirmed()
                },
                signer: args.ledger.as_ref().map_or_else(
                    || {
                        args.keypair
                            .as_ref()
                            .map(|keypair| CliSigner::new_keypair_from_path(keypair))
                    },
                    |ledger| Some(CliSigner::new_ledger(ledger)),
                ),
            }
        }
    };

    Ok(cli_config)
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Cli = Cli::parse();

    if args.markdown_help {
        let markdown = clap_markdown::help_markdown_custom::<Cli>(
            &MarkdownOptions::new().show_table_of_contents(false),
        );
        println!("---");
        println!("title: CLI");
        println!("category: Jekyll");
        println!("layout: post");
        println!("weight: 1");
        println!("---");
        println!();
        println!("{}", markdown);
        return Ok(());
    }

    let cli_config = get_cli_config(&args)?;

    let vault_program_id = if let Some(vault_program_id) = &args.vault_program_id {
        Pubkey::from_str(vault_program_id)?
    } else {
        JITO_VAULT_ID
    };
    let vault_whitelist_program_id = if let Some(vault_program_id) = &args.vault_program_id {
        Pubkey::from_str(vault_program_id)?
    } else {
        JITO_VAULT_WHITELIST_ID
    };

    match args.command.expect("Command not found") {
        ProgramCommand::VaultWhitelist { action } => {
            VaultWhitelistCliHandler::new(
                cli_config,
                vault_whitelist_program_id,
                vault_program_id,
                args.print_tx,
            )
            .handle(action)?;
        }
    }

    Ok(())
}
