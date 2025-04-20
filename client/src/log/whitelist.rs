use jito_restaking_client_common::log::{account_header, field, section_header, PrettyDisplay};

use crate::accounts::Whitelist;

impl PrettyDisplay for Whitelist {
    fn pretty_display(&self) -> String {
        let mut output = String::new();

        output.push_str(&account_header("Whitelist Account"));

        output.push_str(&section_header("Basic Information"));
        output.push_str(&field("Vault", self.vault));
        output.push_str(&field("Bump", self.bump));

        output
    }
}
