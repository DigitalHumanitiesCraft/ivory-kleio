use ivory_kleio::{
    AppliedRule, Error, RULE_VERSION, SNAPSHOT_VERSION, Snapshot, SourceOrigin, TemporalValue,
    YEAR_DOMAIN, could_occur_in_year,
};
use serde_json::{Value, json};

const FIXTURE: &str = include_str!("../research-artefacts/cases/temporal-alternatives.json");

fn fixture() -> Snapshot {
    Snapshot::from_json(FIXTURE).expect("the written synthetic case must be valid")
}

fn invalid_json(text: &str) {
    assert!(
        matches!(Snapshot::from_json(text), Err(Error::InvalidJson(_))),
        "expected InvalidJson for {text}"
    );
}

#[test]
fn case_time_001_matches_the_written_oracle_and_exposes_its_evidence() {
    let snapshot = fixture();
    assert_eq!(snapshot.records.len(), 2);
    assert_eq!(SNAPSHOT_VERSION, 1);
    assert_eq!(YEAR_DOMAIN, "synthetic-years-v1");
    assert_eq!(RULE_VERSION, "could-occur-in-year-v1");
    for (year, answers) in [
        (1699, [false, false]),
        (1700, [true, true]),
        (1705, [false, true]),
        (1710, [true, true]),
        (1711, [false, false]),
    ] {
        for (index, record) in snapshot.records.iter().enumerate() {
            let result = could_occur_in_year(record, year).unwrap();
            assert_eq!(
                result.possible, answers[index],
                "year {year}, record {index}"
            );
            assert_eq!(result.query_year, year);
            assert_eq!(result.record_id, record.record_id);
            assert_eq!(
                result.interpretation_id,
                record.interpretation.interpretation_id
            );
            assert_eq!(result.domain, "synthetic-years-v1");
            assert_eq!(result.rule_version, "could-occur-in-year-v1");
            assert_eq!(result.source, record.source);
            assert_eq!(result.source.origin, SourceOrigin::Synthetic);
            assert_eq!(result.temporal, record.interpretation.temporal);
            assert_eq!(
                result.applied_rule,
                if index == 0 {
                    AppliedRule::AlternativeMembership
                } else {
                    AppliedRule::InclusiveBounds
                }
            );
            let encoded = serde_json::to_value(&result).unwrap();
            assert_eq!(
                encoded["applied_rule"],
                if index == 0 {
                    "alternative_membership"
                } else {
                    "inclusive_bounds"
                }
            );
        }
    }
}

#[test]
fn represented_possibilities_are_exact_over_the_entire_finite_domain() {
    let snapshot = fixture();
    let permitted: Vec<Vec<i32>> = snapshot
        .records
        .iter()
        .map(|record| {
            (1..=9999)
                .filter(|year| could_occur_in_year(record, *year).unwrap().possible)
                .collect()
        })
        .collect();
    assert_eq!(permitted[0], vec![1700, 1710]);
    assert_eq!(
        permitted[1],
        vec![
            1700, 1701, 1702, 1703, 1704, 1705, 1706, 1707, 1708, 1709, 1710
        ]
    );
}

#[test]
fn singleton_forms_permit_only_their_one_year_including_domain_endpoints() {
    let mut record = fixture().records.remove(0);
    for only_year in [1, 1700, 9999] {
        for temporal in [
            TemporalValue::ExclusiveAlternatives {
                years: vec![only_year],
            },
            TemporalValue::InclusiveInterval {
                lower: only_year,
                upper: only_year,
            },
        ] {
            record.interpretation.temporal = temporal;
            let permitted: Vec<i32> = (1..=9999)
                .filter(|year| could_occur_in_year(&record, *year).unwrap().possible)
                .collect();
            assert_eq!(permitted, vec![only_year]);
        }
    }
    record.interpretation.temporal = TemporalValue::InclusiveInterval {
        lower: 1,
        upper: 9999,
    };
    for year in 1..=9999 {
        assert!(could_occur_in_year(&record, year).unwrap().possible);
    }
}

