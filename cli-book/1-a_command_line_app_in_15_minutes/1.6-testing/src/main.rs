// Automated testing

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

fn answer() -> u64 {
    42
}

// Making your code testable

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn making_your_code_testable_1() -> Result<()> {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches_1(&content, &args.pattern);

    Ok(())
}

fn find_matches_1(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn making_your_code_testable_2() -> Result<()> {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches_2(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches_2(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches_2("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() {
    making_your_code_testable_2();
}
