#[macro_use]
extern crate clap;

use std::io::{stdout, Write};

use clap::ArgGroup;
use clap::derive::Clap;
use indexmap::IndexSet;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

fn main() {
    let opts: Opts = Opts::parse();

    let pool = opts.charset.collect();
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
#[clap(author, about, version)]
struct Opts {
    #[clap(flatten)]
    charset: Charset,

    /// Sets the length (number of characters) of the password
    #[clap(short = "L", long, value_name = "NUMBER", default_value = "12")]
    length: usize,

    /// Sets the entropy of the password
    #[clap(short, long, value_name = "NUMBER", conflicts_with = "length")]
    entropy: Option<f64>,

    /// Prints password information
    #[clap(short, long)]
    info: bool,
}

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("charset").required(true).multiple(true))]
struct Charset {
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
}

impl Charset {
    pub const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    pub const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub const DIGITS: &'static str = "0123456789";
    pub const SYMBOLS: &'static str = "*&^%$#@!~";

    /// Will panic if all fields are false
    fn collect(&self) -> IndexSet<char> {
        let mut pool = IndexSet::new();

        if self.uppercase {
            pool.extend(Charset::UPPERCASE.chars());
        }
        if self.lowercase {
            pool.extend(Charset::LOWERCASE.chars());
        }
        if self.digits {
            pool.extend(Charset::DIGITS.chars());
        }
        if self.symbols {
            pool.extend(Charset::SYMBOLS.chars());
        }

        assert!(!pool.is_empty());

        pool
    }
}

/// Generate random password.
/// Panics if `pool` is empty.
fn generate_password(pool: IndexSet<char>, length: usize) -> String {
    assert!(!pool.is_empty());

    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, pool.len());
            pool[idx]
        })
        .collect()
}

/// Calculates entropy.
fn calculate_entropy(length: u32, pool_size: usize) -> f64 {
    BigUint::from(pool_size)
        .pow(length)
        .to_f64()
        .expect("Typecast error! Failed to convert BigUint to f64.")
        .log2()
}

/// Calculates the required password length to obtain the given entropy.
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
    /// Creates new instance
    fn new(entropy: f64, length: usize, pool_size: usize) -> Self {
        Info {
            entropy,
            length,
            pool_size,
        }
    }

    /// Prints info
    // FixMe Как обработать ошибки?
    fn write(&self, mut writer: impl Write) {
        writeln!(writer, "Entropy: {:.0} bits", self.entropy).unwrap();
        writeln!(writer, "Length: {} chars", self.length).unwrap();
        writeln!(writer, "Pool size: {} chars", self.pool_size).unwrap();
    }
}
