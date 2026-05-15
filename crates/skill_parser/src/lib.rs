use std::path::{Path, PathBuf};

use gray_matter::{Matter, engine::YAML};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedSkill {
    pub dir_name: String,
    pub dir_path: PathBuf,
    pub frontmatter: SkillFrontmatter,
    pub body: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("skill directory has no SKILL.md: {0}")]
    MissingSkillMd(PathBuf),
    #[error("SKILL.md at {path} has no YAML frontmatter delimited by ---")]
    NoFrontmatter { path: PathBuf },
    #[error("failed to parse YAML frontmatter at {path}: {source}")]
    InvalidFrontmatter {
        path: PathBuf,
        #[source]
        source: gray_matter::Error,
    },
    #[error("skill directory has no usable name: {0}")]
    InvalidDirName(PathBuf),
    #[error("I/O error reading {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub fn parse_skill(dir: &Path) -> Result<ParsedSkill, ParseError> {
    let skill_md = dir.join("SKILL.md");
    if !skill_md.is_file() {
        return Err(ParseError::MissingSkillMd(dir.to_path_buf()));
    }

    let raw = std::fs::read_to_string(&skill_md).map_err(|e| ParseError::Io {
        path: skill_md.clone(),
        source: e,
    })?;

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
