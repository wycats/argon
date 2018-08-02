use argon::{AbsolutePath, ArgonError, Compilation, Database, GetResult, SkipResult, ToDiagnostic};
use argon_package::package_layout;
use clap::Arg;
use codespan::CodeMap;
use crate::thor::{self, ClapApp, Subcommand, ThorError};
use failure::ResultExt;
use parity_wasm::elements::Serialize;
use std::fs::{self, File};
use std::path::Path;
use std::path::PathBuf;

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

        let path = AbsolutePath::expand(details.lib)?;

        database.add_file(path.clone())?;

        let mut compilation = Compilation::new(database.shared());

        let module = compilation.get(&path);

        let module = match module {
            GetResult::Value(value) => value,
            GetResult::SkipResult(SkipResult::Error(err)) => {
                print_error(err, database.codemap(), &details.root);
                return Ok(());
            }
            GetResult::SkipResult(SkipResult::None) => unimplemented!(),
        };

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

        let value = module.clone_value();
        value.serialize(&mut file).unwrap();

        Ok(())
    }
}

fn print_error(error: ArgonError, codemap: &CodeMap, root: &Path) {
    match error {
        ArgonError::CompileError(err) => {
            let diagnostic = err.to_diagnostic();
            let term = language_reporting::termcolor::StandardStream::stderr(
                language_reporting::termcolor::ColorChoice::Auto,
            );
            language_reporting::emit(
                term,
                codemap,
                &diagnostic,
                &Config {
                    root: root.to_path_buf(),
                },
            ).unwrap();
        }

        other => unimplemented!("print_error {:?}", other),
    }
}

#[derive(Debug)]
struct Config {
    root: PathBuf,
}

impl language_reporting::Config for Config {
    fn filename(&self, path: &Path) -> String {
        pathdiff::diff_paths(path, &self.root)
            .unwrap()
            .display()
            .to_string()
    }
}
