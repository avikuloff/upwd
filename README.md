# upwd
Random password generator

## Installation
Use `cargo` package manager
```sh
cargo install upwd
```

## Features
- Generation of password for a given length or entropy
- Generation of multiple passwords
- User-definable character sets
- Unicode Character Support

## Usage examples
Generates a 12-character password using upper and lowercase letters, digits, special symbols and unicode characters:
```
upwd -uldso -L 12
```
Generate a password with default settings:
```
upwd
```
For more information use `--help` flag
```
upwd 0.3.0
Andrey Vikulov <avikuloff@yandex.ru>
Random password generator

USAGE:
    upwd [FLAGS] [OPTIONS]

FLAGS:
    -u, --uppercase    Use UPPERCASE letters [A-Z]
    -l, --lowercase    Use lowercase letters [a-z]
    -d, --digits       Use digits [0-9]
    -s, --symbols      Use special symbols [*&^%$#@!~]
    -o, --others       Use other symbols [♕♖♗♘♙♚...]
    -i, --info         Prints password information
        --config       Sets config to default values
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -L, --length <NUMBER>     Sets the required password length [default: 12]
    -E, --entropy <NUMBER>    Sets the minimum required password entropy (conflicts with --length)
    -c, --count <NUMBER>      Number of passwords [default: 1]

If you do not specify any of the [--uppercase, --lowercase, --digits, --symbols, --others] flags, then
uppercase, lowercase letters and digits will be used.
```
### Edit character sets
Run the program with `--config` flag, this will create a config file in `/path/to/config/dir/upwd/upwd.conf`.
Open this file in a text editor and change the character sets.

## License
`upwd` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).