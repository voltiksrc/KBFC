# KBFC

KBFC (Koiinit But For C) is a small C project initializer written in Rust.

It creates a basic C17 project with a Makefile and some default files, so you don't have to set everything up yourself.

## Installation

Dependencies:
- Rust
- Cargo

```sh
cargo install --path .
```

## Usage

```sh
kbfc hello
cd hello
make
./hello
```

This generates:

```text
hello/
├── include/
├── src/
│   └── main.c
├── Makefile
├── README.md
└── .gitignore
```

## Options

- `-h`, `--help` - Show help
- `-v`, `--version` - Show version

## License

KBFC is licensed under MPL-2.0. Projects generated with KBFC can use any license you want.
