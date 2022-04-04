pub(crate) mod basic_config;
pub(crate) mod error;
pub(crate) mod poem_config;

use super::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

pub use self::basic_config::BasicConfig;
pub use self::error::ConfigError;
pub use self::poem_config::PoemConfig;

use crate::environment::{Environment, Environment::*};

pub use toml::value::{Array, Datetime, Table, Value};

const CONFIG_FILENAME: &str = "conf/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;
