# Flowchart Reference

Comprehensive guide for creating flowcharts with Mermaid syntax.

## Quick Start

```mermaid
flowchart LR
    Start --> Stop
```

**Direction options**: `LR` (left-right), `RL` (right-left), `TD`/`TB` (top-down), `BT` (bottom-top)

## Node Shapes

### Basic Shapes

```mermaid
flowchart TD
    A[Rectangle]
    B(Rounded)
    C([Stadium])
    D[[Subroutine]]
    E[(Database)]
    F((Circle))
    G>Asymmetric]
    H{Diamond}
    I{{Hexagon}}
    J(((Double Circle)))
```

**Syntax patterns**:
- `id[Text]` - Rectangle
- `id(Text)` - Rounded edges
- `id([Text])` - Stadium
- `id[[Text]]` - Subroutine
- `id[(Text)]` - Cylinder/Database
- `id((Text))` - Circle
- `id>Text]` - Asymmetric (flag)
- `id{Text}` - Diamond/Rhombus
- `id{{Text}}` - Hexagon
- `id(((Text)))` - Double circle

### Parallelograms and Trapezoids

```mermaid
flowchart TD
    A[/Parallelogram/]
    B[\Parallelogram Alt\]
    C[/Trapezoid\]
    D[\Trapezoid Alt/]
```

**Syntax patterns**:
- `id[/Text/]` - Parallelogram (lean right)
- `id[\Text\]` - Parallelogram alt (lean left)
- `id[/Text\]` - Trapezoid (base bottom)
- `id[\Text/]` - Trapezoid alt (base top)

### Extended Shapes (v11.3.0+)

Use `@{ shape: type, label: "text" }` syntax for 30+ additional shapes:

```mermaid
flowchart LR
    A@{ shape: rect, label: "Process" }
    B@{ shape: rounded, label: "Event" }
    C@{ shape: stadium, label: "Terminal" }
    D@{ shape: subproc, label: "Subprocess" }
    E@{ shape: cyl, label: "Database" }
    F@{ shape: circle, label: "Start" }
    G@{ shape: diamond, label: "Decision" }
    H@{ shape: hex, label: "Prepare" }
```

**Common extended shapes**:
- `rect` - Rectangle/Process
- `rounded` - Rounded rectangle/Event
- `stadium` - Stadium/Terminal point
- `subproc` - Subprocess (double border)
- `cyl` - Cylinder/Database
- `circle` - Circle/Start
- `diamond` - Diamond/Decision
- `hex` - Hexagon/Prepare conditional
- `lean-r` - Lean right/Input-Output
- `lean-l` - Lean left/Output-Input
- `trap-b` - Trapezoid base bottom/Priority action
- `trap-t` - Trapezoid base top/Manual operation
- `dbl-circ` - Double circle/Stop
- `odd` - Odd shape
- `docs` - Multiple documents
- `doc` - Document
- `procs` - Multiple processes
- `stored-data` - Stored data
- `lin-cyl` - Lined cylinder
- `lin-doc` - Lined document
- `notch-rect` - Notched rectangle
- `fork` - Fork/Join
- `curv-trap` - Curved trapezoid
- `div-rect` - Divided rectangle
- `div-proc` - Divided process
- `tag-doc` - Tagged document
- `tag-rect` - Tagged rectangle
- `hourglass` - Hourglass
- `comment` - Comment
- `brace-l` - Left brace
- `brace-r` - Right brace
- `braces` - Braces
- `bracks` - Brackets
- `crossroad` - Crossroad
- `win-pane` - Window pane
- `f-circ` - Filled circle
- `delay` - Delay
- `das` - Direct access storage
- `disk` - Disk storage
- `display` - Display
- `junction` - Junction
- `collate` - Collate
- `extract` - Extract
- `internal-storage` - Internal storage
- `lin-pred` - Lined predefined process
- `summary` - Summary
- `manual-file` - Manual file
- `manual-input` - Manual input
- `paper-tape` - Paper tape
- `sm-circ` - Small circle
- `start` - Start
- `stop` - Stop
- `processes` - Processes

## Links and Arrows

### Basic Links

```mermaid
flowchart LR
    A --> B
    C --- D
    E -.-> F
    G ==> H
```

**Link types**:
- `-->` - Arrow
- `---` - Line (no arrow)
- `-.->` - Dotted arrow
- `==>` - Thick arrow

### Link Text

```mermaid
flowchart LR
    A -->|text| B
    C ---|text| D
    E -.->|text| F
    G ==>|text| H
```

**Alternative syntax**:
- `A -- text --> B`
- `A -. text .-> B`
- `A == text ==> B`

### Multi-directional Arrows

```mermaid
flowchart LR
    A <--> B
    C o--o D
    E x--x F
    G <-.-> H
    I <==> J
```

**Bidirectional types**:
- `<-->` - Bidirectional arrow
- `o--o` - Circle edges
- `x--x` - Cross edges
- `<-.->` - Dotted bidirectional
- `<==>` - Thick bidirectional

### Multi-node Links

```mermaid
flowchart LR
    A --> B & C --> D
    E & F --> G & H
```

**Syntax**: `A --> B & C` connects A to both B and C

### Chaining Links

```mermaid
flowchart LR
    A --> B --> C --> D
```

### Edge IDs and Styling (v11.10.0+)

