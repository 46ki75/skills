# Mindmap Reference

Mindmaps visually organize information into a hierarchy radiating from a central concept, with branches representing related ideas.

## Quick Start

```mermaid
mindmap
  root((mindmap))
    Origins
      Long history
      Popularisation
    Research
      On effectiveness
      On Automatic creation
    Tools
      Pen and paper
      Mermaid
```

## Syntax

Hierarchy is defined by indentation. Each level of indentation creates a child node relative to the previous shallower level.

```text
mindmap
    Root
        A
            B
            C
```

The exact indentation amount does not matter — only the relative indentation compared to adjacent rows.
When indentation is ambiguous, Mermaid uses the nearest ancestor with lesser indentation as the parent.

## Node Shapes

### Square

```mermaid
mindmap
    id[I am a square]
```

### Rounded Square

```mermaid
mindmap
    id(I am a rounded square)
```

### Circle

```mermaid
mindmap
    id((I am a circle))
```

### Bang

```mermaid
mindmap
    id))I am a bang((
```

### Cloud

```mermaid
mindmap
    id)I am a cloud(
```

### Hexagon

```mermaid
mindmap
    id{{I am a hexagon}}
```

### Default

```mermaid
mindmap
    I am the default shape
```

## Icons and Classes

### Icons

Add FontAwesome or Material Design icons using `::icon()` syntax on the line after the node:

```mermaid
mindmap
    Root
        A
        ::icon(fa fa-book)
        B(B)
        ::icon(mdi mdi-skull-outline)
```

Icon fonts must be available on the page — this is configured by the site administrator.

### Classes

Apply CSS classes with `:::` followed by space-separated class names:

```mermaid
mindmap
    Root
        A[A]
        :::urgent large
        B(B)
        C
```

Classes must be defined in the site's CSS.

## Markdown Strings

Use backtick strings for bold, italics, and automatic text wrapping:

```mermaid
mindmap
    id1["`**Root** with
a second line
Unicode works too: 🤓`"]
      id2["`The dog in **the** hog... a *very long text* that wraps to a new line`"]
      id3[Regular labels still works]
```

- Bold: `**text**`
- Italic: `*text*`
- Line breaks are automatic; newline characters also work

## Configuration

### Layouts

Use the tidy-tree layout for a more compact representation:

```mermaid
---
config:
  layout: tidy-tree
---
mindmap
root((mindmap))
  A
  B
  C
```
