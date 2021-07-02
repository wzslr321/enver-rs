use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};
use text_io::read;
use std::collections::HashMap;

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

    let action_option: Option<&str>;

    if add {
        action_option = Some("add");
    } else if delete {
        action_option = Some("delete");
    } else if modify {
        action_option = Some("modify");
    } else {
        action_option = None;
    }

    let content = std::fs::read_to_string(&args.dot_path)
        .with_context(|| format!("could not read file `{}`", args.dot_path.display()))?;


    let mut result_num: u8 = 0;
    let mut result_map: Option<HashMap<u8, &str>> = None;

    if action_option != None {
        let (rn, rm) = find_var(&content, &args.var, &mut std::io::stdout());
        result_num = rn;
        result_map = Some(rm);
    }

    let option: u8;

    match action_option {
        None => option = return_action_options(),
        Some(result) => option = print_choose_msg(result_num, &result),
    };

    if result_map != None {
        println!("{:?}", result_map);
    }

    Ok(())
}

fn get_user_option() -> u8 {
    read!()
}

fn return_action_options() -> u8 {
    println!("Please, first specify what do you want to do. \n 1. Add \n 2. Delete \n 3. Modify ");
    get_user_option()
}

fn find_var<'a>(content: &'a str, name: &'a str, mut writer: impl std::io::Write) -> (u8, HashMap<u8, &'a str>) {
    let mut is_found: u8 = 0;
    let mut vars_num_option_map = HashMap::new();

    for line in content.lines() {
        if line.contains(name) {
            is_found += 1;
            vars_num_option_map.insert(is_found, line);
            writeln!(writer, "{}. {}", is_found, line).ok();
        }
    }

    (is_found, vars_num_option_map)
}

fn print_choose_msg(results: u8, action: &str) -> u8 {
    if results > 0 {
        println!("Environment variable(s) have been found, choose which one you want to {}", action);
    } else {
        println!("No environment variable has been found :(");
    }

    get_user_option()
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
