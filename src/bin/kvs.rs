extern crate clap;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION")
    )]
enum Opt {
    #[structopt(name = "set", about = "store a value for a key")]
    Set {
        #[structopt(value_name = "KEY", help = "The key to insert.")]
        key: String,
        #[structopt(value_name = "VALUE", help = "The value to insert.")]
        value: String,
    },
    #[structopt(name = "get", about = "get a value for a given key")]
    Get {
        #[structopt(value_name = "KEY", help = "The key to get the value")]
        key: String,
    },
    #[structopt(name = "rm")]
    Rm {
        #[structopt(value_name = "KEY", help = "The key to remove")]
        key: String,
    },
}

fn main() {

    match Opt::from_args() {
        Opt::Set { key, value } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Opt::Rm { key } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Opt::Get { key } => {
            eprintln!("unimplemented");
            exit(1);
        }

        _ => unreachable!(),
    }
}
