# upwd
Random password generator

## Installation
Use `cargo` package manager
```sh
cargo install upwd
```

## Usage
Generate a 12-character password using upper and lower case letters, numbers, and symbols.
```
upwd -ulds -L 12
```
For more information use `--help` flag
```
upwd --help

upwd 0.1.0
Andrey Vikulov <avikuloff@yandex.ru>
Random password generator

USAGE:
    upwd [FLAGS] [OPTIONS] <--uppercase|--lowercase|--digits|--symbols>

FLAGS:
    -d, --digits       Use digits [0-9]
    -h, --help         Prints help information
    -i, --info         Prints password information
    -l, --lowercase    Uses lowercase [a-z]
    -s, --symbols      Use special symbols [*&^%$#@!~]
    -u, --uppercase    Use UPPERCASE letters [A-Z]
    -V, --version      Prints version information

OPTIONS:
    -E, --entropy <NUMBER>    Sets the minimum required password entropy (conflicts with --length)
    -L, --length <NUMBER>     Sets the required password length [default: 12]
```

## License
`upwd` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).