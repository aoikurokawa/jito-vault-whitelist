use std::{fs::File, io::Write, path::Path};

use codama::{Codama, NodeTrait};

fn main() {
    let program_path = Path::new("./program");
    let core_path = Path::new("./core");
    let sdk_path = Path::new("./sdk");

    let codama = Codama::load_all(&[program_path, core_path, sdk_path]).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    let crate_root = std::env::current_dir().unwrap();
    let out_dir = crate_root.join("idl");
    let mut idl_path = out_dir.join("vault_whitelist");
    idl_path.set_extension("json");

    let mut idl_json_file = File::create(idl_path).unwrap();
    idl_json_file.write_all(idl.as_bytes()).unwrap();
}
