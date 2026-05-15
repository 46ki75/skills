use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::pipeline::{self, BuildOutcome};

#[derive(Parser, Debug)]
#[command(
    name = "skill_uploader",
    about = "Validate, archive, and publish Agent Skills"
)]
pub struct Cli {
    /// Path to the skills directory (defaults to ./skills).
    #[arg(long, default_value = "skills")]
    pub skills_dir: PathBuf,

    /// Path to the output directory for ZIP archives (defaults to ./dist).
    #[arg(long, default_value = "dist")]
    pub dist_dir: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Parse and validate every skill. Exit non-zero on any failure.
    Check,
    /// Validate every skill, then write ZIP archives into the dist directory.
    Build,
    /// Build artifacts and upload any whose <name>-v<version> release tag does not yet exist.
    Upload {
        /// GitHub repository in "owner/name" form. Defaults to $GITHUB_REPOSITORY or the origin remote.
        #[arg(long)]
        repo: Option<String>,
        /// Skip the actual upload — useful for verifying which artifacts would be published.
        #[arg(long)]
        dry_run: bool,
    },
}

pub async fn run(args: Cli) -> anyhow::Result<ExitCode> {
    match args.command {
        Command::Check => {
            let (_, had_errors) = pipeline::scan_and_validate(&args.skills_dir)?;
            Ok(if had_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            })
        }
        Command::Build => {
            let BuildOutcome { had_errors, .. } =
                pipeline::build(&args.skills_dir, &args.dist_dir).await?;
            Ok(if had_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            })
        }
        Command::Upload { repo, dry_run } => {
            let BuildOutcome {
                artifacts,
                had_errors,
            } = pipeline::build(&args.skills_dir, &args.dist_dir).await?;

            let (owner, name) = resolve_repo(repo).await.context("resolving GitHub repo")?;
            tracing::info!(repo = format!("{owner}/{name}"), "uploading to GitHub");
            crate::github::upload_new_artifacts(&owner, &name, &artifacts, dry_run).await?;

            Ok(if had_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            })
        }
    }
}

async fn resolve_repo(explicit: Option<String>) -> anyhow::Result<(String, String)> {
    if let Some(s) = explicit {
        return parse_owner_repo(&s);
    }
    if let Ok(s) = std::env::var("GITHUB_REPOSITORY") {
        return parse_owner_repo(&s);
    }
    let out = tokio::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .await
        .context("running `git remote get-url origin`")?;
    if !out.status.success() {
        anyhow::bail!("could not determine GitHub repo (set --repo or GITHUB_REPOSITORY)");
    }
    let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
    parse_git_remote(&url)
}

fn parse_owner_repo(s: &str) -> anyhow::Result<(String, String)> {
    let s = s.trim().trim_end_matches(".git");
    let (o, r) = s
        .split_once('/')
        .with_context(|| format!("expected 'owner/name', got {s:?}"))?;
    Ok((o.to_string(), r.to_string()))
}

fn parse_git_remote(url: &str) -> anyhow::Result<(String, String)> {
    let u = url.trim().trim_end_matches(".git");
    // git@github.com:owner/name
    if let Some(rest) = u.strip_prefix("git@github.com:") {
        return parse_owner_repo(rest);
    }
    // https://github.com/owner/name
    for prefix in [
        "https://github.com/",
        "http://github.com/",
        "ssh://git@github.com/",
    ] {
        if let Some(rest) = u.strip_prefix(prefix) {
            return parse_owner_repo(rest);
        }
    }
    anyhow::bail!("unrecognized git remote URL: {url}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ssh_remote() {
        let (o, r) = parse_git_remote("git@github.com:46ki75/skills.git").unwrap();
        assert_eq!((o.as_str(), r.as_str()), ("46ki75", "skills"));
    }

    #[test]
    fn parses_https_remote() {
        let (o, r) = parse_git_remote("https://github.com/46ki75/skills").unwrap();
        assert_eq!((o.as_str(), r.as_str()), ("46ki75", "skills"));
    }
}
