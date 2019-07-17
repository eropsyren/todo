mod add;
mod constants;
mod init;
mod list;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand(SubCommand::with_name("init").about("initialize todo list in the current directory"))
        .subcommand(SubCommand::with_name("add").about("add a task to todo list"))
        .subcommand(SubCommand::with_name("list").about("list all tasks in todo list"))
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
