use crate::tuya::model::date::{year_month::YearMonth, year_month_day::YearMonthDay};
use clap::{
    builder::{StringValueParser, TypedValueParser},
    Parser, Subcommand,
};

/// Utility for Tuya smart meter devices.
///
/// This utility wraps the Tuya API in a CLI application and optionally
/// exposes it as a limited API.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
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
    /// Commands for all devices, max 20 devices
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
    /// List all devices (max. 20)
    List,
    /// Retrieve cumulative energy comsumption in kWh for all devices (max. 20)
    #[command(subcommand)]
    Stats(Frequency),
}

#[derive(Subcommand, Debug)]
pub enum GetDeviceCommands {
    /// Retrieve cumulative energy comsumption in kWh
    #[command(subcommand)]
    Stats(Frequency),
    /// Retrieve all device info
    Info,
    /// Query all device properties
    Props,
}

#[derive(Subcommand, Debug)]
pub enum Frequency {
    /// Monthly frequency
    Monthly {
        /// Start date in the format 'yyyymm'
        #[arg(long, short, default_value_t = YearMonth::default(), value_parser = StringValueParser::new().try_map(YearMonth::try_from))]
        start: YearMonth,
        /// End date in the format 'yyyymm'
        #[arg(long, short, default_value_t = YearMonth::default(), value_parser = StringValueParser::new().try_map(YearMonth::try_from))]
        end: YearMonth,
    },
    /// Daily frequency
    Daily {
        /// Start date in the format 'yyyymmdd'
        #[arg(long, short, default_value_t = YearMonthDay::first_day_current_month(), value_parser = StringValueParser::new().try_map(YearMonthDay::try_from))]
        start: YearMonthDay,
        /// End date in the format 'yyyymmdd'
        #[arg(long, short, default_value_t = YearMonthDay::last_day_current_month(), value_parser = StringValueParser::new().try_map(YearMonthDay::try_from))]
        end: YearMonthDay,
    },
}