#[test]
fn alternative_order_is_preserved_but_does_not_change_possibility() {
    let mut snapshot = fixture();
    let original = snapshot.records[0].clone();
    snapshot.records[0].interpretation.temporal = TemporalValue::ExclusiveAlternatives {
        years: vec![1710, 1700],
    };
    let restored = Snapshot::from_json(&snapshot.to_json().unwrap()).unwrap();
    assert_eq!(snapshot, restored);
    for year in 1..=9999 {
        assert_eq!(
            could_occur_in_year(&original, year).unwrap().possible,
            could_occur_in_year(&restored.records[0], year)
                .unwrap()
                .possible
        );
    }
}

#[test]
fn snapshot_round_trip_preserves_complete_results_and_exact_source_text() {
    let mut snapshot = fixture();
    snapshot.records[0].source.wording =
        "  Überlieferung: ‘1700 oder 1710’\n第二行\r\n𐀀\t  ".into();
    snapshot.records[0].source.locator = " Blatt α\nZeile 2 ".into();
    snapshot.records[0].source.source_id = " Quelle ä ".into();
    snapshot.records[0].record_id = " Ereignis A ".into();
    snapshot.records[0].interpretation.interpretation_id = " Deutung α ".into();
    let restored = Snapshot::from_json(&snapshot.to_json().unwrap()).unwrap();
    assert_eq!(restored, snapshot);
    for (before, after) in snapshot.records.iter().zip(&restored.records) {
        for year in [1, 1699, 1700, 1705, 1710, 1711, 9999] {
            assert_eq!(
                could_occur_in_year(before, year).unwrap(),
                could_occur_in_year(after, year).unwrap()
            );
        }
    }
}

