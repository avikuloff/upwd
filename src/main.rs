use clap::Clap;
use std::io::stdout;
use upwd::cli::{Cli, Info};
use upwd::config::Config;
use upwd::{calculate_length, generate_password};

fn main() {
    let opts: Cli = Cli::parse();

    // Todo обработать ошибки, добавить сообщение и завершить выполнение
    if opts.reset() {
        Config::save_default().unwrap();
    }

    let pool = opts.collect();

    let length = opts.entropy().map_or(opts.length(), |e| {
        calculate_length(e, pool.len() as f64).ceil() as usize
    });

    for _ in 0..opts.count() {
        let password = generate_password(&pool, length);

        println!("{}", password);
    }

    if opts.info() {
        Info::new(length, pool.len()).write(stdout());
    }
}
