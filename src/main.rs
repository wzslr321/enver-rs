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
    let args: Opt = Opt::from_args();

    let add = args.add;
    let delete = args.delete;
    let modify = args.modify;

    let action:&str;

    if add {
        action = "add";
    } else if delete {
        action = "delete";
    } else if modify {
        action = "modify";
    }

    let content = std::fs::read_to_string(&args.dot_path)
        .with_context(|| format!("could not read file `{}`", args.dot_path.display()))?;

    let results_number = find_var(&content, &args.var, &mut std::io::stdout());

    print_choose_msg(results_number, &action);

    Ok(())
}

fn find_var(content: &str, name: &str, mut writer: impl std::io::Write) -> u8 {
    let mut is_found: u8 = 0;

    for line in content.lines() {
        if line.contains(name) {
            is_found += 1;
            writeln!(writer, "{}. {}", is_found, line).ok();
        }
    }

    is_found
}

fn print_choose_msg(results:u8, action: &str) {
    if results > 0 {
        println!("Environment variable(s) have been found, choose which one you want to {}", action);
    } else {
        println!("No environment variable has been found :(");
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
