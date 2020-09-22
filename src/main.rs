use clap::Clap;
use clap::{AppSettings};
use indexmap::IndexSet;
use std::io::{stdout, Write};
use upwd::config::Config;
use upwd::{calculate_entropy, calculate_length, generate_password};

fn main() {
    let opts: Opts = Opts::parse();

    if opts.reset {
        Config::save_default().unwrap();
    }

    let pool = opts.collect_charset();

    let length = opts.entropy.map_or(opts.length, |e| {
        calculate_length(e, pool.len() as f64).ceil() as usize
    });

    for _ in 0..opts.count {
        let password = generate_password(&pool, length);

        println!("{}", password);
    }

    if opts.info {
        Info::new(length, pool.len()).write(stdout());
    }
}

#[derive(Clap, Debug)]
#[clap(author, about, version,
setting = AppSettings::DeriveDisplayOrder
)]
struct Opts {
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

    /// Use other symbols (see config file).
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

impl Opts {
    // Will panic if all fields are false
    pub fn collect_charset(&self) -> IndexSet<char> {
        let mut pool = IndexSet::new();

        if self.charset_are_false() {
            pool = self.collect_default_charset();
        } else {
            if self.uppercase {
                pool.extend(&self.config.uppercase());
            }
            if self.lowercase {
                pool.extend(&self.config.lowercase());
            }
            if self.digits {
                pool.extend(&self.config.digits());
            }
            if self.symbols {
                pool.extend(&self.config.symbols());
            }
            if self.others {
                pool.extend(&self.config.others());
            }
        }

        assert!(!pool.is_empty(), "Pool contains no elements!");

        pool
    }

    fn collect_default_charset(&self) -> IndexSet<char> {
        let mut pool = IndexSet::new();

        pool.extend(&self.config.uppercase());
        pool.extend(&self.config.lowercase());
        pool.extend(&self.config.digits());

        pool
    }

    // Returns true if all flags from charset group are missing
    fn charset_are_false(&self) -> bool {
        self.uppercase == false
            && self.lowercase == false
            && self.digits == false
            && self.symbols == false
            && self.others == false
    }
}

#[derive(Debug, Clone)]
struct Info {
    entropy: f64,
    length: usize,
    pool_size: usize,
}

impl Info {
    // Creates new instance
    fn new(length: usize, pool_size: usize) -> Self {
        let entropy = calculate_entropy(length, pool_size);

        Info {
            entropy,
            length,
            pool_size,
        }
    }

    // Prints info
    fn write(&self, mut writer: impl Write) {
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
    fn charset_collect_charset() {
        let opts = Opts {
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
        let pool = opts.collect_charset();

        assert!(pool.is_superset(&opts.config.uppercase()));
        assert!(pool.is_superset(&opts.config.lowercase()));
        assert!(pool.is_superset(&opts.config.digits()));
        assert!(pool.is_superset(&opts.config.symbols()));
        assert!(pool.is_superset(&opts.config.others()));
    }

    #[test]
    fn charset_collect_all_fields_false() {
        let opts = Opts {
            uppercase: false,
            lowercase: false,
            digits: false,
            symbols: false,
            others: false,
            length: 0,
            entropy: None,
            count: 1,
            info: false,
            reset: false,
            config: Config::default(),
        };
        let pool = opts.collect_charset();

        assert!(pool.is_superset(&opts.config.uppercase()));
        assert!(pool.is_superset(&opts.config.lowercase()));
        assert!(pool.is_superset(&opts.config.digits()));

        assert!(!pool.is_superset(&opts.config.symbols()));
    }

    #[test]
    fn generate_password_intersection() {
        let cfg = Config::default();

        let pool = Opts {
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
        }
        .collect_charset();

        let password: IndexSet<char> = generate_password(&pool, 1000).chars().collect();

        assert!(!password.is_disjoint(&cfg.uppercase()));
        assert!(!password.is_disjoint(&cfg.lowercase()));
        assert!(!password.is_disjoint(&cfg.digits()));
        assert!(!password.is_disjoint(&cfg.symbols()));
        assert!(!password.is_disjoint(&cfg.others()));
    }

    #[test]
    fn info_write() {
        let mut actual: Vec<u8> = vec![];
        Info::new(15, 64).write(&mut actual);

        let expected = b"Entropy: 90 bits | Length: 15 chars | Pool size: 64 chars\n".to_vec();

        assert_eq!(actual, expected);
    }
}
