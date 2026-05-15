use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Context;
use octocrab::Octocrab;
use octocrab::models::ReleaseId;
use skill_archiver::BuiltArtifact;

/// One existing release on GitHub, indexed by tag.
struct ExistingRelease {
    id: ReleaseId,
    assets: HashSet<String>,
}

pub async fn upload_new_artifacts(
    owner: &str,
    repo: &str,
    artifacts: &[BuiltArtifact],
    dry_run: bool,
) -> anyhow::Result<()> {
    if artifacts.is_empty() {
        tracing::info!("no artifacts to consider for upload");
        return Ok(());
    }

    let token = std::env::var("GITHUB_TOKEN")
        .context("GITHUB_TOKEN must be set to upload releases")?
        .trim()
        .to_string();
    if token.is_empty() {
        anyhow::bail!("GITHUB_TOKEN is empty after trimming whitespace");
    }
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .context("building octocrab client")?;

    let existing = list_existing_releases(&octocrab, owner, repo).await?;
    tracing::info!(count = existing.len(), "fetched existing releases");

    let mut created = 0usize;
    let mut asset_uploads = 0usize;
    let mut skipped = 0usize;

    for art in artifacts {
        match existing.get(&art.tag) {
            Some(release) if release.assets.contains(&art.file_name) => {
                tracing::info!(tag = %art.tag, "release and asset already present, skipping");
                skipped += 1;
            }
            Some(release) => {
                if dry_run {
                    tracing::info!(
                        tag = %art.tag,
                        "[dry-run] release exists without asset; would upload asset"
                    );
                } else {
                    upload_asset(&octocrab, owner, repo, release.id, art).await?;
                }
                asset_uploads += 1;
            }
            None => {
                if dry_run {
                    tracing::info!(
                        tag = %art.tag,
                        "[dry-run] would create release and upload asset"
                    );
                } else {
                    create_release_and_upload(&octocrab, owner, repo, art).await?;
                }
                created += 1;
            }
        }
    }

    tracing::info!(
        created,
        asset_uploads,
        skipped,
        dry_run,
        "upload run complete"
    );
    Ok(())
}

async fn list_existing_releases(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
) -> anyhow::Result<HashMap<String, ExistingRelease>> {
    let mut releases_by_tag = HashMap::new();
    let repos = octocrab.repos(owner, repo);
    let releases = repos.releases();

    let mut page = releases
        .list()
        .per_page(100)
        .send()
        .await
        .with_context(|| format!("listing releases for {owner}/{repo}"))?;

    loop {
        for rel in page.take_items() {
            let assets = rel.assets.iter().map(|a| a.name.clone()).collect();
            releases_by_tag.insert(rel.tag_name.clone(), ExistingRelease { id: rel.id, assets });
        }
        match octocrab
            .get_page::<octocrab::models::repos::Release>(&page.next)
            .await
            .with_context(|| format!("paginating releases for {owner}/{repo}"))?
        {
            Some(next) => page = next,
            None => break,
        }
    }

    Ok(releases_by_tag)
}

async fn create_release_and_upload(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    art: &BuiltArtifact,
) -> anyhow::Result<()> {
    let repos = octocrab.repos(owner, repo);
    let releases = repos.releases();

    let release = releases
        .create(&art.tag)
        .name(&art.tag)
        .body(&format!(
            "Automated release of skill `{}` version `{}`.",
            art.name, art.version
        ))
        .send()
        .await
        .with_context(|| format!("creating release {}", art.tag))?;

    upload_asset(octocrab, owner, repo, release.id, art).await
}

async fn upload_asset(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    release_id: ReleaseId,
    art: &BuiltArtifact,
) -> anyhow::Result<()> {
    let repos = octocrab.repos(owner, repo);
    let releases = repos.releases();

    let bytes = tokio::fs::read(&art.zip_path)
        .await
        .with_context(|| format!("reading {}", art.zip_path.display()))?;
    let len = bytes.len();

    releases
        .upload_asset(release_id.0, &art.file_name, bytes.into())
        .send()
        .await
        .with_context(|| format!("uploading asset {}", art.file_name))?;

    tracing::info!(tag = %art.tag, bytes = len, "asset uploaded");
    Ok(())
}
