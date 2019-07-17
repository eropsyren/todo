mod init;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init::init();
    }
}
