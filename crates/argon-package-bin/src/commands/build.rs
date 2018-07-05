use argon::{AbsolutePath, Compilation, Database};
use argon_package::package_layout;
use clap::Arg;
use crate::thor::{self, ClapApp, Subcommand, ThorError};
use failure::ResultExt;
use parity_wasm::elements::Serialize;
use std::fs::{self, File};

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

        let mut database = Database::new();

        let path = AbsolutePath::expand(details.lib).with_context(|_| "expandpath".to_string())?;

        database
            .add_file(path.clone())
            .with_context(|_| "adding path".to_string())?;

        let mut compilation = Compilation::new(database.shared());

        let module = compilation.get(&path).unwrap().clone();

        let out_file = details
            .root
            .join("out")
            .join(&details.name)
            .with_extension("wasm");

        let out_dir = &details.out;
        fs::create_dir_all(out_dir)
            .with_context(|_| format!("cannot create {}", &out_dir.display()))?;

        let mut file = File::create(&out_file)
            .with_context(|_| format!("cannot create {}", &out_dir.display()))?;

        module.clone_value().serialize(&mut file).unwrap();

        Ok(())
    }
}
