use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let commands = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .help("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .help("Get string value of a given key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .help("Remove a given key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .get_matches();

    match commands.subcommand() {
        ("set", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            exit(1);
        }
    }
}
