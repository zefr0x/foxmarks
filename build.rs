use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
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
    let stamp_path = outdir.join(format!("{}-stamp", name));
    if let Err(err) = fs::File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    let mut cmd = build_cli();

    // Create directory for shell completions
    let completions_dir = outdir.join("shell_completions");
    match fs::create_dir(&completions_dir) {
        Ok(_) => {}
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {}
            other_error => {
                panic!("Can't create the directory, {:?}", other_error)
            }
        },
    };

    // Generate shell completions
    for shell in [Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh] {
        generate_to(shell, &mut cmd, name, &completions_dir)?;
    }

    // Create directory for man pages
    let man_dir = outdir.join("man_pages");
    match fs::create_dir_all(&man_dir) {
        Ok(_) => {}
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {}
            other_error => {
                panic!("Can't create the directory, {:?}", other_error)
            }
        },
    };

    // Generate main man page
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    fs::write(man_dir.join(format!("{}.1", name)), buffer)?;

    // Generate sub man pages
    for subcommand in cmd.get_subcommands() {
        let display_name = subcommand.get_display_name().unwrap();

        let man = Man::new(subcommand.clone());
        let mut buffer = Vec::new();
        man.title(display_name).render(&mut buffer)?;

        fs::write(man_dir.join(format!("{}.1", display_name)), buffer)?;
    }

    println!(
        "cargo:warning=completion files and man pages are generated: {:?}",
        outdir
    );

    Ok(())
}
