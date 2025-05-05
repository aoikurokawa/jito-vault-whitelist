use std::{fs::File, io::BufReader, path::PathBuf};

use serde::de::DeserializeOwned;

pub mod error;
pub mod generated_merkle_tree;
pub mod merkle_tree;
pub mod pubkey_string_conversion;
pub mod utils;
pub mod vault_whitelist_meta;
pub mod vault_whitelist_meta_tree_node;
pub mod verify;

pub fn read_json_from_file<T>(path: &PathBuf) -> serde_json::Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}
