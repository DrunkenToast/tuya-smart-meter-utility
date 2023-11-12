use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Host name for Tuya endpoints
    #[arg(long, env)]
    pub host: String,

    /// Client ID from the Tuya project
    #[arg(long, env)]
    pub client_id: String,

    /// Client Secret/Access Secret from the Tuya project
    #[arg(long, env)]
    pub client_secret: String,

    /// Set verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub cmd: MainCommands,
}

#[derive(Subcommand, Debug)]
pub enum MainCommands {
    /// Retrieve device information
    Get {
        #[command(subcommand)]
        cmd: GetCommands,
    },
    /// Serve as an API
    Serve {
        #[arg(long, short)]
        port: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum GetCommands {
    /// Commands for all devices
    Devices {
        #[command(subcommand)]
        cmd: GetDevicesCommands,
    },
    /// Commands for a device
    Device {
        #[arg(long, short)]
        id: String,

        #[command(subcommand)]
        cmd: GetDeviceCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum GetDevicesCommands {
    /// List all devices
    List,
    /// Retrieve stats for all devices
    Stats,
}

#[derive(Subcommand, Debug)]
pub enum GetDeviceCommands {
    /// Retrieve stats
    Stats,
    /// Retrieve stats
    Info,
}
