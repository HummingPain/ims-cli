use std::path::PathBuf;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;
use ims_cli::{Cli, find_matches}; // Run programs

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let arg_one = Cli {
        pattern: String::from("eric"),
        path: PathBuf::from("test.txt")
    };
    let test_content = String::from("eric");
    find_matches(&arg_one, test_content, &mut result);
    assert_eq!(result, b"eric\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ims-cli")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("ims-cli")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
