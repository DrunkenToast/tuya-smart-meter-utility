mod args;
mod command_handler;
mod tuya;
mod util;

use crate::args::{Args, GetCommands, GetDevicesCommands, MainCommands};
use args::{Frequency, GetDeviceCommands};
use clap::Parser;
use command_handler::handle_get_commands;
use std::process;
use tuya::{client::TuyaClient, model::model::TuyaResult};

#[tokio::main]
async fn main() -> () {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let mut client = TuyaClient::new(&args.host, &args.client_id, &args.client_secret);

    let res = match args.cmd {
        MainCommands::Get { cmd } => handle_get_commands(&cmd, &mut client).await,
        MainCommands::Serve { port: _ } => {
            todo!("To be implemented")
        }
    };

    if let Err(e) = res {
        eprintln!("{0}", e);
        process::exit(1);
    };
}
