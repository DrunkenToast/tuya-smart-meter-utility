mod args;
mod command_handler;
mod tuya;
mod util;

use crate::args::{Args, MainCommands};
use clap::Parser;
use command_handler::handle_get_commands;
use std::process;
use tuya::client::TuyaClient;

#[tokio::main]
async fn main() -> () {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let mut client = TuyaClient::new(&args.host, &args.client_id, &args.client_secret);

    let res = match args.cmd {
        MainCommands::Get { cmd } => handle_get_commands(&cmd, &mut client).await,
        MainCommands::Serve { port: _ } => {
            todo!("API setup")
        }
    };

    if let Err(e) = res {
        eprintln!("{0}", e);
        process::exit(1);
    };
}
