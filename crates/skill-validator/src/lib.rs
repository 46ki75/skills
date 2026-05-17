#![doc = include_str!("../README.md")]

use skill_parser::ParsedSkill;

/// A single rule violation discovered by [`validate`].
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Frontmatter `name` is missing or whitespace-only.
    #[error("name is empty")]
    NameMissing,
    /// Frontmatter `name` contains characters outside `[a-z0-9-]` or has leading/trailing `-`.
    #[error("name {name:?} is not kebab-case (allowed: lowercase letters, digits, hyphens)")]
    NameNotKebabCase {
        /// The offending `name` value.
        name: String,
    },
    /// Frontmatter `name` does not equal the on-disk directory name.
    #[error("name {name:?} does not match directory name {dir:?}")]
    NameMismatch {
        /// The `name` declared in the frontmatter.
        name: String,
        /// The directory name found on disk.
        dir: String,
    },
    /// Frontmatter `description` is missing or whitespace-only.
    #[error("description is empty")]
    DescriptionMissing,
    /// Frontmatter `description` exceeds the 1024-character upload limit.
    #[error("description is too long ({0} chars, max 1024)")]
    DescriptionTooLong(usize),
    /// Frontmatter `description` contains an XML/HTML-like tag.
    #[error("description must not contain XML tags (found {0:?})")]
    DescriptionContainsXmlTag(String),
    /// `metadata.author` is required by this repo but absent or empty.
    #[error("metadata.author is required but missing")]
    AuthorMissing,
    /// `metadata.version` is required by this repo but absent or empty.
    #[error("metadata.version is required but missing")]
    VersionMissing,
    /// `metadata.version` is present but does not match the expected shape.
    #[error(
        "metadata.version {0:?} is not a valid semver-like version (expected MAJOR.MINOR.PATCH, digits only, no leading zeros)"
    )]
    InvalidVersion(String),
}

/// Outcome of validating a single skill.
#[derive(Debug, Default)]
#[must_use = "inspect the report (or call `is_ok`) before assuming a skill is publishable"]
pub struct ValidationReport {
    /// Directory name of the skill that was validated.
    pub dir_name: String,
    /// Every rule violation found. Empty on success.
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    /// Returns `true` when no rule violations were recorded.
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

/// Validates a [`ParsedSkill`] and returns every rule violation found.
///
/// The returned report is always populated with `dir_name`. Inspect
/// [`ValidationReport::is_ok`] (or `errors.is_empty()`) to decide whether
/// the skill is publishable.
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
    } else {
        let len = fm.description.chars().count();
        if len > 1024 {
            errors.push(ValidationError::DescriptionTooLong(len));
        }
        if let Some(tag) = find_xml_tag(&fm.description) {
            errors.push(ValidationError::DescriptionContainsXmlTag(tag));
        }
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
    if !s
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return false;
    }
    s.split('-').all(|segment| !segment.is_empty())
}

fn find_xml_tag(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let after = bytes.get(i + 1).copied();
            let is_tag_start = matches!(after, Some(b'/') | Some(b'a'..=b'z') | Some(b'A'..=b'Z'));
            if is_tag_start {
                if let Some(end) = bytes[i..].iter().position(|&b| b == b'>') {
                    return Some(s[i..i + end + 1].to_string());
                }
            }
        }
        i += 1;
    }
    None
}

fn is_semver_like(v: &str) -> bool {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| {
        !p.is_empty()
            && p.chars().all(|c| c.is_ascii_digit())
            && !(p.len() > 1 && p.starts_with('0'))
    })
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
    fn two_part_version_rejected() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("1.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));
    }

    #[test]
    fn four_part_version_rejected() {
        let s = make(
            "markdown",
            "markdown",
            "lint md",
            Some("X"),
            Some("1.0.0.0"),
        );
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));
    }

    #[test]
    fn version_with_leading_zero_rejected() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("01.0.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));

        let s = make("markdown", "markdown", "lint md", Some("X"), Some("1.02.0"));
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));

        let s = make(
            "markdown",
            "markdown",
            "lint md",
            Some("X"),
            Some("1.0.003"),
        );
        let r = validate(&s);
        assert!(matches!(r.errors[0], ValidationError::InvalidVersion(_)));
    }

    #[test]
    fn version_zero_segment_ok() {
        let s = make("markdown", "markdown", "lint md", Some("X"), Some("0.0.0"));
        assert!(validate(&s).is_ok());

        let s = make("markdown", "markdown", "lint md", Some("X"), Some("1.0.0"));
        assert!(validate(&s).is_ok());
    }

    #[test]
    fn double_hyphen_rejected() {
        let s = make("foo--bar", "foo--bar", "x", Some("X"), Some("1.0.0"));
        let r = validate(&s);
        assert!(matches!(
            r.errors[0],
            ValidationError::NameNotKebabCase { .. }
        ));
    }

    #[test]
    fn trailing_hyphen_rejected() {
        let s = make("foo-", "foo-", "x", Some("X"), Some("1.0.0"));
        let r = validate(&s);
        assert!(matches!(
            r.errors[0],
            ValidationError::NameNotKebabCase { .. }
        ));
    }

    #[test]
    fn description_at_limit_ok() {
        let desc = "a".repeat(1024);
        let s = make("markdown", "markdown", &desc, Some("X"), Some("1.0.0"));
        assert!(validate(&s).is_ok());
    }

    #[test]
    fn description_too_long() {
        let desc = "a".repeat(1025);
        let s = make("markdown", "markdown", &desc, Some("X"), Some("1.0.0"));
        let r = validate(&s);
        assert!(matches!(
            r.errors[0],
            ValidationError::DescriptionTooLong(1025)
        ));
    }

    #[test]
    fn description_with_xml_tag_rejected() {
        let s = make(
            "markdown",
            "markdown",
            "hello <example>foo</example>",
            Some("X"),
            Some("1.0.0"),
        );
        let r = validate(&s);
        assert!(matches!(
            r.errors[0],
            ValidationError::DescriptionContainsXmlTag(_)
        ));
    }

    #[test]
    fn description_with_self_closing_tag_rejected() {
        let s = make(
            "markdown",
            "markdown",
            "x <br/> y",
            Some("X"),
            Some("1.0.0"),
        );
        let r = validate(&s);
        assert!(matches!(
            r.errors[0],
            ValidationError::DescriptionContainsXmlTag(_)
        ));
    }

    #[test]
    fn description_with_lt_operator_ok() {
        let s = make(
            "markdown",
            "markdown",
            "use when a < b or 1 < 2",
            Some("X"),
            Some("1.0.0"),
        );
        assert!(validate(&s).is_ok());
    }
}
