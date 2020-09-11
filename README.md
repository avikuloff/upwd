# upwd
Random password generator

## Installation
Use `cargo` package manager
```sh
cargo install upwd
```

## Features
- Generating password with a given length or entropy
- Customizable and extensible character sets
- Unicode character support

## Usage example
Generate a 12-character password using upper and lower case letters, numbers, and symbols.
```
upwd -ulds -L 12
```
For more information use `--help` flag
```
upwd --help

upwd 0.1.1
Andrey Vikulov <avikuloff@yandex.ru>
Random password generator

USAGE:
    upwd [FLAGS] [OPTIONS] <--uppercase|--lowercase|--digits|--symbols|--others>

FLAGS:
    -u, --uppercase    Use UPPERCASE letters [A-Z]
    -l, --lowercase    Uses lowercase [a-z]
    -d, --digits       Use digits [0-9]
    -s, --symbols      Use special symbols [*&^%$#@!~]
    -o, --others       Use other symbols (see config file)
    -i, --info         Prints password information
        --config       Sets config to default values
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -L, --length <NUMBER>     Sets the required password length [default: 12]
    -E, --entropy <NUMBER>    Sets the minimum required password entropy (conflicts with --length)
```
### Edit character sets
Run program with `--config` flag, this will create a config file at `/path/to/config/dir/upwd/upwd.conf`.
Change the character sets in this file to your liking.

## License
`upwd` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).