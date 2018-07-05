use clap::{App, Arg, ArgMatches, SubCommand};

use commands::command::Command;
use commands::command::Context;
use types::Task;
use util::timestamp;

pub struct AddCommand {}

impl AddCommand {
    pub fn new() -> AddCommand {
        AddCommand {}
    }
}

impl Command for AddCommand {
    fn subcommand<'b, 'c>() -> App<'b, 'c> {
        SubCommand::with_name("add").about("add").arg(
            Arg::with_name("text")
                .help("The text to add")
                .required(true),
        )
    }

    fn execute<'d, 'e>(&self, ctx: &'d Context, matches: &'e ArgMatches) {
        println!("add");

        let conn = &ctx.conn;
        let mut task = Task {
            id: 0,
            text: matches.value_of("text").unwrap(),
            created: timestamp(),
        };

        conn.execute(
            "INSERT INTO task (text, created)
                VALUES (?1, ?2)",
            &[&task.text, &task.created],
        ).unwrap();
        conn.query_row("SELECT max(id) from task", &[], |row| {
            task.id = row.get(0);
        }).unwrap();

        println!("{:?}", task);
    }
}
