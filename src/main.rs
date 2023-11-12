mod args;
mod tuya;
mod util;

use crate::args::{Args, GetCommands, GetDevicesCommands, MainCommands};
use args::GetDeviceCommands;
use clap::Parser;
use std::process;
use tuya::{client::TuyaClient, model::model::TuyaResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let mut client = TuyaClient::new(&args.host, &args.client_id, &args.client_secret);

    match args.cmd {
        MainCommands::Get { cmd } => match cmd {
            GetCommands::Devices { cmd } => handle_get_devices(&mut client, &cmd).await?,
            GetCommands::Device { id, cmd } => handle_get_device(&mut client, &id, &cmd).await?,
        },
        MainCommands::Serve { port: _ } => {
            todo!("To be implemented")
        }
    }

    println!("Done.");

    Ok(())
}

async fn handle_get_devices(client: &mut TuyaClient, opt: &GetDevicesCommands) -> TuyaResult<()> {
    if let Ok(devices) = client.get_devices().await {
        match opt {
            GetDevicesCommands::List => {
                println!("Listing all devices ({}):\n", devices.len());
                for (i, d) in devices.iter().enumerate() {
                    let name: &String = if d.custom_name.len() == 0 {
                        &d.product_name
                    } else {
                        &d.custom_name
                    };
                    println!("[{}] name: {}\n    id: {}", i + 1, name, d.device_id);
                }
                println!();
            }
            GetDevicesCommands::Stats => {
                for d in devices {
                    let stats = client.get_device_statistics(d.device_id.as_str()).await?;
                    let mut str: Vec<String> = vec![];
                    stats
                        .months
                        .iter()
                        .for_each(|m| str.push(format!("{0}: {1}", m.0, m.1)));
                    println!("{0}: {1}", d.custom_name, str.join(", "));
                }
            }
        };
    } else {
        eprintln!("Error retrieving devices");
        process::exit(1);
    }
    Ok(())
}

async fn handle_get_device(
    client: &mut TuyaClient,
    id: &str,
    cmd: &GetDeviceCommands,
) -> TuyaResult<()> {
    let info = client.get_device_info(id).await?;
    match cmd {
        GetDeviceCommands::Info => {
            println!(
                "Device info: {}",
                serde_json::to_string_pretty(&info).unwrap()
            );
        }
        GetDeviceCommands::Stats => {
            let stats = client.get_device_statistics(id).await?;
            let mut str: Vec<String> = vec![];
            stats
                .months
                .iter()
                .for_each(|m| str.push(format!("{0}: {1}", m.0, m.1)));
            println!("{0}", str.join(", "));
        }
    };
    Ok(())
}
