use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use skill_parser::ParsedSkill;
use walkdir::WalkDir;
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("skill {0:?} has no metadata.version (cannot build archive)")]
    MissingVersion(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("walkdir error: {0}")]
    Walk(#[from] walkdir::Error),
    #[error("join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

#[derive(Debug, Clone)]
pub struct BuiltArtifact {
    pub name: String,
    pub version: String,
    pub tag: String,
    pub file_name: String,
    pub zip_path: PathBuf,
}

pub async fn clean_dist(dist: &Path) -> Result<(), ArchiveError> {
    if tokio::fs::try_exists(dist).await? {
        tokio::fs::remove_dir_all(dist).await?;
    }
    tokio::fs::create_dir_all(dist).await?;
    Ok(())
}

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

    let mut buf = Vec::new();

    for entry in WalkDir::new(src).follow_links(false).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();

        if path
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|n| n.starts_with('.'))
        {
            continue;
        }
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
            buf.clear();
            File::open(path)?.read_to_end(&mut buf)?;
            writer.write_all(&buf)?;
        }
    }

    writer.finish()?;
    Ok(())
}
