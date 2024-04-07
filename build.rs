use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let name = "foxmarks";

    // Locate cargo's outdir
    let outdir_string = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };
    let outdir = Path::new(&outdir_string);

    // HACK: Create a stamp file to detect where is the latest outdir.
    let stamp_path = outdir.join(format!("{name}-stamp"));
    #[allow(clippy::unwrap_used)]
    fs::File::create(stamp_path).unwrap();

    let mut cmd = build();

    // Create directory for shell completions
    let completions_dir = outdir.join("shell_completions");
    #[allow(clippy::unwrap_used)]
    fs::create_dir_all(&completions_dir).unwrap();

    // Generate shell completions
    for shell in [Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh] {
        generate_to(shell, &mut cmd, name, &completions_dir)?;
    }

    // Create directory for man pages
    let man_dir = outdir.join("man_pages");
    #[allow(clippy::unwrap_used)]
    fs::create_dir_all(&man_dir).unwrap();

    // Generate main man page
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    fs::write(man_dir.join(format!("{name}.1")), buffer)?;

    // Generate sub man pages
    for subcommand in cmd.get_subcommands() {
        if let Some(display_name) = subcommand.get_display_name() {
            let man = Man::new(subcommand.clone());
            let mut buffer = Vec::new();
            man.title(display_name).render(&mut buffer)?;

            fs::write(man_dir.join(format!("{display_name}.1")), buffer)?;
        }
    }

    println!("cargo:warning=completion files and man pages are generated: {outdir:?}");

    Ok(())
}
