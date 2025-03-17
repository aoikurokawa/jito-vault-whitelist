use std::path::Path;

use codama::{Codama, NodeTrait};

fn main() {
    let program_path = Path::new("./program");
    let core_path = Path::new("./core");
    let sdk_path = Path::new("./sdk");

    let codama = Codama::load_all(&[program_path, core_path, sdk_path]).unwrap();
    let idl = codama.get_idl().unwrap().to_json_pretty().unwrap();

    println!("IDL: {}", idl)
}
