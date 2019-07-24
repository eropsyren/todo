#[macro_use]
mod utils;
mod add;
mod constants;
mod discard;
mod done;
mod init;
mod list;
mod remove;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .subcommand(
            SubCommand::with_name("init").about("Initializes todo list in the current directory"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a task to todo list")
                .arg(
                    Arg::with_name("task")
                        .value_name("TASK")
                        .help("Task message")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("Lists all tasks in todo list"))
        .subcommand(
            SubCommand::with_name("done")
                .about("Marks a task as done")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("Task's id to be marked as done")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("discard")
                .about("Marks a task as discarded")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("Task's id to be marked as discarded")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove").about("Remove a task").arg(
                Arg::with_name("id")
                    .value_name("ID")
                    .help("Task's id to be marked as discarded")
                    .required(true),
            ),
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

    if let Some(matches) = matches.subcommand_matches("discard") {
        if let Some(id) = matches.value_of("id") {
            let f = |id| move || discard::discard(id);

            utils::if_todo_exists(f(id));
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        if let Some(id) = matches.value_of("id") {
            let f = |id| move || remove::remove(id);

            utils::if_todo_exists(f(id));
        }
    }
}
