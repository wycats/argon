use argon_package;
use argon_package::read_manifest;
use clap::Arg;
use crate::thor;
use crate::thor::{ClapApp, Subcommand, ThorError};
use failure::ResultExt;

pub struct ReadManifest;

impl Subcommand for ReadManifest {
    fn definition(&self, app: ClapApp) -> ClapApp {
        app.arg(Arg::from_usage(
            "--pretty 'print the json in pretty format'",
        ))
    }

    fn run(&self, matches: &thor::CommandMatches) -> Result<(), ThorError> {
        let package = matches.command.value_of("package").unwrap();

        let manifest = read_manifest(package).with_context(|_| "readmanifest".to_string())?;
        let pretty = matches.command.is_present("pretty");

        let serialize = if pretty {
            argon_package::to_json_pretty
        } else {
            argon_package::to_json
        };

        println!(
            "{}",
            serialize(&manifest).with_context(|_| format!("failed to serialize manifest"))?
        );

        Ok(())
    }
}
