// ProvenanceValidator — validates memory entry provenance and freshness.
//
// Checks: hash integrity, timestamp freshness, source authenticity,
// chain-of-custody validation.

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub content: String,
    pub source: String,
    pub timestamp: String,
    pub hash: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProvenanceValidatorConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Max entry age in seconds (default: 30 days = 2_592_000).
    #[serde(default = "default_max_age_secs")]
    pub max_age_secs: u64,
    /// Trusted sources (entries from these are pre-approved).
    #[serde(default)]
    pub trusted_sources: Vec<String>,
}

<<<<<<< HEAD
fn default_enabled() -> bool {
    true
}
fn default_max_age_secs() -> u64 {
    30 * 24 * 3600
}

impl Default for ProvenanceValidatorConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            max_age_secs: default_max_age_secs(),
            trusted_sources: vec!["docs".into(), "verified".into(), "official".into()],
        }
=======
fn default_enabled() -> bool { true }
fn default_max_age_secs() -> u64 { 30 * 24 * 3600 }

impl Default for ProvenanceValidatorConfig {
    fn default() -> Self {
        Self { enabled: default_enabled(), max_age_secs: default_max_age_secs(), trusted_sources: vec!["docs".into(), "verified".into(), "official".into()] }
>>>>>>> 4b60ced (docs: update README)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProvenanceVerdict {
    pub valid_count: usize,
    pub stale_count: usize,
    pub tampered_count: usize,
    pub untrusted_count: usize,
    pub risk_score: f64,
    pub summary: String,
}

pub struct ProvenanceValidator {
    config: ProvenanceValidatorConfig,
}

impl ProvenanceValidator {
    pub fn new(config: &ProvenanceValidatorConfig) -> Self {
<<<<<<< HEAD
        Self {
            config: config.clone(),
        }
=======
        Self { config: config.clone() }
>>>>>>> 4b60ced (docs: update README)
    }

    pub fn validate(&self, entries: &[MemoryEntry]) -> ProvenanceVerdict {
        if !self.config.enabled {
<<<<<<< HEAD
            return ProvenanceVerdict {
                valid_count: entries.len(),
                stale_count: 0,
                tampered_count: 0,
                untrusted_count: 0,
                risk_score: 0.0,
                summary: "provenance validator disabled".into(),
            };
=======
            return ProvenanceVerdict { valid_count: entries.len(), stale_count: 0, tampered_count: 0, untrusted_count: 0, risk_score: 0.0, summary: "provenance validator disabled".into() };
>>>>>>> 4b60ced (docs: update README)
        }

        let mut valid = 0usize;
        let mut stale = 0usize;
        let mut tampered = 0usize;
        let mut untrusted = 0usize;

        for entry in entries {
            let mut entry_valid = true;

            // Check timestamp freshness.
            if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(&entry.timestamp) {
                let age = chrono::Utc::now().timestamp() - ts.timestamp();
                if age > self.config.max_age_secs as i64 {
                    stale += 1;
                    entry_valid = false;
                }
            } else {
                // Cannot parse timestamp — treat as stale.
                stale += 1;
                entry_valid = false;
            }

            // Check hash integrity.
            if let Some(expected_hash) = &entry.hash {
                let actual_hash = format!("{:x}", Sha256::digest(entry.content.as_bytes()));
                if actual_hash != *expected_hash {
                    tampered += 1;
                    entry_valid = false;
                }
            }

            // Check source trust.
            let source_lower = entry.source.to_lowercase();
<<<<<<< HEAD
            if !self
                .config
                .trusted_sources
                .iter()
                .any(|t| source_lower.contains(&t.to_lowercase()))
                && source_lower != "docs"
                && source_lower != "verified"
            {
=======
            if !self.config.trusted_sources.iter().any(|t| source_lower.contains(&t.to_lowercase())) && source_lower != "docs" && source_lower != "verified" {
>>>>>>> 4b60ced (docs: update README)
                untrusted += 1;
                entry_valid = false;
            }

<<<<<<< HEAD
            if entry_valid {
                valid += 1;
            }
        }

        let total = entries.len().max(1);
        let risk_score = ((tampered as f64 * 10.0 + stale as f64 * 3.0 + untrusted as f64 * 2.0)
            / total as f64)
            .clamp(0.0, 10.0);

        let summary = if tampered > 0 {
            format!(
                "{} tampered, {} stale, {} untrusted of {} entries",
                tampered, stale, untrusted, total
            )
=======
            if entry_valid { valid += 1; }
        }

        let total = entries.len().max(1);
        let risk_score = ((tampered as f64 * 10.0 + stale as f64 * 3.0 + untrusted as f64 * 2.0) / total as f64).clamp(0.0, 10.0);

        let summary = if tampered > 0 {
            format!("{} tampered, {} stale, {} untrusted of {} entries", tampered, stale, untrusted, total)
>>>>>>> 4b60ced (docs: update README)
        } else if stale > 0 {
            format!("{} stale entries detected", stale)
        } else if untrusted > 0 {
            format!("{} entries from untrusted sources", untrusted)
        } else {
            "all entries have valid provenance".into()
        };

<<<<<<< HEAD
        ProvenanceVerdict {
            valid_count: valid,
            stale_count: stale,
            tampered_count: tampered,
            untrusted_count: untrusted,
            risk_score,
            summary,
        }
=======
        ProvenanceVerdict { valid_count: valid, stale_count: stale, tampered_count: tampered, untrusted_count: untrusted, risk_score, summary }
>>>>>>> 4b60ced (docs: update README)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_validator() -> ProvenanceValidator {
        ProvenanceValidator::new(&ProvenanceValidatorConfig::default())
    }

<<<<<<< HEAD
    fn make_entry(
        id: &str,
        content: &str,
        source: &str,
        ts: &str,
        hash: Option<String>,
    ) -> MemoryEntry {
        MemoryEntry {
            id: id.into(),
            content: content.into(),
            source: source.into(),
            timestamp: ts.into(),
            hash,
        }
=======
    fn make_entry(id: &str, content: &str, source: &str, ts: &str, hash: Option<String>) -> MemoryEntry {
        MemoryEntry { id: id.into(), content: content.into(), source: source.into(), timestamp: ts.into(), hash }
>>>>>>> 4b60ced (docs: update README)
    }

    fn compute_hash(content: &str) -> String {
        format!("{:x}", Sha256::digest(content.as_bytes()))
    }

    #[test]
    fn valid_entries_pass() {
        let v = default_validator();
        let hash = compute_hash("Hello world");
<<<<<<< HEAD
        let entries = vec![make_entry(
            "e1",
            "Hello world",
            "docs",
            "2026-07-28T12:00:00Z",
            Some(hash),
        )];
=======
        let entries = vec![make_entry("e1", "Hello world", "docs", "2026-07-28T12:00:00Z", Some(hash))];
>>>>>>> 4b60ced (docs: update README)
        let r = v.validate(&entries);
        assert_eq!(r.tampered_count, 0);
        assert_eq!(r.stale_count, 0);
    }

    #[test]
    fn tampered_hash_detected() {
        let v = default_validator();
<<<<<<< HEAD
        let entries = vec![make_entry(
            "e1",
            "Hello world",
            "docs",
            "2026-07-28T12:00:00Z",
            Some("wrong_hash".into()),
        )];
=======
        let entries = vec![make_entry("e1", "Hello world", "docs", "2026-07-28T12:00:00Z", Some("wrong_hash".into()))];
>>>>>>> 4b60ced (docs: update README)
        let r = v.validate(&entries);
        assert_eq!(r.tampered_count, 1);
        assert!(r.risk_score > 5.0);
    }

    #[test]
    fn stale_entry_detected() {
        let v = default_validator();
<<<<<<< HEAD
        let entries = vec![make_entry(
            "e1",
            "Old data",
            "docs",
            "2020-01-01T00:00:00Z",
            None,
        )];
=======
        let entries = vec![make_entry("e1", "Old data", "docs", "2020-01-01T00:00:00Z", None)];
>>>>>>> 4b60ced (docs: update README)
        let r = v.validate(&entries);
        assert_eq!(r.stale_count, 1);
    }

    #[test]
    fn untrusted_source_detected() {
        let v = default_validator();
<<<<<<< HEAD
        let entries = vec![make_entry(
            "e1",
            "Data",
            "unknown-malicious-source",
            "2026-07-28T12:00:00Z",
            None,
        )];
=======
        let entries = vec![make_entry("e1", "Data", "unknown-malicious-source", "2026-07-28T12:00:00Z", None)];
>>>>>>> 4b60ced (docs: update README)
        let r = v.validate(&entries);
        assert_eq!(r.untrusted_count, 1);
    }

    #[test]
    fn empty_entries_valid() {
        let v = default_validator();
        let r = v.validate(&[]);
        assert_eq!(r.valid_count, 0);
        assert_eq!(r.risk_score, 0.0);
    }

    #[test]
    fn disabled_skips_all() {
<<<<<<< HEAD
        let v = ProvenanceValidator::new(&ProvenanceValidatorConfig {
            enabled: false,
            ..Default::default()
        });
        let entries = vec![make_entry(
            "e1",
            "data",
            "unknown",
            "2020-01-01",
            Some("wrong".into()),
        )];
=======
        let v = ProvenanceValidator::new(&ProvenanceValidatorConfig { enabled: false, ..Default::default() });
        let entries = vec![make_entry("e1", "data", "unknown", "2020-01-01", Some("wrong".into()))];
>>>>>>> 4b60ced (docs: update README)
        let r = v.validate(&entries);
        assert_eq!(r.risk_score, 0.0);
    }
}