```mermaid
flowchart LR
    A e1@==> B
    A e2@--> C
    e1@{ curve: linear }
    e2@{ curve: natural }
```

**Curve styles**: `basis`, `bumpX`, `bumpY`, `cardinal`, `catmullRom`, `linear`, `monotoneX`, `monotoneY`, `natural`, `step`, `stepAfter`, `stepBefore`

## Text Formatting

### Unicode Text

```mermaid
flowchart LR
    id["Unicode ❤ supported"]
```

Use `"` to enclose unicode text.

### Markdown Formatting

```mermaid
---
config:
  flowchart:
    htmlLabels: false
---
flowchart LR
    markdown["`This **is** _Markdown_`"]
    newLines["`Line1
    Line 2
    Line 3`"]
```

Use double quotes and backticks `"` text `"` for markdown. Requires `htmlLabels: false` in config.

### Line Breaks

Use `<br>` or `<br/>` for line breaks in text:

```mermaid
flowchart TD
    A[First line<br>Second line]
```

## Subgraphs

```mermaid
flowchart TB
    c1-->a2
    subgraph one
      a1-->a2
    end
    subgraph two
      b1-->b2
    end
    subgraph three
      c1-->c2
    end
    one --> two
    three --> two
    two --> c2
```

**Syntax**:
```
subgraph title
  graph definition
end
```

**Subgraph direction** (v10.7.0+):
```
subgraph id [title]
  direction TB
  nodes...
end
```

## Styling

### Node Styling

```mermaid
flowchart LR
    id1(Start)-->id2(Stop)
    style id1 fill:#f9f,stroke:#333,stroke-width:4px
    style id2 fill:#bbf,stroke:#f66,stroke-width:2px,color:#fff,stroke-dasharray: 5 5
```

**Style properties**: `fill`, `stroke`, `stroke-width`, `color`, `stroke-dasharray`

### Classes

Define and apply style classes:

```mermaid
flowchart LR
    A:::someclass --> B
    classDef someclass fill:#f96
```

**Syntax**:
- Define: `classDef className fill:#f9f,stroke:#333,stroke-width:4px;`
- Apply inline: `id:::className`
- Apply separately: `class id1,id2 className;`
- Multiple classes: `classDef class1,class2 font-size:12pt;`

**Default class**: Class named `default` applies to all nodes without specific classes.

### Link Styling

```mermaid
flowchart LR
    A --> B
    B --> C
    linkStyle 0 stroke:#ff3,stroke-width:4px
    linkStyle 1 stroke:#0f0
```

**Syntax**: `linkStyle <index> stroke:#ff3,stroke-width:4px,color:red;`

Links are indexed from 0 in order of definition. Can style multiple: `linkStyle 1,2,7 color:blue;`

## FontAwesome Icons

```mermaid
flowchart TD
    B["fa:fa-twitter for peace"]
    B-->C[fa:fa-ban forbidden]
    B-->D(fa:fa-spinner)
```

**Syntax**: `fa:icon-name` or `fab:icon-name`, `fas:icon-name`, `far:icon-name`, `fal:icon-name`, `fad:icon-name`, `fak:icon-name` (custom)

Requires FontAwesome CSS or registered icon packs (v11.7.0+).

## Interactivity

### Click Events

```mermaid
flowchart LR
    A-->B
    B-->C
    click A callback "Tooltip"
    click B "https://example.com" "Link text" _blank
    click C href "https://example.com" _blank
```

**Syntax**:
- Callback: `click nodeId callback "tooltip"`
- Link: `click nodeId "url" "tooltip" _blank`
- Href: `click nodeId href "url" "tooltip" _blank`

## Comments

```mermaid
flowchart LR
    %% This is a comment
    A -- text --> B
```

Comments start with `%%` and continue to end of line.

## Configuration

### Renderer Selection

```
---
config:
  flowchart:
    defaultRenderer: "elk"
---
```

**Renderers**: `dagre` (default), `elk` (better for complex diagrams, v9.4+)

### Curve Style

```
---
config:
  flowchart:
    curve: stepBefore
---
```

### Width

```javascript
mermaid.flowchartConfig = {
  width: "100%"
}
```

## Critical Warnings

**Reserved word "end"**: Capitalize any letter in "end" or use workaround. `end` in lowercase breaks flowcharts.

**Circle/cross edges**: Starting a node ID with `o` or `x` after `---` creates special edges:
- `A---oB` creates circle edge
- `A---xB` creates cross edge

To avoid, add space (`A--- oB`) or capitalize (`A---OB`).

## Common Patterns

### Decision Tree

```mermaid
flowchart TD
    A{Decision} -->|Yes| B[Action 1]
    A -->|No| C[Action 2]
    B --> D[Result]
    C --> D
```

### Process Flow

```mermaid
flowchart LR
    Start([Start]) --> Process[Process]
    Process --> Decision{Check}
    Decision -->|Pass| End([End])
    Decision -->|Fail| Process
```

### System Architecture

```mermaid
flowchart TB
    User((User))
    subgraph Frontend
      A[Web App]
      B[Mobile App]
    end
    subgraph Backend
      C[API Gateway]
      D[Service Layer]
      E[(Database)]
    end
    User --> A & B
    A & B --> C
    C --> D
    D --> E
```
