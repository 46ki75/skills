use skill_parser::ParsedSkill;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("name is empty")]
    NameMissing,
    #[error("name {name:?} is not kebab-case (allowed: lowercase letters, digits, hyphens)")]
    NameNotKebabCase { name: String },
    #[error("name {name:?} does not match directory name {dir:?}")]
    NameMismatch { name: String, dir: String },
    #[error("description is empty")]
    DescriptionMissing,
    #[error("metadata.author is required but missing")]
    AuthorMissing,
    #[error("metadata.version is required but missing")]
    VersionMissing,
    #[error(
        "metadata.version {0:?} is not a valid semver-like version (expected MAJOR.MINOR[.PATCH])"
    )]
    InvalidVersion(String),
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    pub dir_name: String,
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

impl std::fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "{}: OK", self.dir_name);
        }
        writeln!(f, "{}: {} error(s)", self.dir_name, self.errors.len())?;
        for e in &self.errors {
            writeln!(f, "  - {e}")?;
        }
        Ok(())
    }
}

pub fn validate(skill: &ParsedSkill) -> ValidationReport {
    let mut errors = Vec::new();
    let fm = &skill.frontmatter;

    if fm.name.trim().is_empty() {
        errors.push(ValidationError::NameMissing);
    } else if !is_kebab_case(&fm.name) {
        errors.push(ValidationError::NameNotKebabCase {
            name: fm.name.clone(),
        });
    } else if fm.name != skill.dir_name {
        errors.push(ValidationError::NameMismatch {
            name: fm.name.clone(),
            dir: skill.dir_name.clone(),
        });
    }

    if fm.description.trim().is_empty() {
        errors.push(ValidationError::DescriptionMissing);
    }

    match fm.metadata.as_ref() {
        None => {
            errors.push(ValidationError::AuthorMissing);
            errors.push(ValidationError::VersionMissing);
        }
        Some(m) => {
            if m.author.as_deref().map(str::trim).unwrap_or("").is_empty() {
                errors.push(ValidationError::AuthorMissing);
            }
            match m
                .version
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                None => errors.push(ValidationError::VersionMissing),
                Some(v) if !is_semver_like(v) => {
                    errors.push(ValidationError::InvalidVersion(v.to_string()))
                }
                Some(_) => {}
            }
        }
    }

    ValidationReport {
        dir_name: skill.dir_name.clone(),
        errors,
    }
}

fn is_kebab_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn is_semver_like(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if !(2..=3).contains(&parts.len()) {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use skill_parser::{Metadata, ParsedSkill, SkillFrontmatter};
    use std::path::PathBuf;

    fn make(
        dir: &str,
        name: &str,
        desc: &str,
        author: Option<&str>,
        version: Option<&str>,
    ) -> ParsedSkill {
        ParsedSkill {
            dir_name: dir.to_string(),
            dir_path: PathBuf::from(dir),
            frontmatter: SkillFrontmatter {
                name: name.to_string(),
                description: desc.to_string(),
                license: Some("MIT".into()),
                metadata: Some(Metadata {
                    author: author.map(String::from),
                    version: version.map(String::from),
                }),
            },
            body: String::new(),
        }
    }

    #[test]
    fn happy_path() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("1.0.0"));
        assert!(validate(&s).is_ok());
    }

    #[test]
    fn missing_author() {
        let s = make("markdown", "markdown", "lint md", None, Some("1.0.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::AuthorMissing));
    }

    #[test]
    fn missing_version() {
        let s = make("markdown", "markdown", "lint md", Some("X"), None);
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::VersionMissing));
    }

    #[test]
    fn name_dir_mismatch() {
        let s = make("markdown", "md", "lint md", Some("X"), Some("1.0.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::NameMismatch { .. }));
    }

    #[test]
    fn bad_version() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("v1.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));
    }

    #[test]
    fn two_part_version_ok() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("1.0"));
        assert!(validate(&s).is_ok());
    }
}
