//! Multi-step flows: ordered request files sharing one [`crate::RuntimeEnv`] session.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::execute::{
    execute_request_with_env, OutcomePolicy, ExecutionResult, RunOptions,
};

fn default_version() -> u32 {
    1
}

/// One entry in a sequence file: path to a request definition, relative to the sequence file dir.
#[derive(Debug, Clone, Deserialize)]
pub struct SequenceStep {
    pub file: String,
}

/// Sequence document (JSON or YAML).
#[derive(Debug, Clone, Deserialize)]
pub struct SequenceFile {
    #[serde(default = "default_version")]
    pub version: u32,
    pub steps: Vec<SequenceStep>,
}

/// Load and deserialize a sequence file; returns the document and the directory for resolving `steps[].file`.
pub fn load_sequence_file(path: &Path) -> Result<(SequenceFile, PathBuf)> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let text = std::fs::read_to_string(path)?;
    let file = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&text).map_err(|e| Error::ParseSequenceYaml {
            path: path.to_path_buf(),
            source: e,
        })?,
        "json" => serde_json::from_str(&text).map_err(|e| Error::ParseSequenceJson {
            path: path.to_path_buf(),
            source: e,
        })?,
        _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
    };
    let base = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    Ok((file, base))
}

/// Completed step metadata for CLI / callers.
#[derive(Debug, Clone)]
pub struct StepSummary {
    pub index: usize,
    pub total: usize,
    pub path: PathBuf,
    pub result: ExecutionResult,
}

/// Outcome after all steps succeed.
#[derive(Debug, Clone)]
pub struct SequenceResult {
    pub steps: Vec<StepSummary>,
}

/// Run each step in order with one shared [`RuntimeEnv`]. Uses [`OutcomePolicy::SequenceStep`].
pub async fn execute_sequence(path: &Path, opts: &RunOptions) -> Result<SequenceResult> {
    let (seq, base_dir) = load_sequence_file(path)?;
    if seq.steps.is_empty() {
        return Err(Error::InvalidSequence(
            "sequence must contain at least one step".into(),
        ));
    }

    let env = RuntimeEnv::from_process_env();
    let mut step_opts = opts.clone();
    step_opts.outcome_policy = OutcomePolicy::SequenceStep;

    let total = seq.steps.len();
    let mut summaries = Vec::with_capacity(total);

    for (i, step) in seq.steps.iter().enumerate() {
        let step_path = base_dir.join(&step.file);
        if !step_path.is_file() {
            return Err(Error::SequenceStepNotFound(step_path));
        }
        let result = execute_request_with_env(&step_path, &step_opts, &env).await?;
        summaries.push(StepSummary {
            index: i + 1,
            total,
            path: step_path,
            result,
        });
    }

    Ok(SequenceResult { steps: summaries })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_sequence_yaml() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("seq.yaml");
        std::fs::write(
            &p,
            b"version: 1\nsteps:\n  - file: a.yaml\n  - file: b.yaml\n",
        )
        .unwrap();
        let (s, base) = load_sequence_file(&p).unwrap();
        assert_eq!(s.steps.len(), 2);
        assert_eq!(s.steps[0].file, "a.yaml");
        assert_eq!(base, dir.path());
    }
}
