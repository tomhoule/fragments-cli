#[macro_use]
extern crate structopt;
extern crate fragments_cli;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "repl")]
    Repl,
    #[structopt(name = "server")]
    Server,
}

fn main() {
    match Command::from_args() {
        Command::Repl => panic!("repl"),
        Command::Server => panic!("server"),
    }
}
