use confy::ConfyError;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

// ToDo Нужно реализовать сериализацию в компактном виде
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn new() -> Config {
        confy::load(crate_name!()).unwrap_or_default()
    }

    // Перезаписывает файл конфигурации значениями по умолчанию
    pub fn save_default() -> Result<(), ConfyError> {
        confy::store(crate_name!(), Config::default())
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
