use crate::{
    accounts::Whitelist,
    pretty_display::{account_header, field, section_header, PrettyDisplay},
};

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
