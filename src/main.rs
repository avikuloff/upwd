#[macro_use]
extern crate clap;

use clap::derive::Clap;
use clap::{AppSettings, ArgGroup};
use indexmap::IndexSet;
use std::io::{stdout, Write};
use upwd::{calculate_entropy, calculate_length, generate_password};
use upwd::config::Config;

fn main() {
    let opts: Opts = Opts::parse();

    if opts.reset {
        Config::save_default().unwrap();
    }

    let pool = opts.collect_charset();

    // Если в команде передана опция --entropy, то вычисляем необходимую длину пароля,
    // иначе присваивается значение по умолчанию.
    let length = opts.entropy.map_or(opts.length, |e| {
        calculate_length(e, pool.len() as f64).ceil() as usize
    });

    for _ in 0..opts.count {
        let password = generate_password(&pool, length);

        println!("{}", password);
    }

    if opts.info {
        let entropy = calculate_entropy(length, pool.len());
        Info::new(entropy, length, pool.len()).write(stdout());
    }
}

#[derive(Clap, Debug)]
#[clap(author, about, version,
group = ArgGroup::new("charset").required(true).multiple(true),
setting = AppSettings::DeriveDisplayOrder
)]
struct Opts {
    /// Use UPPERCASE letters [A-Z]
    #[clap(short, long, group = "charset")]
    uppercase: bool,

    /// Uses lowercase [a-z]
    #[clap(short, long, group = "charset")]
    lowercase: bool,

    /// Use digits [0-9]
    #[clap(short, long, group = "charset")]
    digits: bool,

    /// Use special symbols [*&^%$#@!~]
    #[clap(short, long, group = "charset")]
    symbols: bool,

    /// Use other symbols (see config file).
    #[clap(short, long, group = "charset")]
    others: bool,

    /// Sets the required password length
    #[clap(short = "L", long, value_name = "NUMBER", default_value = "12")]
    length: usize,

    /// Sets the minimum required password entropy (conflicts with --length)
    #[clap(short = "E", long, value_name = "NUMBER", conflicts_with = "length")]
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
    fn collect_charset(&self) -> IndexSet<char> {
        let mut pool = IndexSet::new();

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

        assert!(!pool.is_empty(), "Pool contains no elements!");

        pool
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
    fn new(entropy: f64, length: usize, pool_size: usize) -> Self {
        Info {
            entropy,
            length,
            pool_size,
        }
    }

    // Prints info
    // FixMe Как обработать ошибки?
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

        assert!(pool.contains(&'A'));
        assert!(pool.contains(&'a'));
        assert!(pool.contains(&'0'));
        assert!(pool.contains(&'&'));
        assert!(pool.contains(&'♖'));
    }

    #[test]
    #[should_panic(expected = "Pool contains no elements!")]
    fn charset_collect_all_fields_false() {
        Opts {
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
        }
        .collect_charset();
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
        Info::new(64.0, 15, 64).write(&mut actual);

        let expected = b"Entropy: 64 bits | Length: 15 chars | Pool size: 64 chars\n".to_vec();

        assert_eq!(actual, expected);
    }
}
