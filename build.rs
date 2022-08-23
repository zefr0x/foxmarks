use std::env;
use std::io::Error;

use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();

    for shell in [Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh] {
        let path = generate_to(shell, &mut cmd, "foxmarks", outdir.clone())?;

        println!("cargo:warning=completion file is generated: {:?}", path);
    }

    Ok(())
}
