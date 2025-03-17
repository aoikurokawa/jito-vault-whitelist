use codama::CodamaAccount;
use solana_pubkey::Pubkey;

#[derive(CodamaAccount)]
pub struct Config {
    pub vault: Pubkey
}