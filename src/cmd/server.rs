
use super::types::*;

use super::super::api;

/// Get the server subcommand for the cmdline
/// Partof: 
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("server")
        .about("start the web-ui server")
        .settings(&[AS::DeriveDisplayOrder, COLOR])
        .arg(Arg::with_name("addr")
                 .help("full address to start server on. Default='127.0.0.1:8000'")
                 .use_delimiter(false))
}


/// pull out the command settings
pub fn get_cmd(matches: &ArgMatches) -> String {
    matches.value_of("addr").unwrap_or("127.0.0.1:4000").to_string()
}

// TODO: should technically return result
// need to do conditional compilation on types
// to auto-convert web errors
pub fn run_cmd(project: Project, addr: &str) {
    api::start_api(project, addr);
}