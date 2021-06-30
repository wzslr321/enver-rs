use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    // Belissima
    println!("{:?}", belissima());

    // spaghetti
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
        }
        Err(error) => { println!("Oh no: {} ", error); }
    }
}

fn belissima() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(&path).with_context(|| format!("Could not read the file `{}", path))?;
    println!("File content: {}", content);
    Ok(())
}