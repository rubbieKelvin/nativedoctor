//! Multi-step flows: ordered request files sharing one [`crate::RuntimeEnv`] session.
//!
//! Sequence schema types are in [`crate::model`] (`SequenceFile`, `SequenceStep`).

use std::path::{Path, PathBuf};

use tracing::{debug, info};

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::execute::{
    execute_request_post_script, execute_request_with_env, ExecutionResult, OutcomePolicy,
    RunOptions,
};
use crate::model::{SequenceFile, SequenceStep};

/// Load and deserialize a sequence file; returns the document and the directory for resolving `steps[].file`.
pub fn load_sequence_file(path: &Path) -> Result<(SequenceFile, PathBuf)> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let text = std::fs::read_to_string(path)?;
    let file: SequenceFile = match ext.as_str() {
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
    debug!(
        path = %path.display(),
        format = %ext,
        steps = file.steps.len(),
        name = ?file.name,
        "loaded sequence file"
    );
    Ok((file, base))
}

/// Load a sequence file from `path`, then yield each [`SequenceStep`] in order.
///
/// Same parse rules and I/O as [`load_sequence_file`]; the returned iterator owns the steps and
/// walks them lazily. Step `file` paths remain relative to the sequence file’s directory.
pub fn sequence_step_iter(path: &Path) -> Result<impl Iterator<Item = (SequenceStep, PathBuf)>> {
    let (seq, basedir) = load_sequence_file(path)?;

    if seq.steps.is_empty() {
        return Err(Error::InvalidSequence(
            "sequence must contain at least one step".into(),
        ));
    }

    Ok(seq
        .steps
        .into_iter()
        .map(move |s| (s.clone(), basedir.join(&s.file))))
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
    /// Copied from the sequence file’s optional `name`.
    pub sequence_name: Option<String>,
    pub steps: Vec<StepSummary>,
}

/// Run each step in order with one shared [`RuntimeEnv`].
pub async fn execute_sequence(path: &Path, opts: &RunOptions) -> Result<SequenceResult> {
    let (seq, base_dir) = load_sequence_file(path)?;
    let sequence_name = seq.name.clone();

    if seq.steps.is_empty() {
        return Err(Error::InvalidSequence(
            "sequence must contain at least one step".into(),
        ));
    }

    let env = RuntimeEnv::from_process_env();
    let mut step_opts = opts.clone();
    step_opts.outcome_policy = OutcomePolicy::SequenceStep;

    let total = seq.steps.len();

    info!(
        path = %path.display(),
        name = ?sequence_name,
        steps = total,
        "sequence started"
    );

    let mut summaries = Vec::with_capacity(total);

    for (i, step) in seq.steps.iter().enumerate() {
        let step_path = base_dir.join(&step.file);

        if !step_path.is_file() {
            return Err(Error::SequenceStepNotFound(step_path));
        }

        debug!(
            step_index = i + 1,
            step_total = total,
            step_file = %step_path.display(),
            "sequence executing step"
        );

        let result = execute_request_with_env(&step_path, &step_opts, &env).await?;
        execute_request_post_script(&result, &step_opts, &env)?;

        summaries.push(StepSummary {
            index: i + 1,
            total,
            path: step_path,
            result,
        });
    }

    info!(
        path = %path.display(),
        steps_ok = summaries.len(),
        "sequence finished"
    );

    Ok(SequenceResult {
        sequence_name,
        steps: summaries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn sequence_step_iter_yields_steps_in_order() {
        let mut tmp = tempfile::NamedTempFile::with_suffix(".yaml").unwrap();
        writeln!(
            tmp,
            r#"version: "0.0.0"
steps:
  - file: a.yaml
  - file: b.yaml
"#
        )
        .unwrap();
        let steps: Vec<(SequenceStep, PathBuf)> = sequence_step_iter(tmp.path()).unwrap().collect();

        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].0.file, "a.yaml");
        assert_eq!(steps[1].0.file, "b.yaml");
    }
}
