# Pie Chart Reference

Pie charts display proportional data as slices of a circle — ideal for showing percentages, distributions, and part-to-whole relationships.

## Basic Syntax

```mermaid
pie title Pets adopted by volunteers
    "Dogs" : 386
    "Cats" : 85
    "Rats" : 15
```

Each slice is defined as `"Label" : value`. Labels must be quoted. Values are relative weights (not required to sum to 100).

## Title

Add an optional title inline or via frontmatter:

```mermaid
pie title My Distribution
    "A" : 40
    "B" : 30
    "C" : 20
    "D" : 10
```

## Show Data (percentages on slices)

Use `showData` to display raw values on the slices instead of percentages:

```mermaid
pie showData
    title Browser Market Share
    "Chrome" : 65.5
    "Safari" : 18.9
    "Firefox" : 4.0
    "Edge" : 4.1
    "Other" : 7.5
```

## Configuration

### textPosition

Control where label text appears — `0` is center, `1` is outer edge (default `0.75`):

```mermaid
---
config:
  pie:
    textPosition: 0.5
---
pie title Votes
    "Alice" : 45
    "Bob" : 30
    "Carol" : 25
```

## Common Patterns

### Budget Allocation

```mermaid
pie title Annual Budget Allocation
    "Engineering" : 40
    "Marketing" : 25
    "Operations" : 15
    "HR" : 10
    "Research" : 10
```

### Survey Results

```mermaid
pie showData title Customer Satisfaction
    "Very Satisfied" : 342
    "Satisfied" : 215
    "Neutral" : 88
    "Dissatisfied" : 44
    "Very Dissatisfied" : 11
```

### Technology Usage

```mermaid
pie title Programming Languages Used
    "Python" : 30
    "JavaScript" : 28
    "TypeScript" : 18
    "Java" : 12
    "Go" : 7
    "Other" : 5
```

## Tips

- Slices render in the order they are declared
- Values are automatically normalized to percentages
- Use `showData` when exact counts matter more than proportions
- Keep slices to 5–7 for readability; combine small values into "Other"
