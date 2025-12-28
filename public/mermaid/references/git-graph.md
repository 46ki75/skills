# Git Graph Diagrams

Git Graphs are pictorial representations of git commits and actions on various branches, particularly helpful for visualizing branching strategies and git flow.

## Basic Structure

```mermaid
gitGraph
   commit
   commit
   branch develop
   checkout develop
   commit
   commit
   checkout main
   merge develop
   commit
```

Always start with the `gitGraph` keyword. The diagram initializes with a `main` branch by default.

## Core Commands

### commit
Register a commit on the current branch.

```mermaid
gitGraph
   commit
   commit
   commit
```

**Attributes:**
- `id: "custom_id"` - Custom commit identifier
- `type: NORMAL|REVERSE|HIGHLIGHT` - Visual representation
  - `NORMAL`: Solid circle (default)
  - `REVERSE`: Crossed solid circle
  - `HIGHLIGHT`: Filled rectangle
- `tag: "tag_name"` - Add release/version tag

**Examples:**
```mermaid
gitGraph
   commit id: "Alpha"
   commit id: "Beta" type: REVERSE
   commit id: "Gamma" type: HIGHLIGHT tag: "v1.0.0"
```

### branch
Create and switch to a new branch (sets it as current).

```mermaid
gitGraph
   commit
   commit
   branch develop
   commit  # This commit goes to develop
```

**Usage:**
- `branch develop`
- `branch "cherry-pick"` (use quotes for names that could be keywords)
- Branch name must be unique

**Order control:**
- `branch develop order: 2` - Control visual branch ordering (lower = left, higher = right)

### checkout / switch
Switch to an existing branch (sets it as current). Both keywords are interchangeable.

```mermaid
gitGraph
   commit
   branch develop
   commit
   checkout main  # or: switch main
   commit
```

### merge
Merge a branch into the current branch. Creates a merge commit (filled double circle).

```mermaid
gitGraph
   commit
   branch develop
   commit
   checkout main
   merge develop
   commit
```

**Attributes:**
- `id: "merge_id"` - Custom merge commit ID
- `tag: "tag_name"` - Tag for merge commit
- `type: NORMAL|REVERSE|HIGHLIGHT` - Override merge commit shape

**Example:**
```mermaid
gitGraph
   commit id: "1"
   branch feature
   commit id: "2"
   checkout main
   commit id: "3"
   merge feature id: "merge-1" tag: "v2.0" type: REVERSE
```

### cherry-pick
Apply a commit from another branch to the current branch.

**Syntax:**
- `cherry-pick id:"commit_id"` - Cherry-pick specific commit
- `cherry-pick tag:"tag_name"` - Cherry-pick tagged commit
- Supports same attributes as commit: `id`, `type`, `tag`, `parent`

**Parent specification:**
```mermaid
gitGraph
   commit id: "A"
   branch feature
   commit id: "B"
   checkout main
   cherry-pick id:"B" parent:"A"
```

## Advanced Features

### Commit Ordering and Parents
Control commit positioning using `parent` attribute:

```mermaid
gitGraph
   commit id: "A"
   commit id: "B"
   branch feature
   commit id: "C" parent:"A"  # Skip B
```

### Branch Ordering
Control visual branch layout:

```mermaid
gitGraph
   branch feature1 order: 1
   branch feature2 order: 3
   branch hotfix order: 2
```

### Multiple Parents (Merge Commits)
Specify multiple parents for complex merges:

```mermaid
gitGraph
   commit id: "1-abcdefg"
   commit id: "2-abcdefg"
   branch develop
   commit id: "3-abcdefg"
   checkout main
   merge develop id: "5-abcdefg" type: REVERSE tag: "v1.0.0"
```

## Theming and Customization

### Branch Colors
Customize using `git0` through `git7` theme variables (supports up to 8 branches, then cycles):

```mermaid
---
config:
  theme: 'base'
  themeVariables:
    'git0': '#ff0000'
    'git1': '#00ff00'
    'git2': '#0000ff'
---
gitGraph
   commit
   branch develop
   commit
```

### Branch Label Colors
Use `gitBranchLabel0` through `gitBranchLabel7`:

```mermaid
---
config:
  themeVariables:
    'gitBranchLabel0': '#ffffff'
    'gitBranchLabel1': '#000000'
---
gitGraph
   commit
   branch develop
```

### Commit Styling
- `commitLabelColor` - Commit label text color
- `commitLabelBackground` - Commit label background
- `commitLabelFontSize` - Font size (e.g., `'16px'`)

### Tag Styling
- `tagLabelColor` - Tag text color
- `tagLabelBackground` - Tag background color
- `tagLabelBorder` - Tag border color
- `tagLabelFontSize` - Font size (e.g., `'16px'`)

### Highlight Commit Colors
Use `gitInv0` through `gitInv7` for branch-specific highlight commits:

```mermaid
---
config:
  themeVariables:
    'gitInv0': '#ff0000'  # Highlights on first branch (main)
---
gitGraph
   commit type: HIGHLIGHT
```

## Common Patterns

### Git Flow
```mermaid
gitGraph
   commit
   branch develop
   checkout develop
   commit
   branch feature
   checkout feature
   commit
   commit
   checkout develop
   merge feature
   checkout main
   merge develop tag: "v1.0"
```

### Hotfix Workflow
```mermaid
gitGraph
   commit
   branch hotfix
   checkout hotfix
   commit
   checkout main
   merge hotfix tag: "v1.0.1"
   branch develop
   checkout develop
   merge hotfix
```

### Feature Branches
```mermaid
gitGraph
   commit
   branch feature-a
   branch feature-b
   checkout feature-a
   commit
   checkout feature-b
   commit
   checkout main
   merge feature-a
   merge feature-b
```

## Best Practices

1. **Branch naming**: Use descriptive names; quote names that could be keywords
2. **Commit IDs**: Use consistent ID patterns (e.g., semantic versions, short hashes)
3. **Visual clarity**: Use `HIGHLIGHT` type sparingly for important commits
4. **Tags**: Apply to release commits and major milestones
5. **Branch ordering**: Use `order` attribute for complex diagrams with many branches
6. **Theming**: Apply consistent color schemes for branch types (e.g., main, develop, feature)