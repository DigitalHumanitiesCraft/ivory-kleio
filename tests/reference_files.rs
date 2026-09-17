use ivory_kleio::{Snapshot, could_occur_in_year};
use serde_json::{Value, json};

const ORIGINAL_V1: &str = include_str!("../research-artefacts/cases/temporal-alternatives.json");
const EXPECTED_REPORT: &str = include_str!("fixtures/temporal-results-v1.json");
const SOURCE_FIDELITY_V1: &str = include_str!("fixtures/snapshot-v1-source-fidelity.json");

fn actual_results(snapshot: &Snapshot, year: i32) -> Value {
    serde_json::to_value(
        snapshot
            .records
            .iter()
            .map(|record| could_occur_in_year(record, year).unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn original_v1_query_matches_the_independently_written_reference() {
    let expected: Value = serde_json::from_str(EXPECTED_REPORT).unwrap();
    assert_eq!(expected["query_year"], 1705);
    assert_eq!(expected["round_trip_verified"], true);
    let snapshot = Snapshot::from_json(ORIGINAL_V1).unwrap();
    assert_eq!(actual_results(&snapshot, 1705), expected["results"]);
}

#[test]
fn saving_and_reloading_preserve_the_fixed_v1_reference_values() {
    let expected: Value = serde_json::from_str(EXPECTED_REPORT).unwrap();
    let input: Value = serde_json::from_str(ORIGINAL_V1).unwrap();
    let snapshot = Snapshot::from_json(ORIGINAL_V1).unwrap();
    let saved = snapshot.to_json().unwrap();
    assert_eq!(serde_json::from_str::<Value>(&saved).unwrap(), input);
    let restored = Snapshot::from_json(&saved).unwrap();
    assert_eq!(actual_results(&restored, 1705), expected["results"]);
}

#[test]
fn fixed_v1_source_fidelity_input_preserves_order_whitespace_and_domain_edges() {
    let original: Value = serde_json::from_str(SOURCE_FIDELITY_V1).unwrap();
    let snapshot = Snapshot::from_json(SOURCE_FIDELITY_V1).unwrap();
    let saved = snapshot.to_json().unwrap();
    assert_eq!(serde_json::from_str::<Value>(&saved).unwrap(), original);
    let restored = Snapshot::from_json(&saved).unwrap();
    let expected_at_one = json!([
        {
            "possible": true,
            "query_year": 1,
            "record_id": " boundary alternatives ",
            "interpretation_id": " unsorted alternatives ",
            "domain": "synthetic-years-v1",
            "rule_version": "could-occur-in-year-v1",
            "applied_rule": "alternative_membership",
            "temporal": {"kind": "exclusive_alternatives", "years": [9999, 1, 1700]},
            "source": {
                "source_id": " shared α ",
                "locator": " Blatt α\nZeile 2 ",
                "wording": "  ‘9999 oder 1 oder 1700’\r\n第二行\t  ",
                "origin": "synthetic"
            }
        },
        {
            "possible": true,
            "query_year": 1,
            "record_id": "singleton interval",
            "interpretation_id": "singleton bounds",
            "domain": "synthetic-years-v1",
            "rule_version": "could-occur-in-year-v1",
            "applied_rule": "inclusive_bounds",
            "temporal": {"kind": "inclusive_interval", "lower": 1, "upper": 1},
            "source": {
                "source_id": " shared α ",
                "locator": "Blatt β",
                "wording": "im synthetischen Jahr 1",
                "origin": "synthetic"
            }
        }
    ]);
    for state in [&snapshot, &restored] {
        assert_eq!(actual_results(state, 1), expected_at_one);
        for (year, expected) in [
            (2, [false, false]),
            (1700, [true, false]),
            (9998, [false, false]),
            (9999, [true, false]),
        ] {
            let actual = actual_results(state, year);
            assert_eq!(
                [
                    actual[0]["possible"].as_bool().unwrap(),
                    actual[1]["possible"].as_bool().unwrap()
                ],
                expected
            );
        }
    }
}
