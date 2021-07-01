use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::io::{self,Write};
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("enver-rs")?;
    cmd.arg("--dot-path").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("enver-rs")?;
    cmd.arg("--dot-path").arg(file.path());
    cmd.assert()
        .success();

    Ok(())
}