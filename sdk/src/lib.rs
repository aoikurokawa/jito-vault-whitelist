use codama::{codama, CodamaInstruction};

#[derive(CodamaInstruction)]
pub enum Instruction {
    #[codama(account(name="config_info", writable))]
    InitializeConfig,
}
