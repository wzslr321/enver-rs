use std::path::PathBuf;
use structopt::StructOpt;
use std::home;

#[derive(StructOpt)]
#[structopt(about = "Even the best sometimes need -h or --help...")]
struct Opt {
    /// Search for the specified variable, and modify if found
    #[structopt(short, long)]
    modify: bool,

    /// Search for the specified variable, and delete if found
    #[structopt(short, long)]
    delete: bool,

    /// Use specified variable name to create a new one
    add: bool,

    /// Require double confirmation to perform changes
    #[structopt(long, default_value = true)]
    confirm: bool,

    /// Path to dotfile holding environment variables configuration.
    #[structopt(long, parse(from_os_str), default = resolve_home_path())]
    dot_path: PathBuf,

    /// Environment variable name to look for
    #[structopt(short, long, default = "PATH")]
    var: String,
}

fn main() {
}

fn resolve_home_path() -> PathBuf {
    home::home_dir()?
}
