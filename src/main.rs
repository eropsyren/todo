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
        let title = matches.value_of("title");
        let description = matches.value_of("description");

        match (title, description) {
            (Some(title), Some(description)) => {
                let f = |t, d| move || add::add_full(t, d);

                utils::if_todo_exists(f(title, description));
            }
            (Some(title), None) => {
                let f = |t| move || add::add_title(t);

                utils::if_todo_exists(f(title));
            }
            _ => utils::if_todo_exists(add::add_with_prompt),
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
