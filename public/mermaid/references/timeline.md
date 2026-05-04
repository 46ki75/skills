# Timeline Reference

Timelines illustrate a chronology of events, dates, or periods of time arranged along a horizontal (or vertical) axis.

## Quick Start

```mermaid
timeline
    title History of Social Media Platform
    2002 : LinkedIn
    2004 : Facebook
         : Google
    2005 : YouTube
    2006 : Twitter
```

## Syntax

```text
timeline [direction]
    [title <text>]
    <time-period> : <event>
    <time-period> : <event> : <event>
```

- Start with the `timeline` keyword
- Optional `title` on the next line
- Each line begins with a time period followed by `:` and one or more events
- Time periods and events are plain text — not limited to numbers
- Multiple events for one period can be on the same line (`2004 : Facebook : Google`) or separate lines

## Sections

Group time periods into named sections:

```mermaid
timeline
    title Timeline of Industrial Revolution
    section 17th-20th century
        Industry 1.0 : Machinery, Water power, Steam <br>power
        Industry 2.0 : Electricity, Internal combustion engine, Mass production
        Industry 3.0 : Electronics, Computers, Automation
    section 21st century
        Industry 4.0 : Internet, Robotics, Internet of Things
        Industry 5.0 : Artificial intelligence, Big data, 3D printing
```

All periods within a section share a color scheme. Without sections, each time period gets its own color (default behavior).

## Direction

Control orientation with a direction keyword after `timeline` (v11.14.0+):

```mermaid
timeline TD
  title 2023 Timeline
    section Q1
      Bullet 1 : sub-point 1a : sub-point 1b
    section Q2
      Bullet 2 : sub-point 2a : sub-point 2b
```

**Options:** `LR` (left to right, default), `TD` (top to bottom)

## Text Wrapping

Long text wraps automatically. Use `<br>` to force a line break:

```mermaid
timeline
    section Stone Age
      6000 BC : Sea levels rise and Britain becomes an island.<br> The people who live here are hunter-gatherers.
```

## Styling

### Disabling Multi-Color

By default, each time period or section uses a distinct color. To use a single color scheme:

```mermaid
---
config:
  timeline:
    disableMulticolor: true
---
timeline
    title History of Social Media Platform
    2002 : LinkedIn
    2004 : Facebook : Google
    2005 : YouTube
```

### Custom Color Scheme

Override section colors using `cScale0` through `cScale11` theme variables. `cScaleLabel0`–`cScaleLabel11` control foreground colors:

```mermaid
---
config:
  theme: 'default'
  themeVariables:
    cScale0: '#ff0000'
    cScaleLabel0: '#ffffff'
    cScale1: '#00ff00'
    cScale2: '#0000ff'
    cScaleLabel2: '#ffffff'
---
timeline
    title History of Social Media Platform
    2002 : LinkedIn
    2004 : Facebook : Google
    2005 : YouTube
    2006 : Twitter
```

More than 12 sections repeat the color scheme.
