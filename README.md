# ArgonOne-Native-Fan-Controller
Written in Rust fan control daemon for Argon One v2 case for Raspberry Pi 4B. Fully native - unlike the official vendor controller which is just a python script. 
Easy to integrate with your operating system image building tools.

# How to use
In the [releases](https://github.com/JhnW/ArgonOne-Native-Fan-Controller/releases) tab there are already built files for Linux along with some supporting files. 
Archive contains a variant that uses the configuration file (please check argon_fan_controller_cfg.yml). To build another feature variant, check how to build.

The repository and build artifacts contain several additional files:
- argon_fan_controller.service needed by systemd
- argon_fan_controller_cfg.yml example configuration. Interval specifies the time between successive wakeup 
of the fan control process. The higher it is, the lower the CPU usage. Fan is array of pairs temperature
plus fan speed in %. Must be in increased order.
- deploy.sh is simple script who copy (if you copy all files to device) files to destinations folders, enable
i2c and setup systemd service.

# How to build
You need install on your operating system gcc aarch64 toolchain. 
Rust need gcc linker for that architecture. Next just write for standard build:
>cargo build --target aarch64-unknown-linux-gnu --release

or for build without configuration file reading

>cargo build --target aarch64-unknown-linux-gnu --release --no-default-features
