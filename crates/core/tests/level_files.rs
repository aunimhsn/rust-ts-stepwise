//! Every level file of the website, read and checked as the core reads it, with its starting
//! code checked by the lexer, the parser and the checker: a level that starts wrong is a bug
//! of the level.

use std::fs;
use std::path::{Path, PathBuf};

use stepwise_core::checker::check;
use stepwise_core::level::{Level, Sandbox};
use stepwise_core::parser::parse;

/// The folder of the level files, `web/levels/`.
fn levels_folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../web/levels")
}

/// The level files, sorted by name.
fn level_files() -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(levels_folder())
        .expect("the folder of the levels")
        .map(|entry| entry.expect("an entry of the folder").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
        .collect();
    files.sort();
    files
}

/// Whether `name` is in kebab-case: the stable name of a level, in the progress saved and in
/// the names of its Fluent messages.
fn is_kebab_case(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
        && name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

/// The errors of the starting code, empty when it has none.
fn starting_code_errors(code: &str) -> Vec<String> {
    match parse(code) {
        Ok(program) => check(&program)
            .into_iter()
            .map(|error| format!("{error:?}"))
            .collect(),
        Err(error) => vec![format!("{error:?}")],
    }
}

#[test]
fn every_level_file_is_valid() {
    let files = level_files();
    assert!(
        files.iter().any(|file| file.ends_with("sandbox.toml")),
        "no sandbox.toml in {}",
        levels_folder().display()
    );
    let mut failures = Vec::new();
    for file in &files {
        let name = file
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if !is_kebab_case(name) {
            failures.push(format!("{name}: the name of a level is in kebab-case"));
        }
        let source = fs::read_to_string(file).expect("a level file");
        let starting_code = if name == "sandbox" {
            Sandbox::from_toml(&source).map(|sandbox| sandbox.starting_code().to_owned())
        } else {
            Level::from_toml(&source).map(|level| level.starting_code().to_owned())
        };
        match starting_code {
            Ok(code) => failures.extend(
                starting_code_errors(&code)
                    .into_iter()
                    .map(|error| format!("{name}: the starting code: {error}")),
            ),
            Err(error) => failures.push(format!("{name}: {error}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn the_names_of_the_levels_are_in_kebab_case() {
    for name in ["sandbox", "paint-a-square", "level-2"] {
        assert!(is_kebab_case(name), "{name}");
    }
    for name in [
        "",
        "Paint",
        "paint_a_square",
        "-paint",
        "paint-",
        "paint--a",
        "carré",
    ] {
        assert!(!is_kebab_case(name), "{name}");
    }
}

#[test]
fn a_starting_code_with_an_error_is_found() {
    assert_eq!(
        starting_code_errors("# Try anything here.\nforward()\n"),
        [""; 0]
    );
    assert_eq!(starting_code_errors("forward(\n").len(), 1);
    assert_eq!(starting_code_errors("x = 3\n").len(), 1);
}
