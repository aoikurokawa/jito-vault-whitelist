use std::fmt::{Debug, Formatter};

use solana_program::system_instruction::transfer;
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    clock::Clock, commitment_config::CommitmentLevel, native_token::sol_to_lamports,
    pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};

use crate::client::{
    restaking_clinet::{OperatorRoot, RestakingProgramClient},
    vault_client::{VaultProgramClient, VaultRoot},
    vault_whitelist_client::VaultWhitelistClient,
};

use super::TestResult;

pub struct ConfiguredVault {
    pub vault_program_client: VaultProgramClient,

    #[allow(dead_code)]
    pub restaking_program_client: RestakingProgramClient,

    pub vault_whitelist_client: VaultWhitelistClient,

    #[allow(dead_code)]
    pub vault_config_admin: Keypair,

    pub vault_root: VaultRoot,

    #[allow(dead_code)]
    pub restaking_config_admin: Keypair,

    pub operator_roots: Vec<OperatorRoot>,
}

pub struct TestBuilder {
    pub context: ProgramTestContext,
}

impl Debug for TestBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestBuilder",)
    }
}

impl TestBuilder {
    pub async fn new() -> Self {
        // $ cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo nextest run
        let mut program_test = ProgramTest::new(
            "jito_vault_whitelist_program",
            jito_vault_whitelist_program::id(),
            processor!(jito_vault_whitelist_program::process_instruction),
        );
        program_test.prefer_bpf(true);
        program_test.add_program("jito_vault_program", jito_vault_program::id(), None);
        program_test.add_program("jito_restaking_program", jito_restaking_program::id(), None);

        let context = program_test.start_with_context().await;
        Self { context }
    }

    pub fn restaking_program_client(&self) -> RestakingProgramClient {
        RestakingProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn vault_program_client(&self) -> VaultProgramClient {
        VaultProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn vault_whitelist_program_client(&self) -> VaultWhitelistClient {
        VaultWhitelistClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub async fn warp_slot_incremental(
        &mut self,
        incremental_slots: u64,
    ) -> Result<(), BanksClientError> {
        let clock: Clock = self.context.banks_client.get_sysvar().await?;
        self.context
            .warp_to_slot(clock.slot.checked_add(incremental_slots).unwrap())
            .map_err(|_| BanksClientError::ClientError("failed to warp slot"))?;
        Ok(())
    }

    pub async fn transfer(&mut self, to: &Pubkey, sol: f64) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(
                        &self.context.payer.pubkey(),
                        to,
                        sol_to_lamports(sol),
                    )],
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }

    /// Configures a vault with an NCN and operators fully configured
    pub async fn setup_vault_with_ncn_and_operators(
        &mut self,
        deposit_fee_bps: u16,
        withdrawal_fee_bps: u16,
        reward_fee_bps: u16,
        num_operators: u16,
        slasher_amounts: &[u64],
    ) -> TestResult<ConfiguredVault> {
        let mut vault_program_client = self.vault_program_client();
        let mut restaking_program_client = self.restaking_program_client();
        let vault_whitelist_client = self.vault_whitelist_program_client();

        let (vault_config_admin, vault_root) = vault_program_client
            .setup_config_and_vault(deposit_fee_bps, withdrawal_fee_bps, reward_fee_bps)
            .await?;
        let restaking_config_admin = restaking_program_client.do_initialize_config().await?;

        let ncn_root = restaking_program_client.do_initialize_ncn().await?;

        // vault <> ncn
        restaking_program_client
            .do_initialize_ncn_vault_ticket(&ncn_root, &vault_root.vault_pubkey)
            .await?;
        self.warp_slot_incremental(1).await.unwrap();
        restaking_program_client
            .do_warmup_ncn_vault_ticket(&ncn_root, &vault_root.vault_pubkey)
            .await?;
        vault_program_client
            .do_initialize_vault_ncn_ticket(&vault_root, &ncn_root.ncn_pubkey)
            .await?;
        self.warp_slot_incremental(1).await.unwrap();
        vault_program_client
            .do_warmup_vault_ncn_ticket(&vault_root, &ncn_root.ncn_pubkey)
            .await?;

        let mut operator_roots = Vec::with_capacity(num_operators as usize);
        for _ in 0..num_operators {
            let operator_root = restaking_program_client.do_initialize_operator().await?;

            // ncn <> operator
            restaking_program_client
                .do_initialize_ncn_operator_state(&ncn_root, &operator_root.operator_pubkey)
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_ncn_warmup_operator(&ncn_root, &operator_root.operator_pubkey)
                .await?;
            restaking_program_client
                .do_operator_warmup_ncn(&operator_root, &ncn_root.ncn_pubkey)
                .await?;

            // vault <> operator
            restaking_program_client
                .do_initialize_operator_vault_ticket(&operator_root, &vault_root.vault_pubkey)
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_warmup_operator_vault_ticket(&operator_root, &vault_root.vault_pubkey)
                .await?;
            vault_program_client
                .do_initialize_vault_operator_delegation(
                    &vault_root,
                    &operator_root.operator_pubkey,
                )
                .await?;

            operator_roots.push(operator_root);
        }

        let mut slashers_amounts: Vec<(Keypair, u64)> = Vec::with_capacity(slasher_amounts.len());
        for amount in slasher_amounts {
            let slasher = Keypair::new();
            self.transfer(&slasher.pubkey(), 10.0).await?;

            restaking_program_client
                .do_initialize_ncn_vault_slasher_ticket(
                    &ncn_root,
                    &vault_root.vault_pubkey,
                    &slasher.pubkey(),
                    *amount,
                )
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            restaking_program_client
                .do_warmup_ncn_vault_slasher_ticket(
                    &ncn_root,
                    &vault_root.vault_pubkey,
                    &slasher.pubkey(),
                )
                .await?;

            vault_program_client
                .do_initialize_vault_ncn_slasher_ticket(
                    &vault_root,
                    &ncn_root.ncn_pubkey,
                    &slasher.pubkey(),
                )
                .await?;
            self.warp_slot_incremental(1).await.unwrap();
            vault_program_client
                .do_warmup_vault_ncn_slasher_ticket(
                    &vault_root,
                    &ncn_root.ncn_pubkey,
                    &slasher.pubkey(),
                )
                .await?;

            slashers_amounts.push((slasher, *amount));
        }

        Ok(ConfiguredVault {
            vault_program_client,
            restaking_program_client,
            vault_whitelist_client,
            vault_root,
            vault_config_admin,
            restaking_config_admin,
            operator_roots,
        })
    }
}
