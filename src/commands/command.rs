use clap::{App, ArgMatches};
use rusqlite::Connection;

pub struct Context {
    pub conn: Connection,
}

pub trait Command {
    fn execute<'c, 'd>(&self, &'c Context, &'d ArgMatches);
    fn subcommand<'a, 'b>() -> App<'a, 'b>;
}
