#![doc = include_str!("../README.md")]

use std::path::{Path, PathBuf};

use gray_matter::{Matter, engine::YAML};
use serde::Deserialize;

/// YAML frontmatter of a `SKILL.md` file.
#[derive(Debug, Clone, Deserialize)]
pub struct SkillFrontmatter {
    /// Skill identifier. Must be kebab-case and match the skill's directory name.
    pub name: String,
    /// Free-form description of what the skill does and when to invoke it.
    pub description: String,
    /// SPDX license identifier (e.g. `"MIT"`). Optional in the spec.
    #[serde(default)]
    pub license: Option<String>,
    /// Repository-specific metadata (author, version, …).
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

/// Repo-local extension fields under the `metadata` key.
///
/// These fields are optional in the upstream spec but treated as required
/// by `skill_validator` for this repository.
#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    /// Skill author. Required by `skill_validator`.
    #[serde(default)]
    pub author: Option<String>,
    /// Skill version (e.g. `"1.0.0"`). Required by `skill_validator`; drives
    /// the `<name>-v<version>` release tag.
    #[serde(default)]
    pub version: Option<String>,
}

/// A fully parsed skill on disk.
#[derive(Debug, Clone)]
pub struct ParsedSkill {
    /// Final path component of `dir_path` (i.e. the directory's own name).
    pub dir_name: String,
    /// Absolute or repo-relative path to the skill's directory.
    pub dir_path: PathBuf,
    /// Deserialized YAML frontmatter.
    pub frontmatter: SkillFrontmatter,
    /// Markdown content following the frontmatter.
    pub body: String,
}

/// Errors produced by [`parse_skill`].
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// The skill directory exists but contains no `SKILL.md`.
    #[error("skill directory has no SKILL.md: {0}")]
    MissingSkillMd(PathBuf),
    /// The `SKILL.md` file has no `---`-delimited YAML frontmatter block.
    #[error("SKILL.md at {path} has no YAML frontmatter delimited by ---")]
    NoFrontmatter {
        /// Path to the offending `SKILL.md`.
        path: PathBuf,
    },
    /// The frontmatter is present but could not be deserialized into [`SkillFrontmatter`].
    #[error("failed to parse YAML frontmatter at {path}: {source}")]
    InvalidFrontmatter {
        /// Path to the offending `SKILL.md`.
        path: PathBuf,
        /// Underlying YAML/deserialization error from [`gray_matter`].
        #[source]
        source: gray_matter::Error,
    },
    /// The directory has no usable final component (e.g. is empty or non-UTF-8).
    #[error("skill directory has no usable name: {0}")]
    InvalidDirName(PathBuf),
    /// Reading `SKILL.md` failed.
    #[error("I/O error reading {path}: {source}")]
    Io {
        /// Path that failed to read.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// Reads `dir/SKILL.md`, splits frontmatter and body, and returns a [`ParsedSkill`].
///
/// Does not enforce validation rules — that is `skill_validator`'s job.
pub async fn parse_skill(dir: &Path) -> Result<ParsedSkill, ParseError> {
    let skill_md = dir.join("SKILL.md");
    let raw = match tokio::fs::read_to_string(&skill_md).await {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(ParseError::MissingSkillMd(dir.to_path_buf()));
        }
        Err(e) => {
            return Err(ParseError::Io {
                path: skill_md.clone(),
                source: e,
            });
        }
    };

    let matter = Matter::<YAML>::new();
    let parsed =
        matter
            .parse::<SkillFrontmatter>(&raw)
            .map_err(|e| ParseError::InvalidFrontmatter {
                path: skill_md.clone(),
                source: e,
            })?;

    let frontmatter = parsed.data.ok_or_else(|| ParseError::NoFrontmatter {
        path: skill_md.clone(),
    })?;

    let dir_name = dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ParseError::InvalidDirName(dir.to_path_buf()))?
        .to_string();

    Ok(ParsedSkill {
        dir_name,
        dir_path: dir.to_path_buf(),
        frontmatter,
        body: parsed.content,
    })
}
