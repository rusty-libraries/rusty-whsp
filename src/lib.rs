use std::collections::HashMap;
use std::env;
use std::fmt;

pub type ConfigType = &'static str;

#[derive(Debug, Clone)]
pub enum ValidValue {
    Number(i64),
    String(String),
    Boolean(bool),
}

pub struct Whsp<'a> {
    pub config_set: HashMap<&'a str, ConfigOptionBase<'a>>,
    pub short_options: HashMap<&'a str, &'a str>,
    pub options: WhspOptions,
}

#[derive(Debug)]
pub struct WhspOptions {
    pub allow_positionals: bool,
    pub env_prefix: Option<&'static str>,
    pub usage: Option<&'static str>,
}

#[derive(Debug)]
pub struct ConfigOptionBase<'a> {
    pub config_type: ConfigType,
    pub short: Option<&'a str>,
    pub default: Option<ValidValue>,
    pub description: Option<&'a str>,
    pub validate: Option<Validator>,
    pub multiple: bool,
}

#[derive(Debug)]
pub enum Validator {
    NumberRange(i64, i64),
    Regex(&'static str),
    None,
}

impl<'a> Whsp<'a> {
    pub fn num(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "number";
            self.config_set.insert(name, option);
        }
    }

    pub fn num_list(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "number";
            option.multiple = true;
            self.config_set.insert(name, option);
        }
    }

    pub fn opt(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "string";
            self.config_set.insert(name, option);
        }
    }

    pub fn opt_list(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "string";
            option.multiple = true;
            self.config_set.insert(name, option);
        }
    }

    pub fn flag(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "boolean";
            self.config_set.insert(name, option);
        }
    }

    pub fn flag_list(&mut self, fields: HashMap<&'a str, ConfigOptionBase<'a>>) {
        for (name, mut option) in fields {
            option.config_type = "boolean";
            option.multiple = true;
            self.config_set.insert(name, option);
        }
    }

    pub fn validate_name(&mut self, name: &'a str, option: &ConfigOptionBase<'a>) -> Result<(), String> {
        if !name.chars().all(char::is_alphanumeric) {
            return Err(format!("Invalid option name: {}, must be alphanumeric.", name));
        }
        if let Some(short) = option.short {
            if self.short_options.contains_key(short) {
                return Err(format!("Short option {} is already in use.", short));
            }
            self.short_options.insert(short, name);
        }
        Ok(())
    }

    pub fn write_env(&self, parsed: &OptionsResult) {
        if let Some(prefix) = self.options.env_prefix {
            for (field, value) in &parsed.values {
                let env_key = to_env_key(prefix, field);
                let env_value = to_env_val(value);
                env::set_var(env_key, env_value);
            }
        }
    }

    pub fn parse_raw(&self, args: &[String]) -> OptionsResult {
        let mut values = HashMap::new();
        let mut positionals = Vec::new();
        let mut i = 0;
        
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                let key = &arg[2..];
                if let Some(config) = self.config_set.get(key) {
                    if config.config_type == "boolean" {
                        values.insert(key.to_string(), ValidValue::Boolean(true));
                    } else if i + 1 < args.len() {
                        let val = &args[i + 1];
                        values.insert(key.to_string(), match config.config_type {
                            "string" => ValidValue::String(val.to_string()),
                            "number" => ValidValue::Number(val.parse().unwrap()),
                            _ => panic!("Unknown config type"),
                        });
                        i += 1;
                    }
                }
            } else if arg.starts_with('-') {
                let short = &arg[1..];
                if let Some(&key) = self.short_options.get(short) {
                    if let Some(config) = self.config_set.get(key) {
                        if config.config_type == "boolean" {
                            values.insert(key.to_string(), ValidValue::Boolean(true));
                        } else if i + 1 < args.len() {
                            let val = &args[i + 1];
                            values.insert(key.to_string(), match config.config_type {
                                "string" => ValidValue::String(val.to_string()),
                                "number" => ValidValue::Number(val.parse().unwrap()),
                                _ => panic!("Unknown config type"),
                            });
                            i += 1;
                        }
                    }
                }
            } else {
                positionals.push(arg.clone());
            }
            i += 1;
        }

        OptionsResult {
            values,
            positionals,
        }
    }

    pub fn validate(&self, o: &HashMap<String, ValidValue>) -> Result<(), String> {
        for (field, value) in o {
            let config = self.config_set.get(field.as_str()).ok_or(format!("Unknown config option: {}", field))?;
            validate_options(config, value)?;
        }
        Ok(())
    }

    pub fn set_defaults_from_env(&mut self) {
        if let Some(prefix) = self.options.env_prefix {
            for (key, option) in self.config_set.iter_mut() {
                let env_key = to_env_key(prefix, key);
                match env::var(&env_key) {
                    Ok(val) => {
                        let valid_val = from_env_val(&val, option.config_type);
                        option.default = Some(valid_val);
                    },
                    Err(_) => {}
                }
            }
        }
    }
}

impl fmt::Display for ValidValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidValue::Number(val) => write!(f, "{}", val),
            ValidValue::String(val) => write!(f, "{}", val),
            ValidValue::Boolean(val) => write!(f, "{}", val),
        }
    }
}

impl<'a> ConfigOptionBase<'a> {
    pub fn new(config_type: ConfigType, multiple: bool, short: Option<&'a str>, description: Option<&'a str>) -> Self {
        ConfigOptionBase {
            config_type,
            short,
            default: None,
            description,
            validate: None,
            multiple,
        }
    }

    pub fn validate_value(&self, value: &ValidValue) -> bool {
        if let Some(ref validate) = self.validate {
            match validate {
                Validator::Regex(regex) => matches!(value, ValidValue::String(s) if regex == s),
                Validator::NumberRange(min, max) => matches!(value, ValidValue::Number(num) if *num >= *min && *num <= *max),
                Validator::None => true,
            }
        } else {
            match (self.config_type, value) {
                ("string", ValidValue::String(_)) => true,
                ("number", ValidValue::Number(_)) => true,
                ("boolean", ValidValue::Boolean(_)) => true,
                _ => false,
            }
        }
    }
}

pub fn to_env_key(prefix: &str, key: &str) -> String {
    format!("{}_{}", prefix.to_uppercase(), key.to_uppercase())
}

pub fn from_env_val(env: &str, config_type: &str) -> ValidValue {
    match config_type {
        "string" => ValidValue::String(env.to_string()),
        "number" => ValidValue::Number(env.parse().unwrap()),
        "boolean" => ValidValue::Boolean(env == "1"),
        _ => panic!("Unknown config type"),
    }
}

pub fn to_env_val(value: &ValidValue) -> String {
    match value {
        ValidValue::String(v) => v.clone(),
        ValidValue::Number(v) => v.to_string(),
        ValidValue::Boolean(v) => if *v { "1".to_string() } else { "0".to_string() },
    }
}

pub fn validate_options(config: &ConfigOptionBase, value: &ValidValue) -> Result<(), String> {
    if !config.validate_value(value) {
        return Err(format!("Invalid value {:?} for option", value));
    }
    Ok(())
}

#[derive(Debug)]
pub struct OptionsResult {
    pub values: HashMap<String, ValidValue>,
    pub positionals: Vec<String>,
}