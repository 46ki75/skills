# Mindmap Reference

Mindmaps visualize hierarchical information radiating from a central concept — ideal for brainstorming, knowledge mapping, and concept exploration.

## Basic Syntax

```mermaid
mindmap
  root((mindmap))
    Origins
      Long history
      Popularisation
        British popular psychology author Tony Buzan
    Research
      On effectiveness<br/>and features
      On Automatic creation
        Uses
            Creative techniques
            Strategic planning
            Argument mapping
    Tools
      Pen and paper
      Mermaid
```

The first non-whitespace line after `mindmap` is the root node. Indentation defines the hierarchy.

## Node Shapes

Control node appearance using delimiters:

| Shape | Syntax | Description |
| --- | --- | --- |
| Default | `text` | Rounded rectangle |
| Square | `[text]` | Rectangle |
| Rounded | `(text)` | Rounded edges |
| Circle | `((text))` | Circle |
| Bang | `)text(` | Explode/cloud shape |
| Cloud | `)text(` | Cloud |
| Hexagon | `{{text}}` | Hexagon |

```mermaid
mindmap
  root((Central Idea))
    Branch A[Square Node]
    Branch B(Rounded Node)
    Branch C((Circle Node))
    Branch D{{Hexagon Node}}
    Branch E)Bang Node(
```

## Icons

Add icons to nodes using `::icon()` syntax:

```mermaid
mindmap
  root((My Project))
    Planning
      ::icon(fa fa-calendar)
      Timeline
    Development
      ::icon(fa fa-code)
      Frontend
      Backend
    Deployment
      ::icon(fa fa-rocket)
      Staging
      Production
```

Requires FontAwesome to be loaded in the environment.

## Classes

Apply CSS classes to nodes for custom styling:

```mermaid
mindmap
  root((Topic))
    Node A:::important
    Node B:::highlight
    Node C
```

Define styles in your CSS:

```css
.important { fill: #f96; }
.highlight { stroke: #f00; }
```

## Line Breaks in Nodes

Use `<br>` or `<br/>` to create multiline labels:

```mermaid
mindmap
  root((Main Topic))
    Long<br/>Label Node
    Another Node<br/>with two lines
```

## Markdown Formatting

Use backtick-wrapped strings for Markdown inside nodes:

```mermaid
mindmap
  root("`**Bold Root**`")
    "`_Italic_ Branch`"
    "`~~Strikethrough~~`"
```

## Common Patterns

### Project Breakdown

```mermaid
mindmap
  root((Project Alpha))
    Goals
      Increase revenue
      Improve UX
      Reduce churn
    Team
      Engineering
        Frontend
        Backend
        DevOps
      Design
      Product
    Timeline
      Q1 Planning
      Q2 Development
      Q3 Launch
    Risks
      Resource constraints
      Technical debt
      Market competition
```

### Technology Stack

```mermaid
mindmap
  root((Tech Stack))
    Frontend
      Framework
        React
        Next.js
      Styling
        Tailwind CSS
        CSS Modules
      Testing
        Jest
        Playwright
    Backend
      Runtime
        Node.js
      Framework
        Express
        Fastify
      Database
        PostgreSQL
        Redis
    Infrastructure
      Cloud
        AWS
        Vercel
      CI/CD
        GitHub Actions
```

### Learning Map

```mermaid
mindmap
  root((Learn TypeScript))
    Basics
      Types
        Primitives
        Arrays
        Tuples
      Interfaces
      Enums
    Intermediate
      Generics
      Utility Types
      Decorators
    Advanced
      Type Guards
      Conditional Types
      Template Literal Types
    Tools
      TSConfig
      ts-node
      Vite
```
