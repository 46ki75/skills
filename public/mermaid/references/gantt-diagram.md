# Gantt Chart Reference

Gantt charts illustrate project schedules showing tasks, durations, and dependencies — great for project planning and roadmaps.

## Basic Syntax

```mermaid
gantt
    title A Gantt Diagram
    dateFormat YYYY-MM-DD
    section Section
        A task          :a1, 2014-01-01, 30d
        Another task    :after a1, 20d
    section Another
        Task in Another :2014-01-12, 12d
        another task    :24d
```

## Required Declarations

### dateFormat

Defines the input date format for task start dates:

```text
dateFormat YYYY-MM-DD
```

**Common tokens:**

| Token | Example | Description |
| --- | --- | --- |
| `YYYY` | `2024` | 4-digit year |
| `YY` | `24` | 2-digit year |
| `MM` | `01`–`12` | Month number |
| `DD` | `01`–`31` | Day of month |
| `HH` | `00`–`23` | Hour (24h) |
| `mm` | `00`–`59` | Minute |
| `ss` | `00`–`59` | Second |

### title

Optional chart title:

```text
title Project Timeline
```

## Tasks

### Task Syntax

```text
taskName : [crit,] [done|active|milestone,] [id,] [startDate,] endDateOrDuration
```

**Components:**

- `taskName` — Display label (required)
- `crit` — Mark as critical path (red highlight)
- `done` — Mark as completed (grayed out)
- `active` — Mark as in-progress (blue highlight)
- `milestone` — Render as a milestone diamond
- `id` — Reference ID for dependencies
- `startDate` — Explicit start date or `after <id>`
- `endDateOrDuration` — End date or duration like `5d`, `2w`, `1h`

### Duration Units

- `d` - days
- `h` - hours
- `m` - minutes (not months)
- `s` - seconds
- `w` - weeks

### Task Status

```mermaid
gantt
    dateFormat YYYY-MM-DD
    section Status Examples
        Completed task  :done, 2024-01-01, 5d
        Active task     :active, 2024-01-06, 5d
        Future task     :2024-01-11, 5d
        Critical task   :crit, 2024-01-16, 5d
```

### Dependencies

Use task IDs in `after` clauses:

```mermaid
gantt
    dateFormat YYYY-MM-DD
    section Dependent Tasks
        Design      :design, 2024-01-01, 7d
        Development :dev, after design, 14d
        Testing     :test, after dev, 5d
        Deployment  :after test, 2d
```

Multiple dependencies: `after task1 task2`

### Milestones

```mermaid
gantt
    dateFormat YYYY-MM-DD
    section Milestones
        Design complete     :milestone, m1, 2024-01-07, 0d
        Beta release        :milestone, m2, 2024-02-01, 0d
        Production release  :milestone, m3, 2024-02-15, 0d
```

Milestones use `0d` duration and render as diamonds.

## Sections

Group related tasks under section headings:

```mermaid
gantt
    dateFormat YYYY-MM-DD
    title Software Project
    section Planning
        Requirements   :r, 2024-01-01, 5d
        Architecture   :a, after r, 5d
    section Development
        Frontend       :f, 2024-01-11, 14d
        Backend        :b, 2024-01-11, 14d
    section Testing
        Integration    :i, after f b, 7d
        UAT            :after i, 5d
```

## Display Configuration

### axisFormat

Control the date display on the time axis:

```text
axisFormat %Y-%m-%d
```

**Common format strings:**

- `%Y` - 4-digit year
- `%y` - 2-digit year
- `%m` - Month number
- `%d` - Day
- `%H` - Hour
- `%M` - Minute
- `%b` - Month abbreviation (Jan, Feb…)
- `%B` - Full month name

### tickInterval

Set the interval between axis ticks:

```text
tickInterval 1week
tickInterval 1day
tickInterval 1month
```

### weekday

Set which day starts the week:

```text
weekday monday
```

### todayMarker

Show a vertical line at today's date:

```text
todayMarker on
todayMarker off
todayMarker stroke-width:5px,stroke:#0f0,opacity:0.5
```

### excludes

Exclude weekends or specific dates from scheduling:

```text
excludes weekends
excludes 2024-12-25, 2025-01-01
```

When a task would land on an excluded date, it shifts forward.

## Comments

```mermaid
gantt
    %% This is a comment
    dateFormat YYYY-MM-DD
```

## Common Patterns

### Sprint Planning

```mermaid
gantt
    title Sprint 12 Plan
    dateFormat YYYY-MM-DD
    axisFormat %d %b

    section Design
        UI mockups      :done, d1, 2024-01-01, 3d
        Design review   :done, d2, after d1, 1d

    section Development
        Auth feature    :active, dev1, 2024-01-05, 5d
        API endpoints   :dev2, after dev1, 4d
        Frontend pages  :dev3, 2024-01-05, 7d

    section QA
        Test auth       :after dev1, 2d
        Regression      :crit, after dev2 dev3, 3d
        Go/No-go        :milestone, after dev2 dev3, 0d
```

### Product Roadmap

```mermaid
gantt
    title Product Roadmap 2024
    dateFormat YYYY-MM
    axisFormat %b %Y

    section Q1
        Research & Discovery  :done, 2024-01, 2M
        MVP Development       :active, 2024-02, 2M

    section Q2
        Beta Launch           :milestone, 2024-04, 0M
        User Feedback         :2024-04, 1M
        Iteration 1           :2024-05, 2M

    section Q3
        Public Launch         :milestone, crit, 2024-07, 0M
        Growth Features       :2024-07, 3M
```
