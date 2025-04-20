use jito_restaking_client_common::log::{account_header, field, section_header, PrettyDisplay};

use crate::accounts::Config;

impl PrettyDisplay for Config {
    fn pretty_display(&self) -> String {
        let mut output = String::new();

        output.push_str(&account_header("Config Account"));

        output.push_str(&section_header("Admin Authorities"));
        output.push_str(&field("Admin", self.admin));

        output
    }
}
