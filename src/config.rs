use confy::ConfyError;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");

// ToDo Нужно реализовать сериализацию в компактном виде
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    uppercase: IndexSet<char>,
    lowercase: IndexSet<char>,
    digits: IndexSet<char>,
    symbols: IndexSet<char>,
    others: IndexSet<char>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            uppercase: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            lowercase: "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            digits: "0123456789".chars().collect(),
            symbols: "*&^%$#@!~".chars().collect(),
            others: "♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♩♪♫♬♭♮♯".chars().collect(),
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
    pub fn uppercase(&self) -> IndexSet<char> {
        self.uppercase.clone()
    }

    pub fn lowercase(&self) -> IndexSet<char> {
        self.lowercase.clone()
    }

    pub fn digits(&self) -> IndexSet<char> {
        self.digits.clone()
    }

    pub fn symbols(&self) -> IndexSet<char> {
        self.symbols.clone()
    }

    pub fn others(&self) -> IndexSet<char> {
        self.others.clone()
    }
    // ----------------------- End Getters ----------------------- //
}
