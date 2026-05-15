use std::path::{Path, PathBuf};

use anyhow::Context;
use skill_archiver::{BuiltArtifact, build_archive, clean_dist};
use skill_parser::{ParseError, ParsedSkill, parse_skill};
use skill_validator::{ValidationReport, validate};
use tokio::task::JoinSet;

pub struct BuildOutcome {
    pub artifacts: Vec<BuiltArtifact>,
    pub had_errors: bool,
}

pub async fn scan_and_validate(skills_dir: &Path) -> anyhow::Result<(Vec<ParsedSkill>, bool)> {
    let mut entries = tokio::fs::read_dir(skills_dir)
        .await
        .with_context(|| format!("reading {}", skills_dir.display()))?;

    let mut dirs: Vec<PathBuf> = Vec::new();
    while let Some(e) = entries.next_entry().await? {
        if e.file_type().await?.is_dir() {
            dirs.push(e.path());
        }
    }
    dirs.sort();

    let mut valid: Vec<ParsedSkill> = Vec::new();
    let mut parse_errors: Vec<(PathBuf, ParseError)> = Vec::new();
    let mut validation_reports: Vec<ValidationReport> = Vec::new();

    for dir in &dirs {
        match parse_skill(dir).await {
            Ok(parsed) => {
                let report = validate(&parsed);
                if report.is_ok() {
                    valid.push(parsed);
                } else {
                    validation_reports.push(report);
                }
            }
            Err(ParseError::MissingSkillMd(_)) => {
                tracing::debug!(path = %dir.display(), "skipping directory without SKILL.md");
            }
            Err(e) => parse_errors.push((dir.clone(), e)),
        }
    }

    let had_errors = !parse_errors.is_empty() || !validation_reports.is_empty();

    tracing::info!(
        total = dirs.len(),
        valid = valid.len(),
        parse_failures = parse_errors.len(),
        validation_failures = validation_reports.len(),
        "skill scan complete"
    );

    for (path, err) in &parse_errors {
        tracing::error!(skill = %path.display(), "parse error: {err}");
    }
    for r in &validation_reports {
        tracing::error!("{r}");
    }

    Ok((valid, had_errors))
}

pub async fn build(skills_dir: &Path, dist_dir: &Path) -> anyhow::Result<BuildOutcome> {
    let (valid, had_errors) = scan_and_validate(skills_dir).await?;

    if had_errors {
        // Skip clean_dist and archiving entirely so a failing run does not wipe a
        // previously-valid `dist/`. Callers (e.g. `build`/`upload` subcommands)
        // see `had_errors == true` and exit non-zero.
        return Ok(BuildOutcome {
            artifacts: Vec::new(),
            had_errors,
        });
    }

    if valid.is_empty() {
        // No skills to archive — don't wipe a previously-good `dist/` on a scan that
        // simply found nothing.
        tracing::warn!(skills_dir = %skills_dir.display(), "no valid skills found; leaving dist/ untouched");
        return Ok(BuildOutcome {
            artifacts: Vec::new(),
            had_errors,
        });
    }

    clean_dist(dist_dir)
        .await
        .context("cleaning dist directory")?;

    let mut tasks: JoinSet<anyhow::Result<BuiltArtifact>> = JoinSet::new();
    let expected = valid.len();
    for skill in valid {
        let dist = dist_dir.to_path_buf();
        tasks.spawn(async move {
            let dir_name = skill.dir_name.clone();
            let art = build_archive(&skill, &dist)
                .await
                .with_context(|| format!("building archive for {dir_name}"))?;
            tracing::info!(file = %art.file_name, "built artifact");
            Ok(art)
        });
    }

    let mut artifacts = Vec::with_capacity(expected);
    let mut errors: Vec<anyhow::Error> = Vec::new();
    while let Some(joined) = tasks.join_next().await {
        match joined {
            Ok(Ok(art)) => artifacts.push(art),
            Ok(Err(e)) => errors.push(e),
            Err(join_err) => errors.push(anyhow::anyhow!("archive task join failed: {join_err}")),
        }
    }

    if !errors.is_empty() {
        for e in &errors {
            tracing::error!("archive build error: {e:#}");
        }
        let first = errors.remove(0);
        return Err(first.context(format!(
            "{} archive task(s) failed (see preceding logs for details)",
            errors.len() + 1
        )));
    }

    artifacts.sort_by(|a, b| a.file_name.cmp(&b.file_name));

    Ok(BuildOutcome {
        artifacts,
        had_errors,
    })
}
