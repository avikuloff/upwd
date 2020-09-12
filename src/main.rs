#[macro_use]
extern crate clap;

use std::io::{stdout, Write};

use clap::{AppSettings, ArgGroup};
use clap::derive::Clap;
use indexmap::IndexSet;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

use config::Config;

mod config;

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

    let password = generate_password(pool.clone(), length);

    println!("{}", password);

    if opts.info {
        let entropy = calculate_entropy(length as u32, pool.len());
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

// Generate random password.
// Panics if `pool` is empty.
fn generate_password(pool: IndexSet<char>, length: usize) -> String {
    assert!(!pool.is_empty(), "Pool contains no elements!");

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pool.len());
            pool[idx]
        })
        .collect()
}

// Calculates entropy.
// ToDo Нужно ли возвращать бесконечность?
fn calculate_entropy(length: u32, pool_size: usize) -> f64 {
    BigUint::from(pool_size)
        .pow(length)
        .to_f64()
        .expect("Typecast error! Failed to convert BigUint to f64!")
        .log2()
}

// Calculates the required password length to obtain the given entropy.
fn calculate_length(entropy: f64, pool_size: f64) -> f64 {
    entropy / pool_size.log2()
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
        ).unwrap();
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
            info: false,
            reset: false,
            config: Config::default(),
        }
            .collect_charset();
    }

    #[test]
    fn generate_password_assert_len() {
        let pool = "0123456789".chars().collect::<IndexSet<char>>();
        let password = generate_password(pool, 15);

        assert_eq!(password.len(), 15);
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
            info: false,
            reset: false,
            config: Config::default(),
        }
            .collect_charset();

        let password: IndexSet<char> = generate_password(pool, 1000).chars().collect();

        assert!(!password.is_disjoint(&cfg.uppercase()));
        assert!(!password.is_disjoint(&cfg.lowercase()));
        assert!(!password.is_disjoint(&cfg.digits()));
        assert!(!password.is_disjoint(&cfg.symbols()));
        assert!(!password.is_disjoint(&cfg.others()));
    }

    #[test]
    #[should_panic(expected = "Pool contains no elements!")]
    fn generate_password_passed_empty_pool() {
        let pool = "".chars().collect::<IndexSet<char>>();

        generate_password(pool, 15);
    }

    #[test]
    fn calculate_entropy_assert_true() {
        let entropy = calculate_entropy(12, 64);

        assert_eq!(entropy, 72.0);
    }

    #[test]
    fn calculate_entropy_passed_length_is_0() {
        let entropy = calculate_entropy(0, 64);

        assert_eq!(entropy, 0.0)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_0() {
        let entropy = calculate_entropy(12, 0);

        assert_eq!(entropy, f64::NEG_INFINITY)
    }

    #[test]
    fn calculate_entropy_passed_pool_size_is_1() {
        let entropy = calculate_entropy(12, 1);

        assert_eq!(entropy, 0.0)
    }

    #[test]
    fn calculate_length_assert_true() {
        let length = calculate_length(128.0, 64.0);

        assert_eq!(length.ceil(), 22.0);
    }

    #[test]
    fn calculate_length_entropy_is_0() {
        let length = calculate_length(0.0, 64.0);

        assert_eq!(length, 0.0);
    }

    #[test]
    fn calculate_length_pool_size_is_0() {
        let length = calculate_length(128.0, 0.0);

        assert_eq!(length, 0.0);
    }

    #[test]
    fn info_write() {
        let mut actual: Vec<u8> = vec![];
        Info::new(64.0, 15, 64).write(&mut actual);

        let expected = b"Entropy: 64 bits | Length: 15 chars | Pool size: 64 chars\n".to_vec();

        assert_eq!(actual, expected);
    }
}
