#[cfg(test)]
#[macro_use]
extern crate indoc;

#[macro_use]
extern crate derive_new;

mod commands;

#[cfg(test)]
mod test;
pub mod thor;

use self::thor::ThorError;
use clap::Arg;
use crate::thor::{App, ClapApp, Subcommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("argon", Main, VERSION)
        .subcommand("read-manifest", commands::ReadManifest)
        .subcommand("build", commands::Build)
        .arg(Arg::from_usage("--package -p 'directory of the package'").default_value("."))
        .arg(Arg::from_usage("--verbose -v 'verbose output'"));

    let target = app.build(std::env::args());
    let verbose = target.matches().global.is_present("verbose");

    match target.dispatch() {
        Ok(()) => return,
        Err(e) => println!("{}", e.format(verbose)),
    }
}

struct Main;

impl Subcommand for Main {
    fn definition(&self, app: ClapApp) -> ClapApp {
        app.arg(
            Arg::from_usage("[out-dir] -o, --out-dir=[OUT] 'output directory'")
                .default_value("./out"),
        )
    }

    fn run(&self, matches: &thor::CommandMatches) -> Result<(), ThorError> {
        println!("{:?}", matches.command.value_of("out-dir").unwrap());
        println!("{:?}", matches.command.value_of("package").unwrap());

        Ok(())
    }
}
