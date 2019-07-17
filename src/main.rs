mod add;
mod constants;
mod init;
mod list;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand(SubCommand::with_name("init"))
        .subcommand(SubCommand::with_name("add"))
        .subcommand(SubCommand::with_name("list"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        init::init();
    }

    if let Some(_) = matches.subcommand_matches("add") {
        add::add();
    }

    if let Some(_) = matches.subcommand_matches("list") {
        list::list();
    }
}
