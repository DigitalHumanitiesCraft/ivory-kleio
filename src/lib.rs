#![forbid(unsafe_code)]

//! Synthetic year interpretations and their declared possibility query.
//!
//! Public input types and their serde traits are raw representations. Use
//! [`Snapshot::from_json`] and [`Snapshot::to_json`] for validated persistence;
//! evaluation also validates directly constructed or mutated records.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const SNAPSHOT_VERSION: u32 = 1;
pub const YEAR_DOMAIN: &str = "synthetic-years-v1";
pub const RULE_VERSION: &str = "could-occur-in-year-v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub source_id: String,
    pub locator: String,
    pub wording: String,
    pub origin: SourceOrigin,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", try_from = "String")]
pub enum SourceOrigin {
    Synthetic,
}

impl TryFrom<String> for SourceOrigin {
    type Error = serde::de::value::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "synthetic" => Ok(Self::Synthetic),
            _ => Err(serde::de::Error::unknown_variant(&value, &["synthetic"])),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TemporalValue {
    ExclusiveAlternatives { years: Vec<i32> },
    InclusiveInterval { lower: i32, upper: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interpretation {
    pub interpretation_id: String,
    pub temporal: TemporalValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub record_id: String,
    pub domain: String,
    pub source: Source,
    pub interpretation: Interpretation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub format_version: u32,
    pub records: Vec<Record>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppliedRule {
    AlternativeMembership,
    InclusiveBounds,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct QueryResult {
    pub possible: bool,
    pub query_year: i32,
    pub record_id: String,
    pub interpretation_id: String,
    pub domain: String,
    pub rule_version: &'static str,
    pub applied_rule: AppliedRule,
    pub temporal: TemporalValue,
    pub source: Source,
}

#[derive(Debug)]
pub enum Error {
    InvalidYear { year: i32 },
    EmptyAlternatives,
    DuplicateAlternative { year: i32 },
    ReversedInterval { lower: i32, upper: i32 },
    UnsupportedDomain { domain: String },
    UnsupportedSnapshotVersion { version: u32 },
    BlankField { field: &'static str },
    DuplicateRecordId { record_id: String },
    DuplicateInterpretationId { interpretation_id: String },
    InvalidJson(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidYear { year } => {
                write!(f, "InvalidYear: {year} is outside 1 through 9999")
            }
            Self::EmptyAlternatives => {
                write!(f, "EmptyAlternatives: at least one year is required")
            }
            Self::DuplicateAlternative { year } => {
                write!(f, "DuplicateAlternative: year {year} occurs more than once")
            }
            Self::ReversedInterval { lower, upper } => {
                write!(
                    f,
                    "ReversedInterval: lower bound {lower} exceeds upper bound {upper}"
                )
            }
            Self::UnsupportedDomain { domain } => {
                write!(f, "UnsupportedDomain: {domain:?}; expected {YEAR_DOMAIN:?}")
            }
            Self::UnsupportedSnapshotVersion { version } => write!(
                f,
                "UnsupportedSnapshotVersion: {version}; expected {SNAPSHOT_VERSION}"
            ),
            Self::BlankField { field } => write!(f, "BlankField: {field} must not be blank"),
            Self::DuplicateRecordId { record_id } => {
                write!(f, "DuplicateRecordId: {record_id:?}")
            }
            Self::DuplicateInterpretationId { interpretation_id } => {
                write!(f, "DuplicateInterpretationId: {interpretation_id:?}")
            }
            Self::InvalidJson(error) => write!(f, "InvalidJson: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidJson(error) => Some(error),
            _ => None,
        }
    }
}

/// Checks a label against the complete synthetic year domain.
pub fn validate_year(year: i32) -> Result<(), Error> {
    if (1..=9999).contains(&year) {
        Ok(())
    } else {
        Err(Error::InvalidYear { year })
    }
}

fn nonblank(value: &str, field: &'static str) -> Result<(), Error> {
    if value.trim().is_empty() {
        Err(Error::BlankField { field })
    } else {
        Ok(())
    }
}

impl Record {
    pub fn validate(&self) -> Result<(), Error> {
        nonblank(&self.record_id, "record_id")?;
        nonblank(&self.source.source_id, "source.source_id")?;
        nonblank(&self.source.locator, "source.locator")?;
        nonblank(&self.source.wording, "source.wording")?;
        nonblank(
            &self.interpretation.interpretation_id,
            "interpretation.interpretation_id",
        )?;
        if self.domain != YEAR_DOMAIN {
            return Err(Error::UnsupportedDomain {
                domain: self.domain.clone(),
            });
        }
        match &self.interpretation.temporal {
            TemporalValue::ExclusiveAlternatives { years } => {
                if years.is_empty() {
                    return Err(Error::EmptyAlternatives);
                }
                let mut seen = HashSet::new();
                for &year in years {
                    validate_year(year)?;
                    if !seen.insert(year) {
                        return Err(Error::DuplicateAlternative { year });
                    }
                }
            }
            TemporalValue::InclusiveInterval { lower, upper } => {
                validate_year(*lower)?;
                validate_year(*upper)?;
                if lower > upper {
                    return Err(Error::ReversedInterval {
                        lower: *lower,
                        upper: *upper,
                    });
                }
            }
        }
        Ok(())
    }
}

impl Snapshot {
    pub fn validate(&self) -> Result<(), Error> {
        if self.format_version != SNAPSHOT_VERSION {
            return Err(Error::UnsupportedSnapshotVersion {
                version: self.format_version,
            });
        }
        let mut record_ids = HashSet::new();
        let mut interpretation_ids = HashSet::new();
        for record in &self.records {
            record.validate()?;
            if !record_ids.insert(&record.record_id) {
                return Err(Error::DuplicateRecordId {
                    record_id: record.record_id.clone(),
                });
            }
            if !interpretation_ids.insert(&record.interpretation.interpretation_id) {
                return Err(Error::DuplicateInterpretationId {
                    interpretation_id: record.interpretation.interpretation_id.clone(),
                });
            }
        }
        Ok(())
    }

    /// Loads the object-only JSON contract and validates the complete snapshot.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        // Decode the original text so duplicate fields retain serde diagnostics.
        let snapshot: Self = serde_json::from_str(json).map_err(Error::InvalidJson)?;
        let shape: serde_json::Value = serde_json::from_str(json).map_err(Error::InvalidJson)?;
        require_object(&shape, "snapshot")?;
        if let Some(records) = shape["records"].as_array() {
            for (index, record) in records.iter().enumerate() {
                let path = format!("records[{index}]");
                require_object(record, &path)?;
                require_object(&record["source"], &format!("{path}.source"))?;
                require_object(&record["interpretation"], &format!("{path}.interpretation"))?;
                require_object(
                    &record["interpretation"]["temporal"],
                    &format!("{path}.interpretation.temporal"),
                )?;
            }
        }
        snapshot.validate()?;
        Ok(snapshot)
    }

    /// Validates before serialization; direct serde serialization is unvalidated.
    pub fn to_json(&self) -> Result<String, Error> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(Error::InvalidJson)
    }
}

fn require_object(value: &serde_json::Value, path: &str) -> Result<(), Error> {
    if value.is_object() {
        Ok(())
    } else {
        Err(Error::InvalidJson(serde::de::Error::custom(format!(
            "expected JSON object at {path}"
        ))))
    }
}

/// Reports possibility under the selected interpretation, with its full evidence.
pub fn could_occur_in_year(record: &Record, year: i32) -> Result<QueryResult, Error> {
    record.validate()?;
    validate_year(year)?;
    let (possible, applied_rule) = match &record.interpretation.temporal {
        TemporalValue::ExclusiveAlternatives { years } => {
            (years.contains(&year), AppliedRule::AlternativeMembership)
        }
        TemporalValue::InclusiveInterval { lower, upper } => (
            (*lower..=*upper).contains(&year),
            AppliedRule::InclusiveBounds,
        ),
    };
    Ok(QueryResult {
        possible,
        query_year: year,
        record_id: record.record_id.clone(),
        interpretation_id: record.interpretation.interpretation_id.clone(),
        domain: record.domain.clone(),
        rule_version: RULE_VERSION,
        applied_rule,
        temporal: record.interpretation.temporal.clone(),
        source: record.source.clone(),
    })
}
