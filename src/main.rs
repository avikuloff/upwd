use crate::cli::{Cli, Info};
use crate::config::Config;
use clap::Clap;
use std::io::stdout;
use upwd_lib::{calculate_length, generate_password};

pub mod cli;
pub mod config;

fn main() {
    let opts: Cli = Cli::parse();

    if opts.reset() {
        match Config::save_default() {
            Ok(_) => {
                println!("The default configuration was set successfully!");
                std::process::exit(exitcode::OK);
            }
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }

    let pool = opts.collect();

    let length = opts.entropy().map_or(opts.length(), |e| {
        calculate_length(e, pool.len() as f64) as usize
    });

    for _ in 0..opts.count() {
        let password = generate_password(&pool, length);

        println!("{}", password);
    }

    if opts.info() {
        Info::new(length, pool.len()).write(stdout());
    }
}
