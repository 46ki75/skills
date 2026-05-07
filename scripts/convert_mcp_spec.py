#!/usr/bin/env python3
"""Convert MCP MDX spec files to clean Markdown for skill reference files."""

import re
import sys
from pathlib import Path

SPEC_BASE = Path("submodules/modelcontextprotocol/docs/specification")
OUT_BASE = Path("skills/mcp/references")

VERSIONS = ["2024-11-05", "2025-03-26", "2025-06-18", "2025-11-25"]

# Files to include per version (relative to version dir)
FILES_PER_VERSION = {
    "2024-11-05": [
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-03-26": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-06-18": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("client/elicitation.mdx", "elicitation.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
    ],
    "2025-11-25": [
        ("changelog.mdx", "changelog.md"),
        ("architecture/index.mdx", "architecture.md"),
        ("basic/lifecycle.mdx", "lifecycle.md"),
        ("basic/transports.mdx", "transports.md"),
        ("basic/authorization.mdx", "authorization.md"),
        ("server/resources.mdx", "resources.md"),
        ("server/prompts.mdx", "prompts.md"),
        ("server/tools.mdx", "tools.md"),
        ("client/sampling.mdx", "sampling.md"),
        ("client/roots.mdx", "roots.md"),
        ("client/elicitation.mdx", "elicitation.md"),
        ("server/utilities/logging.mdx", "logging.md"),
        ("server/utilities/pagination.mdx", "pagination.md"),
        ("server/utilities/completion.mdx", "completion.md"),
        ("basic/utilities/cancellation.mdx", "cancellation.md"),
        ("basic/utilities/ping.mdx", "ping.md"),
        ("basic/utilities/progress.mdx", "progress.md"),
        ("basic/utilities/tasks.mdx", "tasks.md"),
    ],
}


def convert_mdx(content: str, title_from_frontmatter: str = "") -> str:
    """Convert MDX content to clean Markdown."""

    # Strip YAML frontmatter and extract title
    frontmatter_match = re.match(r"^---\n(.*?)\n---\n", content, re.DOTALL)
    fm_title = ""
    if frontmatter_match:
        fm_block = frontmatter_match.group(1)
        title_match = re.search(r"^title:\s*(.+)$", fm_block, re.MULTILINE)
        if title_match:
            fm_title = title_match.group(1).strip().strip('"')
        content = content[frontmatter_match.end():]

    title = fm_title or title_from_frontmatter

    # Prepend markdownlint-disable so all MDX-converted files are exempt
    header = "<!-- markdownlint-disable -->\n"
    if title:
        header += f"# {title}\n\n"

    # Strip <div ...> tags (self-closing or with content)
    content = re.sub(r"<div[^>]*/>\s*", "", content)
    content = re.sub(r"<div[^>]*>.*?</div>", "", content, flags=re.DOTALL)

    # Convert <Warning>, <Note>, <Info> blocks to blockquotes
    for tag in ("Warning", "Note", "Info"):
        def replace_block(m, tag=tag):
            inner = m.group(1).strip()
            lines = inner.split("\n")
            quoted = "\n".join(f"> {line}" if line.strip() else ">" for line in lines)
            return f"\n> **{tag}:**\n{quoted}\n"
        content = re.sub(
            rf"<{tag}>\s*(.*?)\s*</{tag}>",
            replace_block,
            content,
            flags=re.DOTALL,
        )

    # Strip CardGroup / Card components entirely
    content = re.sub(r"<CardGroup[^>]*>.*?</CardGroup>", "", content, flags=re.DOTALL)
    content = re.sub(r"<Card[^/].*?/>", "", content, flags=re.DOTALL)
    content = re.sub(r"<Card[^>]*>.*?</Card>", "", content, flags=re.DOTALL)

    # Strip any remaining self-closing JSX tags
    content = re.sub(r"<[A-Z][A-Za-z]*[^>]*/>\s*", "", content)

    # Strip any remaining JSX opening/closing tags that may remain
    content = re.sub(r"<[A-Z][A-Za-z]*[^>]*>", "", content)
    content = re.sub(r"</[A-Z][A-Za-z]*>", "", content)

    # Collapse 3+ blank lines to 2
    content = re.sub(r"\n{3,}", "\n\n", content)

    return header + content.lstrip()


def main():
    for version in VERSIONS:
        out_dir = OUT_BASE / version
        out_dir.mkdir(parents=True, exist_ok=True)

        for src_rel, dst_name in FILES_PER_VERSION[version]:
            src_path = SPEC_BASE / version / src_rel
            if not src_path.exists():
                print(f"  SKIP (not found): {src_path}", file=sys.stderr)
                continue

            raw = src_path.read_text(encoding="utf-8")
            converted = convert_mdx(raw)

            dst_path = out_dir / dst_name
            dst_path.write_text(converted, encoding="utf-8")
            print(f"  wrote {dst_path}")

    print("Done.")


if __name__ == "__main__":
    main()
