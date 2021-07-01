use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};

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
    #[structopt(short, long)]
    add: bool,

    /// Path to dotfile holding environment variables configuration.
    #[structopt(long, parse(from_os_str))]
    dot_path: PathBuf,

    /// Environment variable name to look for
    #[structopt(short, long, default_value = "PATH")]
    var: String,
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    let content = std::fs::read_to_string(&args.dot_path)
        .with_context(|| format!("could not read file `{}`", args.dot_path.display()))?;

    find_var(&content, &args.var, &mut std::io::stdout());

    Ok(())
}

fn find_var(content: &str, name: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(name) {
            writeln!(writer, "{}", line);
        }
    }
}

impl Opt {
    fn get_default_var(&self) -> &String {
        &self.var
    }

    fn default() -> Opt {
        let mut path = PathBuf::new();
        path.push("~/.zshrc");
        Opt {
            dot_path: path,
            add: false,
            delete: false,
            modify: false,
            var: "PATH".to_string(),
        }
    }
}

#[test]
fn find_env_var() {
    let mut result = Vec::new();
    let opt = Opt::default();
    find_var("PATH=test\nDEFINITELY_NOT_PAATH=test", opt.get_default_var(), &mut result);
    assert_eq!(result, b"PATH=test\n");
}
