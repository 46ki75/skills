# Requirement Diagram Reference

Requirement diagrams visualize requirements and their connections to each other and to other documented elements, following SysML v1.6 modeling specs.

## Quick Start

```mermaid
requirementDiagram

requirement test_req {
id: 1
text: the test text.
risk: high
verifymethod: test
}

element test_entity {
type: simulation
}

test_entity - satisfies -> test_req
```

## Syntax

There are three component types: requirements, elements, and relationships.

User-defined text can be quoted or unquoted. Quoted text supports Markdown formatting.

### Requirements

```text
<type> user_defined_name {
    id: user_defined_id
    text: user_defined text
    risk: <risk>
    verifymethod: <method>
}
```

| Keyword | Options |
| ------- | ------- |
| Type | `requirement`, `functionalRequirement`, `interfaceRequirement`, `performanceRequirement`, `physicalRequirement`, `designConstraint` |
| Risk | `Low`, `Medium`, `High` |
| VerificationMethod | `Analysis`, `Inspection`, `Test`, `Demonstration` |

### Elements

```text
element user_defined_name {
    type: user_defined_type
    docref: user_defined_ref
}
```

Elements are lightweight and intended to connect requirements to portions of other documents.

### Relationships

```text
{source} - <type> -> {destination}
{destination} <- <type> - {source}
```

Relationship types: `contains`, `copies`, `derives`, `satisfies`, `verifies`, `refines`, `traces`

Each relationship is labeled in the diagram.

### Markdown Formatting

```mermaid
requirementDiagram

requirement "__test_req__" {
    id: 1
    text: "*italicized text* **bold text**"
    risk: high
    verifymethod: test
}
```

## Direction

Control layout with the `direction` statement:

```mermaid
requirementDiagram

direction LR

requirement test_req {
    id: 1
    text: the test text.
    risk: high
    verifymethod: test
}

element test_entity {
    type: simulation
}

test_entity - satisfies -> test_req
```

**Options:** `TB` (default), `BT`, `LR`, `RL`

## Styling

### Direct Styling

```mermaid
requirementDiagram

requirement test_req {
    id: 1
    text: styling example
    risk: low
    verifymethod: test
}

element test_entity {
    type: simulation
}

style test_req fill:#ffa,stroke:#000, color: green
style test_entity fill:#f9f,stroke:#333, color: blue
```

### Class Definitions

```text
classDef important fill:#f96,stroke:#333,stroke-width:4px
classDef test fill:#ffa,stroke:#000
```

**Default class:** A class named `default` applies to all nodes.

### Applying Classes

Apply with the `class` keyword:

```text
class test_req,test_entity important
```

Apply with `:::` shorthand during definition:

```text
requirement test_req:::important {
    id: 1
    text: class styling example
    risk: low
    verifymethod: test
}
```

Or after definition:

```text
test_elem:::myClass
```

## Full Example

```mermaid
requirementDiagram

requirement test_req {
id: 1
text: the test text.
risk: high
verifymethod: test
}

functionalRequirement test_req2 {
id: 1.1
text: the second test text.
risk: low
verifymethod: inspection
}

performanceRequirement test_req3 {
id: 1.2
text: the third test text.
risk: medium
verifymethod: demonstration
}

element test_entity {
type: simulation
}

element test_entity2 {
type: word doc
docRef: reqs/test_entity
}

test_entity - satisfies -> test_req2
test_req - traces -> test_req2
test_req - contains -> test_req3
test_req <- copies - test_entity2
```
