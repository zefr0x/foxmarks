use std::env;
use std::io::Error;
use std::fs::File;
use std::path::Path;

use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    // HACK: Create a stamp file to detect where is the latest outdir.
    let stamp_path = Path::new(&outdir).join("foxmarks-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    let mut cmd = build_cli();

    for shell in [Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh] {
        let path = generate_to(shell, &mut cmd, "foxmarks", outdir.clone())?;

        println!("cargo:warning=completion file is generated: {:?}", path);
    }

    Ok(())
}
