use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
#[structopt(
name = "enver-rs otherwise called the best env variables tool\n",
about = "Let's you modify your environment variables without an effort",
)]
struct Opt {
    /// Use specified variable name to create a new one
    #[structopt(short, long)]
    add: bool,

    /// Search for the specified variable, and delete if found
    #[structopt(short, long)]
    delete: bool,

    /// Search for the specified variable, and modify if found
    #[structopt(short, long)]
    modify: bool,

    /// Path to dotfile holding environment variables configuration.
    #[structopt(long, parse(from_os_str))]
    dot_path: PathBuf,

    /// Environment variable name to look for
    #[structopt(short, long, default_value = "PATH")]
    var: String,
}

fn main() -> Result<()> {
    let args:Opt = Opt::from_args();

    let add = args.add;
    let delete = args.delete;
    let modify = args.modify;

    if add {
       println!("Add");
    } else if delete {
       println!("Delete");
    } else if modify {
       println!("Modify");
    }

    let content = std::fs::read_to_string(&args.dot_path)
        .with_context(|| format!("could not read file `{}`", args.dot_path.display()))?;

    find_var(&content, &args.var, &mut std::io::stdout());

    Ok(())
}

fn find_var(content: &str, name: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(name) {
            writeln!(writer, "{}", line).ok();
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
    let name = Opt::default().get_default_var();
    find_var("PATH=test\nDEFINITELY_NOT_PAATH=test", name, &mut result);
    assert_eq!(result, b"PATH=test\n");
}
