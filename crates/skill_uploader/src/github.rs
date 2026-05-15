use std::collections::HashSet;

use anyhow::Context;
use octocrab::Octocrab;
use skill_archiver::BuiltArtifact;

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

    let token =
        std::env::var("GITHUB_TOKEN").context("GITHUB_TOKEN must be set to upload releases")?;
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .context("building octocrab client")?;

    let existing = list_existing_tags(&octocrab, owner, repo).await?;
    tracing::info!(count = existing.len(), "fetched existing release tags");

    let mut uploaded = 0usize;
    let mut skipped = 0usize;

    for art in artifacts {
        if existing.contains(&art.tag) {
            tracing::info!(tag = %art.tag, "release already exists, skipping");
            skipped += 1;
            continue;
        }
        if dry_run {
            tracing::info!(tag = %art.tag, "[dry-run] would create release and upload asset");
            uploaded += 1;
            continue;
        }
        upload_one(&octocrab, owner, repo, art).await?;
        uploaded += 1;
    }

    tracing::info!(uploaded, skipped, "upload run complete");
    Ok(())
}

async fn list_existing_tags(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
) -> anyhow::Result<HashSet<String>> {
    let mut tags = HashSet::new();
    let repos = octocrab.repos(owner, repo);
    let releases = repos.releases();

    let mut page = releases
        .list()
        .per_page(100)
        .send()
        .await
        .with_context(|| format!("listing releases for {owner}/{repo}"))?;

    loop {
        for rel in &page.items {
            tags.insert(rel.tag_name.clone());
        }
        match octocrab
            .get_page::<octocrab::models::repos::Release>(&page.next)
            .await?
        {
            Some(next) => page = next,
            None => break,
        }
    }

    Ok(tags)
}

async fn upload_one(
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

    let bytes = tokio::fs::read(&art.zip_path)
        .await
        .with_context(|| format!("reading {}", art.zip_path.display()))?;
    let len = bytes.len();

    releases
        .upload_asset(release.id.0, &art.file_name, bytes.into())
        .send()
        .await
        .with_context(|| format!("uploading asset {}", art.file_name))?;

    tracing::info!(tag = %art.tag, bytes = len, "release created and asset uploaded");
    Ok(())
}
