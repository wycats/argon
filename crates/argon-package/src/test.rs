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
                argon-package.exe [OPTIONS] [package]

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -o, --out-dir <OUT>    output directory [default: ./out]

            ARGS:
                <package>    directory of the package [default: .]
        "
        ))
        .unwrap()
}
