use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use std::env;
use std::fs;
use Shell::*;

include!("src/cli.rs");

fn main() {
    let outdir = env::var("SHELL_COMPLETIONS_DIR")
        .or_else(|_| env::var("OUT_DIR"))
        .unwrap();
    fs::create_dir_all(&outdir).unwrap();
    let mut cmd = Interface::command();
    for shell in [Bash, PowerShell, Fish, Elvish, Zsh] {
        generate_to(shell, &mut cmd, "tre", &outdir).unwrap();
    }

    println!("cargo:rerun-if-changed=src/cli.rs");
}
