# rusty-whsp

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![Crates.io](https://img.shields.io/crates/v/rusty-whsp.svg)](https://crates.io/crates/rusty-whsp)
[![Documentation](https://docs.rs/rusty-whsp/badge.svg)](https://docs.rs/rusty-whsp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A flexible and type-safe configuration parsing library for Rust command-line applications.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Documentation](#documentation)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🚀 Easy-to-use API for defining configuration options
- 🔢 Support for string, number, and boolean option types
- 📚 Single and multiple value options
- ✅ Automatic input validation
- 🌍 Default value setting from environment variables
- 🔤 Short and long command-line option support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusty-whsp = "0.1.6"
```

## Quick Start

Here's a simple example to get you started:

```rust
use rusty_whsp::{Whsp, WhspOptions, ConfigOptionBase, ValidValue, Validator};
use std::collections::HashMap;

fn main() {
    let mut whsp = Whsp {
        config_set: HashMap::new(),
        short_options: HashMap::new(),
        options: WhspOptions {
            allow_positionals: true,
            env_prefix: Some("MYAPP".to_string()),
            usage: None,
        },
    };

    whsp.opt(HashMap::from([(
        "config".to_string(),
        ConfigOptionBase {
            config_type: "string".to_string(),
            short: Some("c".to_string()),
            default: None,
            description: Some("Configuration file path".to_string()),
            validate: Some(Validator::None),
            multiple: false,
        },
    )]));

    let args: Vec<String> = std::env::args().collect();
    let parsed_values = whsp.parse_raw(args[1..].to_vec());

    println!("Parsed values: {:?}", parsed_values);
}
```

## Documentation

For detailed documentation, please refer to the [Documentation](https://rusty-libraries.github.io/rusty-whsp) file.

## Examples

Check out the [examples](examples/) directory for more usage examples.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

---

Made with ❤️ by rusty-libraries