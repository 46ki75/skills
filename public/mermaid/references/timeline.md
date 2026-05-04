# Timeline Reference

Timeline diagrams display events in chronological order — ideal for historical timelines, roadmaps, product evolution, and milestone tracking.

## Basic Syntax

```mermaid
timeline
    title History of Social Media Platform
    2002 : LinkedIn
    2004 : Facebook
         : Google
    2005 : YouTube
    2006 : Twitter
```

Multiple events on the same time point are stacked by repeating the `:` entry.

## Title

Add an optional title:

```text
title My Timeline
```

## Sections

Group time periods under section headings:

```mermaid
timeline
    title Product Milestones
    section 2022
        Q1 : Concept & Research
        Q2 : Prototype
        Q3 : Alpha Release
        Q4 : Beta Program
    section 2023
        Q1 : Public Launch
        Q2 : Mobile App
        Q3 : Enterprise Tier
        Q4 : International
```

Sections visually separate related periods with a distinct background color.

## Multiple Events Per Period

List multiple events under the same time label:

```mermaid
timeline
    title Company History
    2015 : Founded
         : Seed funding
    2016 : First customer
    2017 : Series A
         : Reached 100 users
         : Opened new office
    2018 : Series B
         : International expansion
```

## Themes and Styling

Apply global themes via frontmatter:

```mermaid
---
config:
  theme: base
---
timeline
    title Timeline with theme
    2020 : Event A
    2021 : Event B
    2022 : Event C
```

### disableMulticolor

By default, each time period gets a different color. Disable this for a uniform look:

```mermaid
---
config:
  timeline:
    disableMulticolor: true
---
timeline
    2020 : A
    2021 : B
    2022 : C
```

## Common Patterns

### Company History

```mermaid
timeline
    title Acme Corp History
    section Founding Era
        1998 : Company founded
             : First office in San Francisco
        1999 : Series A funding
    section Growth Phase
        2001 : Survived dot-com bust
        2003 : Reached profitability
        2005 : IPO
    section Modern Era
        2010 : Mobile pivot
        2015 : International expansion
        2020 : 10 million customers
        2024 : AI integration launched
```

### Technology Evolution

```mermaid
timeline
    title Web Development Evolution
    section Static Web
        1991 : HTML invented
        1994 : CSS introduced
        1995 : JavaScript created
    section Dynamic Web
        1995 : PHP released
        1998 : Google founded
        2004 : Gmail launches Ajax era
    section Modern Web
        2009 : Node.js released
        2013 : React introduced
        2014 : Vue.js released
        2016 : Next.js launched
    section AI-Augmented
        2022 : GitHub Copilot GA
        2023 : LLM-powered tools proliferate
```

### Product Roadmap

```mermaid
timeline
    title Product Roadmap 2024-2025
    section Q1 2024
        January : Auth redesign
        February : Dashboard v2
        March : API v3 launch
    section Q2 2024
        April : Mobile beta
        May : Enterprise SSO
        June : Public mobile launch
    section Q3 2024
        July : Analytics suite
        August : Integrations marketplace
        September : Performance milestone
    section 2025
        H1 : AI assistant features
        H2 : Platform expansion
```
