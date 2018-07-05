#[macro_use]
extern crate clap;

extern crate rusqlite;
use std::env;
use std::fs;

use clap::{App, AppSettings, Arg};
use rusqlite::Connection;
use std::path::Path;

mod schema;
mod types;
mod util;
use schema::SQL;

mod commands;
use commands::{AddCommand, Command, Context};

fn init(ctx: &Context) {
    println!("init");

    match ctx.conn.execute_batch(SQL) {
        Ok(updated) => println!("OK {:?}", updated),
        Err(err) => println!("Failed {}", err),
    }
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity")
                .default_value("1"),
        )
        .subcommands(vec![AddCommand::subcommand()]);

    let path = Path::new(&env::home_dir().unwrap()).join(".blt");
    let data_path = path.join("data");

    fs::create_dir_all(path).unwrap();

    let exists = data_path.exists();
    let conn = Connection::open(data_path.to_str().unwrap()).unwrap();
    let ctx = Context { conn: conn };

    if !exists {
        init(&ctx);
    }

    let matches = app.get_matches();
    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    match subcommand_name {
        "add" => AddCommand::new().execute(&ctx, &subcommand_matches),
        _ => println!("{}", matches.usage()),
    }

    ctx.conn.close().unwrap();
}
