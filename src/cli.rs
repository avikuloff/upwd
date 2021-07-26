use crate::config::Config;
use clap::AppSettings;
use clap::Clap;
use std::io::Write;
use upwd_lib::{calculate_entropy, Pool};

#[derive(Clap, Debug)]
#[clap(author, about, version,
after_help = "If you do not specify any of the \
[--uppercase, --lowercase, --digits, --symbols, --others] flags, \
then uppercase, lowercase letters and digits will be used.",
setting = AppSettings::DeriveDisplayOrder)]
pub struct Cli {
    /// Use UPPERCASE letters [A-Z]
    #[clap(short, long)]
    uppercase: bool,

    /// Use lowercase letters [a-z]
    #[clap(short, long)]
    lowercase: bool,

    /// Use digits [0-9]
    #[clap(short, long)]
    digits: bool,

    /// Use special symbols [*&^%$#@!~]
    #[clap(short, long)]
    symbols: bool,

    /// Use other symbols [♕♖♗♘♙♚...].
    #[clap(short, long)]
    others: bool,

    /// Sets the required password length
    #[clap(short = 'L', long, value_name = "NUMBER", default_value = "12")]
    length: usize,

    /// Sets the minimum required password entropy (conflicts with --length)
    #[clap(short = 'E', long, value_name = "NUMBER", conflicts_with = "length")]
    entropy: Option<f64>,

    /// Number of passwords
    #[clap(short, long, value_name = "NUMBER", default_value = "1")]
    count: usize,

    /// Prints password information
    #[clap(short, long)]
    info: bool,

    /// Sets config to default values
    #[clap(long = "config")]
    reset: bool,

    #[clap(skip = Config::new())]
    config: Config,
}

impl Cli {
    pub fn collect(&self) -> Pool {
        let mut pool = Pool::new();

        if self.uppercase {
            pool.extend_from_string(self.config.uppercase());
        }
        if self.lowercase {
            pool.extend_from_string(self.config.lowercase());
        }
        if self.digits {
            pool.extend_from_string(self.config.digits());
        }
        if self.symbols {
            pool.extend_from_string(self.config.symbols());
        }
        if self.others {
            pool.extend_from_string(self.config.others());
        }

        if pool.is_empty() {
            pool.extend_from_string(self.config.default_set());
        }

        pool
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn reset(&self) -> bool {
        self.reset
    }

    pub fn entropy(&self) -> Option<f64> {
        self.entropy
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn info(&self) -> bool {
        self.info
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    entropy: f64,
    length: usize,
    pool_size: usize,
}

impl Info {
    // Creates new instance
    pub fn new(length: usize, pool_size: usize) -> Self {
        let entropy = calculate_entropy(length, pool_size);

        Info {
            entropy,
            length,
            pool_size,
        }
    }

    // Prints info
    pub fn write(&self, mut writer: impl Write) {
        writeln!(
            writer,
            "Entropy: {:.0} bits | Length: {} chars | Pool size: {} chars",
            self.entropy, self.length, self.pool_size
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opts_collect() {
        let opts = Cli {
            uppercase: true,
            lowercase: true,
            digits: true,
            symbols: true,
            others: true,
            length: 0,
            entropy: None,
            count: 1,
            info: false,
            reset: false,
            config: Config::default(),
        };
        let pool = opts.collect();

        assert!(pool.contains_all(opts.config.uppercase()));
        assert!(pool.contains_all(opts.config.lowercase()));
        assert!(pool.contains_all(opts.config.digits()));
        assert!(pool.contains_all(opts.config.symbols()));
        assert!(pool.contains_all(opts.config.others()));
    }

    #[test]
    fn info_write() {
        let mut actual: Vec<u8> = vec![];
        Info::new(15, 64).write(&mut actual);

        let expected = b"Entropy: 90 bits | Length: 15 chars | Pool size: 64 chars\n".to_vec();

        assert_eq!(actual, expected);
    }
}
