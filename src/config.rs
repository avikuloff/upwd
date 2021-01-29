use confy::ConfyError;
use serde::{Deserialize, Serialize};

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    default_set: String,
    uppercase: String,
    lowercase: String,
    digits: String,
    symbols: String,
    others: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_set: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                .to_owned(),
            uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned(),
            lowercase: "abcdefghijklmnopqrstuvwxyz".to_owned(),
            digits: "0123456789".to_owned(),
            symbols: "*&^%$#@!~".to_owned(),
            others: "♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♩♪♫♬♭♮♯".to_owned(),
        }
    }
}

impl Config {
    /// Creates a new instance with values from config file `/path/to/config/dir/crate_name/crate_name.toml`.
    /// If file is missing or contains invalid values, then `Config::default()` is used.
    pub fn new() -> Config {
        confy::load(CRATE_NAME).unwrap_or_default()
    }

    /// Overwrites the config file with default values.
    pub fn save_default() -> Result<(), ConfyError> {
        confy::store(CRATE_NAME, Config::default())
    }

    // ----------------------- Getters ----------------------- //
    pub fn default_set(&self) -> &str {
        &self.default_set
    }

    pub fn uppercase(&self) -> &str {
        &self.uppercase
    }

    pub fn lowercase(&self) -> &str {
        &self.lowercase
    }

    pub fn digits(&self) -> &str {
        &self.digits
    }

    pub fn symbols(&self) -> &str {
        &self.symbols
    }

    pub fn others(&self) -> &str {
        &self.others
    }
    // ----------------------- End Getters ----------------------- //
}
