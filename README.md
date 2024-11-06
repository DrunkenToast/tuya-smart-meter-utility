# Tuya Smart meter Utility

## Installation

- Install [Rust](https://www.rust-lang.org/tools/install)
- To install as binary: `cargo install --path .`
    - You can now use `tuya_util`
- Or run it with cargo: `cargo run -- [args]`

## Usage

There are 3 arguments required to be set, the easiest way is via a `.env` file:
```
HOST=https://openapi.tuyaeu.com
CLIENT_ID=
CLIENT_SECRET=
```

Alternatively these can be set like so as well:
`tuya_util --host <HOST> --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET>`

```
tuya_util -h
Utility for Tuya smart meter devices

Usage: tuya_util [OPTIONS] --host <HOST> --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET> <COMMAND>

Commands:
  get    Retrieve device information
  serve  Serve as an API
  help   Print this message or the help of the given subcommand(s)

Options:
      --host <HOST>                    Host name for Tuya endpoints [env: HOST=https://openapi.tuyaeu.com]
      --client-id <CLIENT_ID>          Client ID from the Tuya project [env: CLIENT_ID=y3dugamkvu3dqd4fdtv5]
      --client-secret <CLIENT_SECRET>  Client Secret/Access Secret from the Tuya project [env: CLIENT_SECRET=e98886b0c2f14f3ebab0eef8ac95445a]
  -v, --verbose...                     Set verbosity
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

There are help menus for every subcommand and option with a description of what it does.

## Examples

### List all devices

`tuya_util get devices list`

This will list all the smart meters and their id's

### Get all the energy stats since 2024 October

`tuya_util get devices stats monthly -s 20241`

### Get device specific information

- Device details: `tuya_util get device --id bf4049bbe6fcfe3c91cp6p info`
- Device props (energy usage, voltage etc): `tuya_util get device --id bf4049bbe6fcfe3c91cp6p props`
- Device stats: `tuya_util get device --id <DEVICE_ID> stats daily -s 20241102 -e 20241106`

