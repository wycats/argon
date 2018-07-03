use argon::Compilation;
use argon_package::package_layout;
use clap::Arg;
use crate::thor;
use crate::thor::{ClapApp, Subcommand, ThorError};
use failure::ResultExt;
use parity_wasm::elements::Serialize;
use std::fs;
use std::fs::File;

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

        let mut compilation = Compilation::new();
        let key = compilation
            .add(&details.lib)
            .with_context(|_| "adding path".to_string())?;

        let module = compilation
            .get(&key)
            .with_context(|_| "compiling".to_string())?
            .unwrap();

        let out = details
            .root
            .join("out")
            .join(&details.name)
            .with_extension("wasm");

        fs::create_dir_all(&details.out)
            .with_context(|_| format!("cannot create {}", &details.out.display()))?;

        let mut file =
            File::create(&out).with_context(|_| format!("cannot create {}", &out.display()))?;

        module
            .into_owned()
            .serialize(&mut file)
            .with_context(|_| "write-file".to_string())?;

        Ok(())
    }
}
