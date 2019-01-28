extern crate dirs;
extern crate ini;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

extern crate clap;

#[macro_use]
extern crate serde_derive;

mod system;
mod minv_config;
mod inventory_api;
mod return_matches;



fn main() {
    let home_path = dirs::home_dir().unwrap();
    let config = minv_config::get_config(home_path);
    // let _yaml = load_yaml!("cli.yml");
    // let matches = App::from_yaml(_yaml).get_matches();
    let matches = return_matches::return_matches();

    match matches.subcommand_matches("system") {
        Some(value) => { system::execute(value, config.clone()) },
        None => {}
    }
}