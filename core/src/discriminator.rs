use jito_bytemuck::Discriminator;

use crate::config::Config;

/// Discriminators for HelloWorldNcn accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelloWorldNcnDiscriminator {
    Config = 1,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = HelloWorldNcnDiscriminator::Config as u8;
}
