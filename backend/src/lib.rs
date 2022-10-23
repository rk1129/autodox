mod apis;
mod structure;
mod data_structure;
mod backend_error;

use structure::*;
use backend_error::*;

use ic_kit::{
    macros::*,
    candid::export_service,
};

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("backend.did"), export_candid()).expect("Write failed.");
    }
}