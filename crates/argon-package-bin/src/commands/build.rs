use argon::compile_source;
use argon_package;
use argon_package::package_layout;
use clap::Arg;
use crate::thor;
use crate::thor::{ClapApp, Subcommand, ThorError};
use failure::ResultExt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{copy, Cursor};

pub struct Build;

impl Subcommand for Build {
    fn definition(&self, app: ClapApp) -> ClapApp {
        app.arg(Arg::from_usage(
            "--pretty 'print the json in pretty format'",
        ))
    }

    fn run(&self, matches: &thor::CommandMatches) -> Result<(), ThorError> {
        let package = matches.command.value_of("package").unwrap();

        let details = package_layout(package).with_context(|_| "packagelayout".to_string())?;

        let mut lib = File::open(&details.lib)
            .with_context(|_| format!("No lib found at {}", &details.lib.display()))?;

        let mut contents = String::new();
        lib.read_to_string(&mut contents)
            .with_context(|_| format!("Failed to read {}", &details.lib.display()))?;

        let mut bytes = compile_source(&details.lib, Cursor::new(contents))
            .with_context(|_| "compiling-source".to_string())?;

        let out = details
            .root
            .join("out")
            .join(&details.name)
            .with_extension("wasm");

        fs::create_dir_all(&details.out)
            .with_context(|_| format!("cannot create {}", &details.out.display()))?;

        let mut file =
            File::create(&out).with_context(|_| format!("cannot create {}", &out.display()))?;

        copy(&mut bytes, &mut file).with_context(|_| "write-file".to_string())?;

        Ok(())
    }
}
