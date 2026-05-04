# State Diagram Reference

State diagrams describe system behavior through states and the transitions between them.

## Quick Start

```mermaid
---
title: Simple sample
---
stateDiagram-v2
    [*] --> Still
    Still --> [*]

    Still --> Moving
    Moving --> Still
    Moving --> Crash
    Crash --> [*]
```

Use `stateDiagram-v2` for the current renderer. The older `stateDiagram` keyword uses a legacy renderer.

## States

### Defining States

States can be defined by ID alone:

```mermaid
stateDiagram-v2
    stateId
```

Or with a description using the `state` keyword:

```mermaid
stateDiagram-v2
    state "This is a state description" as s2
```

Or using `id : description` syntax:

```mermaid
stateDiagram-v2
    s2 : This is a state description
```

## Transitions

Transitions are represented with `-->`. When states referenced in a transition are not yet defined, they are created implicitly:

```mermaid
stateDiagram-v2
    s1 --> s2
    s1 --> s2: A transition
```

## Start and End

`[*]` represents start and end states. The direction of the transition determines whether it is a start or stop:

```mermaid
stateDiagram-v2
    [*] --> s1
    s1 --> [*]
```

## Composite States

Composite states contain internal states defined within `{}`:

```mermaid
stateDiagram-v2
    [*] --> First
    state First {
        [*] --> second
        second --> [*]
    }
```

Composite states support multiple nesting levels. Transitions between internal states of different composite states are not allowed.

## Choice

Model branching with `<<choice>>`:

```mermaid
stateDiagram-v2
    state if_state <<choice>>
    [*] --> IsPositive
    IsPositive --> if_state
    if_state --> False: if n < 0
    if_state --> True : if n >= 0
```

## Forks and Joins

Represent parallel execution with `<<fork>>` and `<<join>>`:

```mermaid
stateDiagram-v2
    state fork_state <<fork>>
    [*] --> fork_state
    fork_state --> State2
    fork_state --> State3

    state join_state <<join>>
    State2 --> join_state
    State3 --> join_state
    join_state --> State4
    State4 --> [*]
```

## Notes

```mermaid
stateDiagram-v2
    State1: The state with a note
    note right of State1
        Important information! You can write
        notes.
    end note
    State1 --> State2
    note left of State2 : This is the note to the left.
```

## Concurrency

Use `--` to separate concurrent regions within a composite state:

```mermaid
stateDiagram-v2
    [*] --> Active

    state Active {
        [*] --> NumLockOff
        NumLockOff --> NumLockOn : EvNumLockPressed
        NumLockOn --> NumLockOff : EvNumLockPressed
        --
        [*] --> CapsLockOff
        CapsLockOff --> CapsLockOn : EvCapsLockPressed
        CapsLockOn --> CapsLockOff : EvCapsLockPressed
    }
```

## Direction

```mermaid
stateDiagram-v2
    direction LR
    [*] --> A
    A --> B
    B --> C
```

**Options:** `TB` (default), `BT`, `LR`, `RL`

Direction can also be set per composite state.

## Comments

Lines prefaced with `%%` are ignored by the parser:

```mermaid
stateDiagram-v2
    [*] --> Still
    Still --> [*]
%% this is a comment
    Still --> Moving
    Moving --> Crash %% inline comment
```

## Styling with classDefs

Define styles with `classDef` and apply to states:

```text
classDef movement font-style:italic
classDef badBadEvent fill:#f00,color:white,font-weight:bold,stroke-width:2px,stroke:yellow
```

**Limitations:** classDefs cannot be applied to start/end states or within composite states.

**Apply with `class` statement:**

```text
class Moving, Crash movement
class Crash badBadEvent
```

**Apply with `:::` operator:**

```mermaid
stateDiagram-v2
    Moving:::movement --> Crash:::badBadEvent
    classDef movement font-style:italic
    classDef badBadEvent fill:#f00,color:white
```
