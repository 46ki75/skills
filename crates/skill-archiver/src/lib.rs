#![doc = include_str!("../README.md")]

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use skill_parser::ParsedSkill;
use walkdir::{DirEntry, WalkDir};
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

/// Errors produced by [`clean_dist`] and [`build_archive`].
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    /// The skill has no `metadata.version`, so the artifact filename and tag
    /// cannot be determined.
    #[error("skill {0:?} has no metadata.version (cannot build archive)")]
    MissingVersion(String),
    /// File-system I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// Failure from the `zip` crate while writing the archive.
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    /// Failure from `walkdir` while enumerating skill contents.
    #[error("walkdir error: {0}")]
    Walk(#[from] walkdir::Error),
    /// The blocking task that performs the zip write panicked or was cancelled.
    #[error("join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

/// Metadata about a successfully built ZIP, used downstream by the uploader.
#[derive(Debug, Clone)]
pub struct BuiltArtifact {
    /// Skill name from the frontmatter.
    pub name: String,
    /// Skill version from `metadata.version`.
    pub version: String,
    /// GitHub release tag — always `"{name}-v{version}"`.
    pub tag: String,
    /// Bare filename of the archive (e.g. `markdown-v1.0.0.zip`).
    pub file_name: String,
    /// Full path to the archive inside the dist directory.
    pub zip_path: PathBuf,
}

/// Removes `dist` if present and recreates it as an empty directory.
pub async fn clean_dist(dist: &Path) -> Result<(), ArchiveError> {
    if tokio::fs::try_exists(dist).await? {
        tokio::fs::remove_dir_all(dist).await?;
    }
    tokio::fs::create_dir_all(dist).await?;
    Ok(())
}

/// Builds `dist/<name>-v<version>.zip` from the skill's directory tree.
///
/// Returns a [`BuiltArtifact`] describing the resulting file. Returns
/// [`ArchiveError::MissingVersion`] if `metadata.version` is absent — callers
/// should run `skill_validator::validate` first to surface that consistently.
pub async fn build_archive(
    skill: &ParsedSkill,
    dist: &Path,
) -> Result<BuiltArtifact, ArchiveError> {
    let version = skill
        .frontmatter
        .metadata
        .as_ref()
        .and_then(|m| m.version.clone())
        .ok_or_else(|| ArchiveError::MissingVersion(skill.dir_name.clone()))?;

    let name = skill.frontmatter.name.clone();
    let tag = format!("{name}-v{version}");
    let file_name = format!("{tag}.zip");
    let zip_path = dist.join(&file_name);

    let src = skill.dir_path.clone();
    let dst = zip_path.clone();
    let dir_name = skill.dir_name.clone();

    tokio::task::spawn_blocking(move || zip_skill_dir(&src, &dir_name, &dst)).await??;

    Ok(BuiltArtifact {
        name,
        version,
        tag,
        file_name,
        zip_path,
    })
}

fn zip_skill_dir(src: &Path, top_dir: &str, zip_path: &Path) -> Result<(), ArchiveError> {
    let file = File::create(zip_path)?;
    let mut writer = ZipWriter::new(file);
    let options: FileOptions<()> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    let dir_options: FileOptions<()> = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    let walker = WalkDir::new(src)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !is_hidden(e));

    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        let rel = match path.strip_prefix(src) {
            Ok(r) => r,
            Err(_) => continue,
        };

        let archive_path = if rel.as_os_str().is_empty() {
            PathBuf::from(top_dir)
        } else {
            let mut p = PathBuf::from(top_dir);
            p.push(rel);
            p
        };
        let archive_path_str = archive_path.to_string_lossy().replace('\\', "/");

        if entry.file_type().is_dir() {
            writer.add_directory(format!("{archive_path_str}/"), dir_options)?;
        } else if entry.file_type().is_file() {
            writer.start_file(archive_path_str, options)?;
            let mut reader = BufReader::new(File::open(path)?);
            std::io::copy(&mut reader, &mut writer)?;
        }
    }

    writer.finish()?;
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    // depth() is 0 only for the root of the walk; never prune the root.
    if entry.depth() == 0 {
        return false;
    }
    entry
        .file_name()
        .to_str()
        .is_some_and(|n| n.starts_with('.'))
}
