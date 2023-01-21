#![deny(clippy::all, clippy::cargo, clippy::pedantic)]
// mlua does not implement userdata for &str
// so in some cases we have to use String
#![allow(clippy::needless_pass_by_value)]

use clap::Parser;
use mlua::Result;

mod cli;
mod lune;
mod utils;

use cli::Cli;
use utils::{pretty_print_luau_error, print_label};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.run().await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!();
            print_label("ERROR").unwrap();
            eprintln!();
            pretty_print_luau_error(&e);
            std::process::exit(1);
        }
    }
}

#[tokio::test]
async fn hello_lune() {
    let args = vec!["Hello, test! ✅".to_owned()];
    let cli = Cli::from_path_with_args("hello_lune", args);
    let result = cli.run().await;
    assert!(result.is_ok());
}
