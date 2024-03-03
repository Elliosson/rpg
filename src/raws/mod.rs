mod rawmaster;
pub use rawmaster::*;
use serde::Deserialize;
use std::{fs, sync::Mutex};

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

#[derive(Deserialize, Debug)]
pub struct Raws {
    pub props: Vec<Template>,
}

pub fn load_raws() {
    let raw_string =
        fs::read_to_string("./raws/spawns.json").expect("Should have been able to read the file");

    let decoder: Raws = serde_json::from_str(&raw_string).expect("Unable to parse JSON");

    RAWS.lock().unwrap().load(decoder);
}
