use std::env::current_dir;
use std::fs::create_dir_all;

// TODO: Rename all Messages so we can export them as Schemas
use cosmwasm_schema::remove_schemas;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();
}
