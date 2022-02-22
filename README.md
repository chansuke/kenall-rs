# kenall-rs
![Cargo Test](https://github.com/chansuke/kenall-rs/workflows/Cargo%20Test/badge.svg) ![License](https://img.shields.io/badge/license-MIT%20or%20Apache%202%20-orange) [![kenall-rs at crates.io](https://img.shields.io/crates/v/kenall-rs.svg)](https://crates.io/crates/kenall-rs) ![kenall-rs at doc.rs](https://docs.rs/kenall-rs/badge.svg?version=0.1.0)
## About The Project
kenall-rs is an **UNOFFICIAL** command line tool for [KENALL](https://kenall.jp/) providing `REST API` that enables to search the addresses from postal code straightforward and efficient.

## Setup

```sh
$ cp .env.example .env
```

or

```
$ cp .envrc.tempate .envrc
$ direnv allow
```

and please set your [API key](https://kenall.jp/docs/account/api-key/)

## Installation
### From crates.io

```sh
$ cargo install kenall-rs
```

### From latest main branch commit

```sh
$ cargo install --git https://github.com/chansuke/kenall-rs --branch main
```

### From Homebrew
```sh
$ brew tap chansuke/tap
$ brew install chansuke/tap/kenall-rs
```

## Usage
Please enter the 7digits numeric postal code that you want to search for the Japanese address.

```sh
kenall-rs NNNNNNN`
```

or

```sh
kenall-rs NNN-NNNN
```

## License
kenall-rs is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENCE-APACHE](LICENCE-APACHE), [LICENCE-MIT](LICENCE-MIT) for more information.
