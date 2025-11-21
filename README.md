# system-uptime

[![Crates.io](https://img.shields.io/crates/v/system-uptime.svg)](https://crates.io/crates/system-uptime)
[![Docs.rs](https://docs.rs/system-uptime/badge.svg)](https://docs.rs/system-uptime)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<center><img src="https://github.com/greyshaman/system-uptime/blob/main/images/photo_system-uptime.jpg" width="50%" alt="system-uptime"></center>

A cross-platform Rust library for retrieving operating system uptime.

## Supported OS

- ✅ **Windows** (via `GetTickCount64`)
- ✅ **Linux** and **Android** (via `/proc/uptime`)
- ✅ **macOS**, **iOS**, **FreeBSD** (via `sysctl` with `KERN_BOOTTIME`)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
system-uptime = "0.1.0"
```

or

```bash
$ cargo add system-uptime
```
## Usage

### Basic Example

```rust
use system_uptime::get_os_uptime;

fn main() {
    match get_os_uptime() {
        Ok(uptime_ms) => println!("System has been running for {} milliseconds", uptime_ms),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Getting Uptime as Duration

```rust
use system_uptime::get_os_uptime_duration;

fn main() {
    match get_os_uptime_duration() {
        Ok(duration) => {
            println!("Total uptime: {:?}", duration);
            println!("Seconds: {}", duration.as_secs());
            println!("Milliseconds: {}", duration.as_millis());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Formatting Uptime

```rust
use system_uptime::get_os_uptime_duration;
use std::time::Duration;

fn format_uptime(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

fn main() {
    if let Ok(duration) = get_os_uptime_duration() {
        println!("Uptime: {}", format_uptime(duration));
    }
}
```

## API

### `get_os_uptime() -> Result<u64, Box<dyn Error>>`

Returns system uptime in milliseconds.

Returns:

- `Ok(u64)` - number of milliseconds since system boot

- `Err(Box<dyn Error>)` - error retrieving uptime

### `get_os_uptime_duration() -> Result<Duration, Box<dyn Error>>`

Returns system uptime as `std::time::Duration`.

## Precision

- __Windows:__ precision ~10-16 milliseconds

- __Linux/Android:__ precision ~10 milliseconds (from /proc/uptime)

- __macOS/BSD:__ precision ~1 second (via sysctl)

## Example Output

```text
// On a system running for 2.5 hours:
System has been running for 9000000 milliseconds
Total uptime: 2.5h
Formatted: 2h 30m 0s
```

## Dependencies

- `libc` - for Unix-like systems

- `winapi` - for Windows (automatically included only on Windows)

## License

MIT License - see LICENSE file.

## Contributing

Issues and Pull Requests are welcome!
