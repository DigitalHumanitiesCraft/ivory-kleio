use std::collections::BTreeSet;

use ivory_kleio::{
    AppliedRule, Interpretation, Record, Source, SourceOrigin, TemporalValue, could_occur_in_year,
};

const WINDOW_STARTS: [i32; 3] = [1, 1700, 9992];

fn record(temporal: TemporalValue) -> Record {
    Record {
        record_id: "enumerated event".into(),
        domain: "synthetic-years-v1".into(),
        source: Source {
            source_id: "synthetic-model-space".into(),
            locator: " window α\n ".into(),
            wording: "  Synthetic model: ü, 第二行\n  ".into(),
            origin: SourceOrigin::Synthetic,
        },
        interpretation: Interpretation {
            interpretation_id: "enumerated interpretation".into(),
            temporal,
        },
    }
}

fn queries(start: i32) -> Vec<i32> {
    let mut years: BTreeSet<i32> = (start - 1..=start + 8)
        .filter(|year| (1..=9999).contains(year))
        .collect();
    years.extend([1, 9999]);
    years.into_iter().collect()
}

fn check_evidence(record: &Record, year: i32, possible: bool, rule: AppliedRule) {
    let actual = could_occur_in_year(record, year).unwrap();
    assert_eq!(
        actual.possible, possible,
        "year {year}: {:?}",
        record.interpretation.temporal
    );
    assert_eq!(actual.query_year, year);
    assert_eq!(actual.record_id, "enumerated event");
    assert_eq!(actual.interpretation_id, "enumerated interpretation");
    assert_eq!(actual.source, record.source);
    assert_eq!(actual.domain, "synthetic-years-v1");
    assert_eq!(actual.rule_version, "could-occur-in-year-v1");
    assert_eq!(actual.applied_rule, rule);
    assert_eq!(actual.temporal, record.interpretation.temporal);
}

#[test]
fn every_nonempty_subset_matches_its_bitmask_in_three_eight_year_windows() {
    for start in WINDOW_STARTS {
        for mask in 1_u16..=255 {
            let years: Vec<i32> = (0..8)
                .filter(|offset| mask & (1 << offset) != 0)
                .map(|offset| start + offset)
                .collect();
            let mut reversed = years.clone();
            reversed.reverse();
            let mut rotated = years.clone();
            rotated.rotate_left(1);
            for ordered in [years, reversed, rotated] {
                let record = record(TemporalValue::ExclusiveAlternatives { years: ordered });
                for query in queries(start) {
                    let offset = query - start;
                    let expected = (0..8).contains(&offset) && mask & (1 << offset) != 0;
                    check_evidence(&record, query, expected, AppliedRule::AlternativeMembership);
                }
            }
        }
    }
}

#[test]
fn every_interval_matches_an_independently_materialized_set() {
    for start in WINDOW_STARTS {
        let window: Vec<i32> = (start..start + 8).collect();
        for first in 0..8 {
            for last in first..8 {
                let possible: BTreeSet<i32> = window[first..=last].iter().copied().collect();
                let record = record(TemporalValue::InclusiveInterval {
                    lower: window[first],
                    upper: window[last],
                });
                for query in queries(start) {
                    check_evidence(
                        &record,
                        query,
                        possible.contains(&query),
                        AppliedRule::InclusiveBounds,
                    );
                }
            }
        }
    }
}

#[test]
fn widening_any_interval_within_a_window_cannot_remove_a_possible_year() {
    for start in WINDOW_STARTS {
        let window: Vec<i32> = (start..start + 8).collect();
        let mut models = Vec::new();
        for first in 0..8 {
            for last in first..8 {
                let expected: BTreeSet<i32> = window[first..=last].iter().copied().collect();
                let record = record(TemporalValue::InclusiveInterval {
                    lower: window[first],
                    upper: window[last],
                });
                let actual: BTreeSet<i32> = queries(start)
                    .into_iter()
                    .filter(|year| could_occur_in_year(&record, *year).unwrap().possible)
                    .collect();
                assert_eq!(actual, expected);
                models.push((expected, actual));
            }
        }
        for (narrow_expected, narrow_actual) in &models {
            for (wide_expected, wide_actual) in &models {
                if narrow_expected.is_subset(wide_expected) {
                    assert!(narrow_actual.is_subset(wide_actual));
                }
            }
        }
    }
}

#[test]
fn equal_possibility_sets_keep_distinct_interpretation_forms() {
    for start in WINDOW_STARTS {
        let window: Vec<i32> = (start..start + 8).collect();
        for first in 0..8 {
            for last in first..8 {
                let alternatives = record(TemporalValue::ExclusiveAlternatives {
                    years: window[first..=last].to_vec(),
                });
                let interval = record(TemporalValue::InclusiveInterval {
                    lower: window[first],
                    upper: window[last],
                });
                for query in queries(start) {
                    let alternatives_result = could_occur_in_year(&alternatives, query).unwrap();
                    let interval_result = could_occur_in_year(&interval, query).unwrap();
                    assert_eq!(alternatives_result.possible, interval_result.possible);
                    assert_ne!(alternatives_result.temporal, interval_result.temporal);
                    assert_ne!(
                        alternatives_result.applied_rule,
                        interval_result.applied_rule
                    );
                    assert_ne!(
                        serde_json::to_value(alternatives_result).unwrap()["temporal"]["kind"],
                        serde_json::to_value(interval_result).unwrap()["temporal"]["kind"]
                    );
                }
            }
        }
    }
}
