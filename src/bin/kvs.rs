extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("get").about("Get value for key").arg(
                Arg::with_name("key")
                    .value_name("KEY")
                    .help("The key to search for")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove value for key")
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .help("The key to search for")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set value for key")
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .help("The key to search for")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("value")
                        .value_name("VALUE")
                        .help("The key to search for")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
