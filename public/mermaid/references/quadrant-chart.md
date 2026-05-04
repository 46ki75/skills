# Quadrant Chart Reference

Quadrant charts plot items on an X-Y grid divided into four quadrants — ideal for priority matrices, risk assessments, and strategic analysis (e.g., BCG matrix, effort/impact grids).

## Basic Syntax

```mermaid
quadrantChart
    title Reach and engagement of campaigns
    x-axis Low Reach --> High Reach
    y-axis Low Engagement --> High Engagement
    quadrant-1 We should expand
    quadrant-2 Need to promote
    quadrant-3 Re-evaluate
    quadrant-4 May be improved
    Campaign A: [0.3, 0.6]
    Campaign B: [0.45, 0.23]
    Campaign C: [0.57, 0.69]
    Campaign D: [0.78, 0.34]
    Campaign E: [0.40, 0.34]
    Campaign F: [0.35, 0.78]
```

## Axes

### Axis Labels

Define axis labels with optional directional arrows:

```text
x-axis <left label> --> <right label>
y-axis <bottom label> --> <top label>
```

Or omit arrows for simple labels:

```text
x-axis "Label"
y-axis "Label"
```

## Quadrant Labels

Label each quadrant (optional):

```text
quadrant-1 <top-right label>
quadrant-2 <top-left label>
quadrant-3 <bottom-left label>
quadrant-4 <bottom-right label>
```

## Points

Plot points with coordinates in the range `[0, 1]`:

```text
Point Name: [x, y]
```

- `x = 0` is the left edge; `x = 1` is the right edge
- `y = 0` is the bottom edge; `y = 1` is the top edge
- The quadrant midpoint is `[0.5, 0.5]`

### Point Styling (v11.x+)

Customize individual point appearance:

```mermaid
quadrantChart
    x-axis Low --> High
    y-axis Low --> High
    Point A: [0.3, 0.7] radius: 10
    Point B: [0.7, 0.7] color: #ff0000, radius: 15, stroke-color: #000
    Point C: [0.5, 0.3] color: #00ff00, stroke-width: 2px
```

**Style attributes:**

- `radius` - Circle size in pixels
- `color` - Fill color (hex or name)
- `stroke-color` - Border color
- `stroke-width` - Border width (e.g., `2px`)

## Configuration

Control layout and appearance via frontmatter:

```mermaid
---
config:
  quadrantChart:
    chartWidth: 400
    chartHeight: 400
    pointRadius: 5
    pointLabelFontSize: 14
    quadrantPadding: 5
    xAxisLabelPadding: 10
    yAxisLabelPadding: 10
    quadrantLabelFontSize: 16
    quadrantTextFill: "#999"
    pointFill: "#4f9da6"
    pointTextFill: "#fff"
---
quadrantChart
    x-axis Low --> High
    y-axis Low --> High
    Point A: [0.3, 0.7]
```

## Common Patterns

### Effort vs. Impact (Prioritization Matrix)

```mermaid
quadrantChart
    title Feature Prioritization
    x-axis Low Effort --> High Effort
    y-axis Low Impact --> High Impact
    quadrant-1 Schedule it
    quadrant-2 Do it now
    quadrant-3 Drop it
    quadrant-4 Delegate it
    SSO Login: [0.2, 0.9]
    Dark Mode: [0.3, 0.5]
    New Dashboard: [0.7, 0.8]
    Email Reports: [0.5, 0.4]
    Data Export: [0.4, 0.7]
    Performance Tuning: [0.8, 0.9]
    Onboarding Flow: [0.6, 0.6]
```

### Risk Matrix

```mermaid
quadrantChart
    title Risk Assessment
    x-axis Low Probability --> High Probability
    y-axis Low Impact --> High Impact
    quadrant-1 Critical (Mitigate)
    quadrant-2 Monitor (Watch)
    quadrant-3 Accept (Low priority)
    quadrant-4 Contingency Plan
    Data breach: [0.3, 0.95]
    Server outage: [0.4, 0.8]
    Key staff leaving: [0.5, 0.6]
    Scope creep: [0.75, 0.5]
    Delayed deliveries: [0.6, 0.3]
    Budget overrun: [0.55, 0.65]
```

### BCG Growth-Share Matrix

```mermaid
quadrantChart
    title BCG Portfolio Analysis
    x-axis Low Market Share --> High Market Share
    y-axis Low Market Growth --> High Market Growth
    quadrant-1 Stars
    quadrant-2 Question Marks
    quadrant-3 Dogs
    quadrant-4 Cash Cows
    Product A: [0.75, 0.8]
    Product B: [0.2, 0.75]
    Product C: [0.8, 0.3]
    Product D: [0.25, 0.2]
    Product E: [0.55, 0.55]
```
