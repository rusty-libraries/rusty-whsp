# rusty-whsp Documentation

## Overview

**rusty-whsp** is a Rust library designed for parsing and managing configuration options in command-line applications. It provides a flexible and type-safe way to define options, parse command-line arguments, validate input values, and handle defaults from environment variables.

## Key Features

- Support for string, number, and boolean option types
- Single and multiple value options
- Automatic input validation
- Default value setting from environment variables
- Short and long command-line option support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusty-whsp = "0.1.0"
```

## Usage Guide

### Initializing Whsp

Create a new `Whsp` instance:

```rust
use rusty_whsp::{Whsp, WhspOptions};
use std::collections::HashMap;

let mut whsp = Whsp {
    config_set: HashMap::new(),
    short_options: HashMap::new(),
    options: WhspOptions {
        allow_positionals: true,
        env_prefix: Some("MYAPP".to_string()),
        usage: None,
    },
};
```

### Defining Options

Use these methods to define different types of options:

- `opt()`: String options
- `num()`: Numeric options
- `flag()`: Boolean options
- `opt_list()`, `num_list()`, `flag_list()`: Multiple value versions

Example:

```rust
use rusty_whsp::{ConfigOptionBase, ValidValue, Validator};

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
```

### Parsing Arguments

Parse command-line arguments:

```rust
let args: Vec<String> = std::env::args().collect();
let parsed_values = whsp.parse_raw(args[1..].to_vec());
```

### Validation

Validate parsed values:

```rust
if let Err(e) = whsp.validate(&parsed_values.values) {
    eprintln!("Validation error: {}", e);
}
```

### Environment Variables

Set defaults from environment variables:

```rust
whsp.set_defaults_from_env();
```

## Advanced Features

### Multiple Values

Define options accepting multiple values:

```rust
whsp.opt_list(HashMap::from([(
    "include".to_string(),
    ConfigOptionBase {
        config_type: "string".to_string(),
        short: Some("I".to_string()),
        default: None,
        description: Some("Directories to include".to_string()),
        validate: Some(Validator::None),
        multiple: true,
    },
)]));
```

### Custom Validation

Use regex for strings or range checks for numbers:

```rust
whsp.opt(HashMap::from([(
    "pattern".to_string(),
    ConfigOptionBase {
        config_type: "string".to_string(),
        short: Some("p".to_string()),
        default: None,
        description: Some("Pattern to match".to_string()),
        validate: Some(Validator::Regex("^[a-z]+$".to_string())),
        multiple: false,
    },
)]));

whsp.num(HashMap::from([(
    "level".to_string(),
    ConfigOptionBase {
        config_type: "number".to_string(),
        short: Some("l".to_string()),
        default: Some(ValidValue::Number(3)),
        description: Some("Level value".to_string()),
        validate: Some(Validator::NumberRange(1, 5)),
        multiple: false,
    },
)]));
```

### Usage Information

Set custom usage information:

```rust
whsp.options.usage = Some("Usage: myapp [options]".to_string());
```

## API Reference

### Structs

- `Whsp`: Main struct for managing configuration options
- `WhspOptions`: Options for Whsp behavior
- `ConfigOptionBase`: Defines properties of a configuration option
- `OptionsResult`: Holds parsed values and positional arguments

### Enums

- `ValidValue`: Represents valid option values (Number, String, Boolean)
- `Validator`: Defines validation rules (NumberRange, Regex, None)

### Methods

- `Whsp::num()`, `Whsp::opt()`, `Whsp::flag()`: Define single-value options
- `Whsp::num_list()`, `Whsp::opt_list()`, `Whsp::flag_list()`: Define multi-value options
- `Whsp::validate_name()`: Validate option names
- `Whsp::write_env()`: Write parsed values to environment variables
- `Whsp::parse_raw()`: Parse command-line arguments
- `Whsp::validate()`: Validate parsed values
- `Whsp::set_defaults_from_env()`: Set defaults from environment variables

### Helper Functions

- `to_env_key()`: Convert option name to environment variable key
- `from_env_val()`: Convert environment variable value to ValidValue
- `to_env_val()`: Convert ValidValue to environment variable value
- `validate_options()`: Validate a single option value

## Error Handling

The library uses `Result<T, String>` for error handling, with descriptive error messages for various failure scenarios.

## Thread Safety

The current implementation is not explicitly designed for concurrent use. Consider using appropriate synchronization mechanisms if shared across threads.

## Performance Considerations

The library uses HashMaps for storing and accessing options, providing generally good performance for typical use cases. However, for applications with a very large number of options, consider the memory usage and lookup times.