#[test]
fn empty_snapshots_and_shared_source_ids_are_valid() {
    let empty = Snapshot::from_json(r#"{"format_version":1,"records":[]}"#).unwrap();
    assert!(empty.records.is_empty());
    assert_eq!(
        Snapshot::from_json(&empty.to_json().unwrap()).unwrap(),
        empty
    );
    let snapshot = fixture();
    assert_eq!(
        snapshot.records[0].source.source_id,
        snapshot.records[1].source.source_id
    );
    snapshot.validate().unwrap();
}

#[test]
fn invalid_query_years_are_errors_instead_of_negative_answers() {
    for record in fixture().records {
        for year in [i32::MIN, -1, 0, 10000, i32::MAX] {
            assert!(
                matches!(could_occur_in_year(&record, year), Err(Error::InvalidYear { year: actual }) if actual == year)
            );
        }
    }
}

#[test]
fn invalid_temporal_values_fail_loading_evaluation_and_serialization() {
    let cases = [
        (
            TemporalValue::ExclusiveAlternatives { years: vec![] },
            "empty",
        ),
        (
            TemporalValue::ExclusiveAlternatives {
                years: vec![1700, 1700],
            },
            "duplicate",
        ),
        (
            TemporalValue::ExclusiveAlternatives {
                years: vec![0, 1700],
            },
            "year",
        ),
        (
            TemporalValue::ExclusiveAlternatives {
                years: vec![-1, 1700],
            },
            "year",
        ),
        (
            TemporalValue::ExclusiveAlternatives {
                years: vec![1700, 10000],
            },
            "year",
        ),
        (
            TemporalValue::InclusiveInterval {
                lower: 1710,
                upper: 1700,
            },
            "reversed",
        ),
        (
            TemporalValue::InclusiveInterval {
                lower: 0,
                upper: 1700,
            },
            "year",
        ),
        (
            TemporalValue::InclusiveInterval {
                lower: 1700,
                upper: 10000,
            },
            "year",
        ),
    ];
    for (temporal, expected) in cases {
        let mut snapshot = fixture();
        snapshot.records[0].interpretation.temporal = temporal;
        let raw = serde_json::to_string(&snapshot).unwrap();
        let errors = [
            snapshot.validate().unwrap_err(),
            snapshot.records[0].validate().unwrap_err(),
            could_occur_in_year(&snapshot.records[0], 1705).unwrap_err(),
            snapshot.to_json().unwrap_err(),
            Snapshot::from_json(&raw).unwrap_err(),
        ];
        for error in errors {
            assert!(
                matches!(
                    (&error, expected),
                    (Error::EmptyAlternatives, "empty")
                        | (Error::DuplicateAlternative { .. }, "duplicate")
                        | (Error::InvalidYear { .. }, "year")
                        | (Error::ReversedInterval { .. }, "reversed")
                ),
                "{error:?}: {expected}"
            );
        }
    }
}

#[test]
fn invalid_domain_and_version_are_not_silently_upgraded() {
    let mut snapshot = fixture();
    snapshot.records[0].domain = "historical-calendar".into();
    assert!(matches!(
        could_occur_in_year(&snapshot.records[0], 1700),
        Err(Error::UnsupportedDomain { .. })
    ));
    assert!(matches!(
        snapshot.to_json(),
        Err(Error::UnsupportedDomain { .. })
    ));
    assert!(matches!(
        Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
        Err(Error::UnsupportedDomain { .. })
    ));
    for version in [0, 2, u32::MAX] {
        let mut snapshot = fixture();
        snapshot.format_version = version;
        assert!(
            matches!(snapshot.validate(), Err(Error::UnsupportedSnapshotVersion { version: actual }) if actual == version)
        );
        assert!(matches!(
            snapshot.to_json(),
            Err(Error::UnsupportedSnapshotVersion { .. })
        ));
        assert!(matches!(
            Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
            Err(Error::UnsupportedSnapshotVersion { .. })
        ));
    }
}

#[test]
fn blank_identifiers_and_source_fields_fail_every_entry_point() {
    for field in [
        "record_id",
        "source_id",
        "locator",
        "wording",
        "interpretation_id",
    ] {
        for blank in ["", " \t\r\n", "\u{2003}"] {
            let mut snapshot = fixture();
            let record = &mut snapshot.records[0];
            match field {
                "record_id" => record.record_id = blank.into(),
                "source_id" => record.source.source_id = blank.into(),
                "locator" => record.source.locator = blank.into(),
                "wording" => record.source.wording = blank.into(),
                "interpretation_id" => record.interpretation.interpretation_id = blank.into(),
                _ => unreachable!(),
            }
            assert!(
                matches!(record.validate(), Err(Error::BlankField { .. })),
                "{field}"
            );
            assert!(matches!(
                could_occur_in_year(record, 1700),
                Err(Error::BlankField { .. })
            ));
            assert!(matches!(snapshot.to_json(), Err(Error::BlankField { .. })));
            assert!(matches!(
                Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
                Err(Error::BlankField { .. })
            ));
        }
    }
}

#[test]
fn snapshot_identity_collisions_fail_loading_and_saving() {
    let mut snapshot = fixture();
    snapshot.records[1].record_id = snapshot.records[0].record_id.clone();
    assert!(matches!(
        snapshot.to_json(),
        Err(Error::DuplicateRecordId { .. })
    ));
    assert!(matches!(
        Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
        Err(Error::DuplicateRecordId { .. })
    ));
    let mut snapshot = fixture();
    snapshot.records[1].interpretation.interpretation_id =
        snapshot.records[0].interpretation.interpretation_id.clone();
    assert!(matches!(
        snapshot.to_json(),
        Err(Error::DuplicateInterpretationId { .. })
    ));
    assert!(matches!(
        Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
        Err(Error::DuplicateInterpretationId { .. })
    ));
}

#[test]
fn loading_and_saving_validate_the_entire_collection() {
    let mut snapshot = fixture();
    snapshot.records[1].interpretation.temporal = TemporalValue::InclusiveInterval {
        lower: 1710,
        upper: 1700,
    };
    assert!(matches!(
        snapshot.to_json(),
        Err(Error::ReversedInterval { .. })
    ));
    assert!(matches!(
        Snapshot::from_json(&serde_json::to_string(&snapshot).unwrap()),
        Err(Error::ReversedInterval { .. })
    ));
}

#[test]
fn every_object_requires_its_fields_and_rejects_unknown_fields() {
    let baseline: Value = serde_json::from_str(FIXTURE).unwrap();
    for path in [
        "",
        "/records/0",
        "/records/0/source",
        "/records/0/interpretation",
        "/records/0/interpretation/temporal",
        "/records/1/interpretation/temporal",
    ] {
        let keys: Vec<String> = baseline
            .pointer(path)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        for key in keys {
            let mut value = baseline.clone();
            value
                .pointer_mut(path)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(&key);
            invalid_json(&value.to_string());
        }
        let mut value = baseline.clone();
        value
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".into(), json!(true));
        invalid_json(&value.to_string());
    }
}

#[test]
fn unsupported_forms_origins_and_wrong_types_are_invalid_json() {
    let baseline: Value = serde_json::from_str(FIXTURE).unwrap();
    for (path, wrong) in [
        ("/format_version", json!("1")),
        ("/format_version", json!(-1)),
        ("/records", json!({})),
        ("/records/0", Value::Null),
        ("/records/0/record_id", json!(42)),
        ("/records/0/domain", Value::Null),
        ("/records/0/source", json!([])),
        ("/records/0/source/source_id", json!(false)),
        ("/records/0/source/locator", json!(1)),
        ("/records/0/source/wording", json!([])),
        ("/records/0/source/origin", json!("historical")),
        ("/records/0/source/origin", json!({"synthetic": null})),
        ("/records/0/interpretation", Value::Null),
        ("/records/0/interpretation/interpretation_id", json!(1)),
        ("/records/0/interpretation/temporal", json!("1700")),
        ("/records/0/interpretation/temporal/kind", json!("unknown")),
        ("/records/0/interpretation/temporal/years", json!([1700.0])),
        ("/records/0/interpretation/temporal/years", json!(["1700"])),
        (
            "/records/0/interpretation/temporal/years",
            json!([2147483648_i64]),
        ),
        (
            "/records/0/interpretation/temporal/years",
            json!([-2147483649_i64]),
        ),
        ("/records/0/interpretation/temporal/years", json!(1700)),
        ("/records/1/interpretation/temporal/lower", json!(1700.5)),
        (
            "/records/1/interpretation/temporal/lower",
            json!(-2147483649_i64),
        ),
        (
            "/records/1/interpretation/temporal/upper",
            json!(2147483648_i64),
        ),
        ("/records/1/interpretation/temporal/upper", json!("1710")),
    ] {
        let mut value = baseline.clone();
        *value.pointer_mut(path).unwrap() = wrong;
        invalid_json(&value.to_string());
    }
    for malformed in [
        "",
        "{",
        "null",
        "[]",
        "{} {}",
        "{\"format_version\":1,\"records\":[],}",
    ] {
        invalid_json(malformed);
    }
}

#[test]
fn duplicate_json_keys_are_rejected_at_every_object_level() {
    for (key, value) in [
        ("format_version", "1"),
        ("records", "[]"),
        ("record_id", "\"duplicate\""),
        ("domain", "\"synthetic-years-v1\""),
        ("source", "{}"),
        ("source_id", "\"duplicate\""),
        ("locator", "\"duplicate\""),
        ("wording", "\"duplicate\""),
        ("origin", "\"synthetic\""),
        ("interpretation", "{}"),
        ("interpretation_id", "\"duplicate\""),
        ("temporal", "{}"),
        ("kind", "\"exclusive_alternatives\""),
        ("years", "[1700,1710]"),
        ("lower", "1700"),
        ("upper", "1710"),
    ] {
        let needle = format!("\"{key}\":");
        let replacement = format!("\"{key}\":{value},\"{key}\":");
        invalid_json(&FIXTURE.replacen(&needle, &replacement, 1));
    }
    let interval_kind = FIXTURE.replace(
        "\"kind\": \"inclusive_interval\"",
        "\"kind\": \"inclusive_interval\", \"kind\": \"inclusive_interval\"",
    );
    invalid_json(&interval_kind);
}

#[test]
fn positional_arrays_cannot_replace_contract_objects() {
    invalid_json("[1,[]]");
    let baseline: Value = serde_json::from_str(FIXTURE).unwrap();
    let record = &baseline["records"][0];
    for (path, positional) in [
        (
            "/records/0",
            json!([
                record["record_id"],
                record["domain"],
                record["source"],
                record["interpretation"]
            ]),
        ),
        (
            "/records/0/source",
            json!([
                record["source"]["source_id"],
                record["source"]["locator"],
                record["source"]["wording"],
                "synthetic"
            ]),
        ),
        (
            "/records/0/interpretation",
            json!([
                record["interpretation"]["interpretation_id"],
                record["interpretation"]["temporal"]
            ]),
        ),
        (
            "/records/0/interpretation/temporal",
            json!(["exclusive_alternatives", [1700, 1710]]),
        ),
        (
            "/records/1/interpretation/temporal",
            json!(["inclusive_interval", 1700, 1710]),
        ),
    ] {
        let mut value = baseline.clone();
        *value.pointer_mut(path).unwrap() = positional;
        invalid_json(&value.to_string());
    }
}
