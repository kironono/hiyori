# `hiyori`

[![version](https://img.shields.io/crates/v/hiyori)](https://crates.io/crates/hiyori)
[![license](https://img.shields.io/crates/l/hiyori)](https://crates.io/crates/hiyori)

CLI application to check the current weather.

```
❯ hiyori -l tokyo
Location      : Tokyo, JP (2021-12-21 22:58:46 +09:00)
Weather       : Clear, clear sky
Temperature   : 9.71 C
Pressure      : 1010 hPa
Humidity      : 67 %
Wind Speed    : 2.06 m/s
Wind Direction: 300 deg
```

## Installation

cargo install:

```
cargo install hiyori
```

## USAGE

```
USAGE:
    hiyori [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --location <location>    Weather location [default: tokyo]
```

## License

`hiyori` is distributed under the terms of the MIT license.

See the [LICENSE](LICENSE) files in this repository for more information.
