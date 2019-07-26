#[macro_use]
mod utils;
mod add;
mod clear;
mod constants;
mod discard;
mod done;
mod edit;
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
                    Arg::with_name("title")
                        .short("t")
                        .long("title")
                        .value_name("TITLE")
                        .help("Task title")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .long("description")
                        .value_name("DESCRIPTION")
                        .help("Task description")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all tasks in todo list")
                .arg(
                    Arg::with_name("long")
                        .short("l")
                        .long("long")
                        .help("List tasks with their id")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .help("List the task with the given ID")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("filter")
                        .short("f")
                        .long("filter")
                        .value_name("done | undone | discarded")
                        .help("List all tasks with the given tag")
                        .takes_value(true),
                ),
        )
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
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit a task description")
                .arg(
                    Arg::with_name("id")
                        .value_name("ID")
                        .help("The id of the task to edit")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("clear").about("Clears all done tasks"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        utils::if_todo_not_exists(init::init);
    }

    if let Some(matches) = matches.subcommand_matches("add") {
        let title = matches.value_of("title");
        let description = matches.value_of("description");

        match (title, description) {
            (Some(title), Some(description)) => {
                let f = || add::add_full(title, description);

                utils::if_todo_exists(f);
            }
            (Some(title), None) => {
                let f = || add::add_title(title);

                utils::if_todo_exists(f);
            }
            _ => utils::if_todo_exists(add::add_with_prompt),
        }
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        if let Some(id) = matches.value_of("id") {
            let f = || list::list_task(id);

            utils::if_todo_exists(f);
        } else {
            let is_long = matches.is_present("long");
            let filter = matches.value_of("filter");
            let f = || list::list(is_long, filter);

            utils::if_todo_exists(f);
        }
    }

    if let Some(matches) = matches.subcommand_matches("done") {
        if let Some(id) = matches.value_of("id") {
            let f = || done::done(id);

            utils::if_todo_exists(f);
        }
    }

    if let Some(matches) = matches.subcommand_matches("discard") {
        if let Some(id) = matches.value_of("id") {
            let f = || discard::discard(id);

            utils::if_todo_exists(f);
        }
    }

    if let Some(matches) = matches.subcommand_matches("remove") {
        if let Some(id) = matches.value_of("id") {
            let f = || remove::remove(id);

            utils::if_todo_exists(f);
        }
    }

    if let Some(matches) = matches.subcommand_matches("edit") {
        if let Some(id) = matches.value_of("id") {
            let f = || edit::edit(id);

            utils::if_todo_exists(f);
        }
    }

    if let Some(_) = matches.subcommand_matches("clear") {
        utils::if_todo_exists(clear::clear);
    }
}
