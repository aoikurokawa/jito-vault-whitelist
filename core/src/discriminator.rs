use jito_bytemuck::Discriminator;

use crate::{config::Config, whitelist::Whitelist, whitelist_user::WhitelistUser};

/// Discriminators for Vault Whitelist accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultWhitelistDiscriminator {
    Config = 0,
    Whitelist = 1,
    WhitelistUser = 2,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = VaultWhitelistDiscriminator::Config as u8;
}

impl Discriminator for Whitelist {
    const DISCRIMINATOR: u8 = VaultWhitelistDiscriminator::Whitelist as u8;
}

impl Discriminator for WhitelistUser {
    const DISCRIMINATOR: u8 = VaultWhitelistDiscriminator::WhitelistUser as u8;
}
