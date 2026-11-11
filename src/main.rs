// SPDX-License-Identifier: MPL-2.0

use std::env;

mod project;
mod validate;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: kbfc <project-name>");
        std::process::exit(1);
    }

    let name = &args[1];

    match name.as_str() {
        "--help" | "-h" => {
            println!(
                "\
        KBFC - Koiinit But For C

    Usage:
        kbfc <project-name>
        kbfc [options]

    Description:
        A lightweight C project initializer written in Rust.
        Generates a ready to build C17 project with a Makefile.

    Options:
        -h, --help       Show this help message
        -v, --version    Show KBFC's version

    Example:
        kbfc hello
    "
            );
            return;
        }

        "--version" | "-v" => {
            println!("kbfc {}", env!("CARGO_PKG_VERSION"));
            return;
        }

        _ => {}
    }

    if !validate::name_is_valid(name) {
        std::process::exit(1);
    }

    println!("[KBFC] Creating project '{name}'...");

    // Since our create_project variable returns a Result, if it
    // fails, 'err' holds the actual error we can print.
    match project::create_project(name) {
        Ok(()) => {
            println!("[KBFC] Project '{name}' created successfully!");
        }

        Err(err) => {
            eprintln!("[KBFC] Failed to create project: {err}");
            std::process::exit(1);
        }
    }
}
