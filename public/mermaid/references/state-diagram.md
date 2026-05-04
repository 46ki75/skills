# State Diagram Reference

State diagrams model how a system transitions between states, ideal for state machines, protocol flows, and lifecycle diagrams.

## Basic Syntax

```mermaid
stateDiagram-v2
    [*] --> Still
    Still --> [*]
    Still --> Moving
    Moving --> Still
    Moving --> Crash
    Crash --> [*]
```

Use `[*]` for start and end points. `stateDiagram-v2` is the current version (preferred over `stateDiagram`).

## States

### Simple States

```mermaid
stateDiagram-v2
    s1
    state "This is a state description" as s2
```

**Two ways to define states:**

- `stateId` - Simple state with ID as label
- `state "Label" as stateId` - State with custom label

### Composite States (Nested)

Group related states into a composite state:

```mermaid
stateDiagram-v2
    [*] --> First
    state First {
        [*] --> Second
        Second --> [*]
    }
```

**Rules:**

- Composite states can be nested to any depth
- Each composite state has its own `[*]` start/end
- Transitions can go between inner and outer states

### Fork and Join (Concurrency)

Model parallel state execution:

```mermaid
stateDiagram-v2
    state fork_state <<fork>>
    [*] --> fork_state
    fork_state --> State2
    fork_state --> State3

    state join_state <<join>>
    State2 --> join_state
    State3 --> join_state
    join_state --> [*]
```

**Syntax:** `state stateName <<fork>>` or `state stateName <<join>>`

### Choice (Conditional)

Model conditional branching:

```mermaid
stateDiagram-v2
    state if_state <<choice>>
    [*] --> IsPositive
    IsPositive --> if_state
    if_state --> False: if n < 0
    if_state --> True : if n >= 0
```

**Syntax:** `state stateName <<choice>>`

## Transitions

### Basic Transitions

```mermaid
stateDiagram-v2
    A --> B
```

### Labeled Transitions

```mermaid
stateDiagram-v2
    A --> B : Event or condition
```

**Syntax:** `stateA --> stateB : label`

## Notes

Add annotations to states:

```mermaid
stateDiagram-v2
    State1 --> State2
    note right of State1
        Notes can be added to states.
        Multiple lines allowed.
    end note
```

**Syntax:**

```text
note right of stateName
    Note content
end note
```

Or: `note left of stateName`

## Concurrency (Parallel Regions)

Divide composite states into concurrent regions with `--`:

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

Control diagram layout:

```mermaid
stateDiagram-v2
    direction LR
    [*] --> A
    A --> B
    B --> [*]
```

**Options:** `TB` (top-bottom, default), `BT`, `LR` (left-right), `RL`

## Comments

```mermaid
stateDiagram-v2
    %% This is a comment
    [*] --> A
```

## Styling

### classDef and class

Apply styles to states using class definitions:

```mermaid
stateDiagram-v2
    classDef badBadEvent fill:#f00,color:white,font-weight:bold
    [*] --> A
    A --> B
    B --> C:::badBadEvent
    C --> [*]
```

**Syntax:**

- Define: `classDef className fill:#f9f,stroke:#333`
- Apply inline: `stateName:::className`

## Common Patterns

### Order Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Pending
    Pending --> Processing : payment confirmed
    Processing --> Shipped : items packed
    Shipped --> Delivered : customer received
    Delivered --> [*]
    Processing --> Cancelled : out of stock
    Pending --> Cancelled : payment failed
    Cancelled --> [*]
```

### Authentication Flow

```mermaid
stateDiagram-v2
    [*] --> LoggedOut
    LoggedOut --> Authenticating : login()
    Authenticating --> LoggedIn : valid credentials
    Authenticating --> LoggedOut : invalid credentials
    LoggedIn --> LoggedOut : logout()
    LoggedIn --> SessionExpired : timeout
    SessionExpired --> LoggedOut : acknowledge
```

### Traffic Light

```mermaid
stateDiagram-v2
    direction LR
    [*] --> Red
    Red --> Green : timer
    Green --> Yellow : timer
    Yellow --> Red : timer
```
