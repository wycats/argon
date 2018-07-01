use assert_cli::Assert;

#[test]
fn help() {
    Assert::main_binary()
        .with_args(&["--help"])
        .succeeds()
        .and()
        .stdout()
        .is(indoc!(
            "
            argon 0.1.0

            USAGE:
                argon-package-bin.exe [FLAGS] [OPTIONS] [SUBCOMMAND]

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information
                -v, --verbose    verbose output

            OPTIONS:
                -o, --out-dir <OUT>        output directory [default: ./out]
                -p, --package <package>    directory of the package [default: .]

            SUBCOMMANDS:
                build            
                help             Prints this message or the help of the given subcommand(s)
                read-manifest
            "
        ))
        .unwrap()
}
