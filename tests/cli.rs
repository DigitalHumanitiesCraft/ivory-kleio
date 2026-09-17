use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::{Value, json};
use tempfile::tempdir;

const INPUT: &str = include_str!("../research-artefacts/cases/temporal-alternatives.json");

fn run(input: &Path, year: &str, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ivory-temporal"))
        .arg(input)
        .arg(year)
        .arg(output)
        .output()
        .expect("run ivory-temporal")
}

fn successful_report(output: &Output) -> Value {
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stderr.is_empty(), "{:?}", output);
    serde_json::from_slice(&output.stdout).expect("stdout contains only the JSON report")
}

fn failed(output: &Output, category: Option<&str>) {
    assert!(!output.status.success(), "{:?}", output);
    assert!(output.stdout.is_empty(), "{:?}", output);
    assert!(!output.stderr.is_empty(), "{:?}", output);
    if let Some(category) = category {
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(category),
            "{:?}",
            output
        );
    }
}

#[test]
fn valid_cli_run_matches_independent_result_and_reopens_saved_file() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.json");
    let saved = dir.path().join("saved.json");
    let reopened = dir.path().join("reopened.json");
    fs::write(&input, INPUT).unwrap();

    let result = successful_report(&run(&input, "1705", &saved));
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/temporal-results-v1.json")).unwrap();
    assert_eq!(result, expected);
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&saved).unwrap()).unwrap(),
        serde_json::from_str::<Value>(INPUT).unwrap()
    );
    assert_eq!(successful_report(&run(&saved, "1705", &reopened)), result);
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&saved).unwrap()).unwrap(),
        serde_json::from_slice::<Value>(&fs::read(&reopened).unwrap()).unwrap()
    );
    assert_eq!(fs::read(&input).unwrap(), INPUT.as_bytes());
}

#[test]
fn repeated_executions_in_fresh_directories_preserve_structured_results_and_snapshots() {
    let mut outputs = Vec::new();
    for _ in 0..2 {
        let dir = tempdir().unwrap();
        let input = dir.path().join("input.json");
        let saved = dir.path().join("saved.json");
        fs::write(&input, INPUT).unwrap();
        let output = run(&input, "1705", &saved);
        outputs.push((
            successful_report(&output),
            serde_json::from_slice::<Value>(&fs::read(&saved).unwrap()).unwrap(),
        ));
    }
    assert_eq!(outputs[0], outputs[1]);
}

#[test]
fn invalid_input_fails_before_creating_output() {
    let mut invalid_record: Value = serde_json::from_str(INPUT).unwrap();
    invalid_record["records"][0]["interpretation"]["temporal"]["years"] = json!([]);
    let cases = [
        (invalid_record.to_string(), "EmptyAlternatives"),
        (
            json!({"format_version": 2, "records": []}).to_string(),
            "UnsupportedSnapshotVersion",
        ),
        ("{".to_owned(), "InvalidJson"),
        (String::new(), "InvalidJson"),
        ("[1,[]]".to_owned(), "InvalidJson"),
    ];
    for (text, category) in cases {
        let dir = tempdir().unwrap();
        let input = dir.path().join("input.json");
        let saved = dir.path().join("saved.json");
        fs::write(&input, &text).unwrap();
        failed(&run(&input, "1705", &saved), Some(category));
        assert!(!saved.exists());
        assert_eq!(fs::read(&input).unwrap(), text.as_bytes());
    }
}

#[test]
fn invalid_query_is_rejected_even_for_an_empty_collection() {
    for text in [INPUT, r#"{"format_version":1,"records":[]}"#] {
        for (year, category) in [
            ("0", Some("InvalidYear")),
            ("10000", Some("InvalidYear")),
            ("not-a-year", None),
            ("", None),
        ] {
            let dir = tempdir().unwrap();
            let input = dir.path().join("input.json");
            let saved = dir.path().join("saved.json");
            fs::write(&input, text).unwrap();
            failed(&run(&input, year, &saved), category);
            assert!(!saved.exists());
        }
    }
}

#[test]
fn empty_collection_with_valid_query_is_saved_and_reported() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.json");
    let saved = dir.path().join("saved.json");
    fs::write(&input, r#"{"format_version":1,"records":[]}"#).unwrap();
    assert_eq!(
        successful_report(&run(&input, "1705", &saved)),
        json!({
            "query_year": 1705,
            "round_trip_verified": true,
            "results": []
        })
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&saved).unwrap()).unwrap(),
        json!({
            "format_version": 1,
            "records": []
        })
    );
}

#[test]
fn missing_input_and_missing_arguments_fail_without_stdout() {
    let dir = tempdir().unwrap();
    let saved = dir.path().join("saved.json");
    failed(&run(&dir.path().join("missing.json"), "1705", &saved), None);
    assert!(!saved.exists());
    failed(
        &Command::new(env!("CARGO_BIN_EXE_ivory-temporal"))
            .output()
            .unwrap(),
        None,
    );
}

#[test]
fn existing_destination_is_preserved_byte_for_byte() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.json");
    let saved = dir.path().join("saved.json");
    let sentinel = b"existing destination\n\0\xff";
    fs::write(&input, INPUT).unwrap();
    fs::write(&saved, sentinel).unwrap();
    failed(&run(&input, "1705", &saved), None);
    assert_eq!(fs::read(&saved).unwrap(), sentinel);
    assert_eq!(fs::read(&input).unwrap(), INPUT.as_bytes());
}

#[test]
fn same_input_and_output_is_preserved_byte_for_byte() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.json");
    fs::write(&input, INPUT).unwrap();
    failed(&run(&input, "1705", &input), None);
    assert_eq!(fs::read(&input).unwrap(), INPUT.as_bytes());
}
