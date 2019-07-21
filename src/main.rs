#[macro_use]
mod utils;
mod add;
mod constants;
mod init;
mod list;
mod task;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand(
            SubCommand::with_name("init").about("initialize todo list in the current directory"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("add a task to todo list")
                .arg(Arg::with_name("task").value_name("TASK").required(true)),
        )
        .subcommand(SubCommand::with_name("list").about("list all tasks in todo list"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        utils::if_todo_not_exists(init::init);
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(task) = matches.value_of("task") {
            let f = |task| move || add::add(task);

            utils::if_todo_exists(f(task));
        }
    }

    if let Some(_) = matches.subcommand_matches("list") {
        utils::if_todo_exists(list::list);
    }
}
