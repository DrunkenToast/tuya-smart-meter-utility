use crate::{
    args::{Frequency, GetCommands, GetDeviceCommands, GetDevicesCommands},
    tuya::{client::TuyaClient, model::model::TuyaResult},
    util::pretty_string::PrettyString,
};

pub async fn handle_get_commands(args: &GetCommands, client: &mut TuyaClient) -> TuyaResult<()> {
    match args {
        GetCommands::Devices { cmd } => handle_get_devices(client, &cmd).await,
        GetCommands::Device { id, cmd } => handle_get_device(client, &id, &cmd).await,
    }
}

async fn handle_get_devices(client: &mut TuyaClient, opt: &GetDevicesCommands) -> TuyaResult<()> {
    let devices = client.get_devices().await?;
    println!("Listing all devices ({}):\n", devices.len());

    match opt {
        GetDevicesCommands::List => {
            let max_num_len = (devices.len()).checked_ilog10().unwrap_or(0) + 1;

            for (i, d) in devices.iter().enumerate() {
                println!(
                    "[{0}] name: {1}\n{2: >width$} {3}",
                    i + 1,
                    d.get_name(),
                    "id:",
                    d.device_id,
                    width = (max_num_len + 3 + 3) as usize,
                );
            }
            println!();
        }
        GetDevicesCommands::Stats(f) => match f {
            Frequency::Daily { start, end } => {
                for (i, d) in devices.iter().enumerate() {
                    let stats = client
                        .get_daily_device_statistics(d.device_id.as_str(), &start, &end)
                        .await?;

                    d.print(Some(i), stats);
                }
            }
            Frequency::Monthly { start, end } => {
                for (i, d) in devices.iter().enumerate() {
                    let stats = client
                        .get_monthly_device_statistics(d.device_id.as_str(), &start, &end)
                        .await?;

                    d.print(Some(i), stats)
                }
            }
        },
    };

    Ok(())
}

async fn handle_get_device(
    client: &mut TuyaClient,
    id: &str,
    cmd: &GetDeviceCommands,
) -> TuyaResult<()> {
    match cmd {
        GetDeviceCommands::Info => {
            let info = client.get_device_info(id).await?;
            println!(
                "Device info: {}",
                serde_json::to_string_pretty(&info).unwrap()
            );
        }
        GetDeviceCommands::Props => {
            let props = client.get_device_properties(id).await?;
            println!(
                "Device info: {}",
                serde_json::to_string_pretty(&props).unwrap()
            );
        }
        GetDeviceCommands::Stats(f) => {
            let stats = match f {
                Frequency::Daily { start, end } => client
                    .get_daily_device_statistics(id, &start, &end)
                    .await?
                    .as_pretty_string(),
                Frequency::Monthly { start, end } => client
                    .get_monthly_device_statistics(id, &start, &end)
                    .await?
                    .as_pretty_string(),
            };
            println!("{}", stats)
        }
    };
    Ok(())
}
