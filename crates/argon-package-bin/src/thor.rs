pub mod error;
pub mod prelude;

use std::collections::HashMap;

pub use self::error::ThorError;
pub type ClapApp = clap::App<'static, 'static>;

pub struct App {
    app: ClapApp,
    main: Box<dyn Subcommand>,
    subcommands: HashMap<&'static str, Box<dyn Subcommand>>,
}

impl App {
    pub fn new(name: &'static str, main: impl Subcommand + 'static, version: &'static str) -> App {
        let app = clap::App::new(name).version(version);

        App {
            app,
            main: Box::new(main),
            subcommands: HashMap::new(),
        }
    }

    pub fn arg(mut self, arg: clap::Arg<'static, 'static>) -> App {
        self.app = self.app.arg(arg.global(true));
        self
    }

    pub fn subcommand(mut self, name: &'static str, subcommand: impl Subcommand + 'static) -> App {
        self.subcommands.insert(name, Box::new(subcommand));
        self
    }

    pub fn build(&self, args: impl Iterator<Item = String>) -> AppDispatcher {
        let App {
            app,
            main,
            subcommands,
        } = self;

        let mut app = app.clone();

        for (name, command) in subcommands {
            let subcommand = clap::SubCommand::with_name(name);
            let subcommand = command.definition(subcommand);

            app = app.subcommand(subcommand);
        }

        app = main.definition(app);

        let matches = app.clone().get_matches_from(args);

        let mut dispatch_command = main;
        let mut dispatch_name: Option<&str> = None;
        let dispatch_matches = matches.clone();

        for (name, command) in subcommands {
            if let Some(..) = matches.subcommand_matches(name) {
                dispatch_command = command;
                dispatch_name = Some(name);

                break;
            }
        }

        let dispatch_matches = match dispatch_name {
            Some(..) => CommandMatches::from_args(&dispatch_matches),
            None => CommandMatches::main(&dispatch_matches),
        };

        AppDispatcher {
            matches: dispatch_matches,
            command: dispatch_command,
        }
    }
}

#[derive(Debug)]
pub struct CommandMatches {
    pub global: clap::ArgMatches<'static>,
    pub command: clap::ArgMatches<'static>,
}

impl CommandMatches {
    fn main(matches: &'app clap::ArgMatches<'static>) -> CommandMatches {
        CommandMatches {
            global: matches.clone(),
            command: matches.clone(),
        }
    }

    fn from_args(matches: &'app clap::ArgMatches<'static>) -> CommandMatches {
        let command = &matches.subcommand.as_ref().unwrap().matches;

        CommandMatches {
            global: matches.clone(),
            command: command.clone(),
        }
    }
}

pub struct AppDispatcher<'app> {
    matches: CommandMatches,
    command: &'app Box<dyn Subcommand>,
}

impl AppDispatcher<'app> {
    pub fn dispatch(&self) -> Result<(), ThorError> {
        self.command.run(&self.matches)
    }

    pub fn matches(&self) -> &CommandMatches {
        &self.matches
    }
}

pub trait Subcommand {
    fn definition(&self, app: ClapApp) -> ClapApp {
        app
    }

    fn run(&self, matches: &CommandMatches) -> Result<(), ThorError>;
}
