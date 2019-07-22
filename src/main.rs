#[macro_use]
mod utils;
mod add;
mod constants;
mod done;
mod init;
mod list;

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
        .subcommand(
            SubCommand::with_name("done")
                .about("mark a task as done")
                .arg(Arg::with_name("id").value_name("ID").required(true)),
        )
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

    if let Some(matches) = matches.subcommand_matches("done") {
        if let Some(id) = matches.value_of("id") {
            let f = |id| move || done::done(id);

            utils::if_todo_exists(f(id));
        }
    }
}
