use jito_bytemuck::Discriminator;

use crate::config::Config;

/// Discriminators for HelloWorldNcn accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultWhitelistDiscriminator {
    Config = 0,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = VaultWhitelistDiscriminator::Config as u8;
}
