# Entity Relationship Diagram Reference

ER diagrams model entities, their attributes, and the relationships between them — ideal for database schema design and data modeling.

## Basic Syntax

```mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
    CUSTOMER }|..|{ DELIVERY-ADDRESS : uses
```

## Entities and Relationships

### Relationship Syntax

Format: `ENTITY-A [relationship] ENTITY-B : label`

### Cardinality Symbols

| Value (left) | Value (right) | Meaning |
| --- | --- | --- |
| `\|o` | `o\|` | Zero or one |
| `\|\|` | `\|\|` | Exactly one |
| `}o` | `o{` | Zero or more |
| `}\|` | `\|{` | One or more |

**Combined in a relationship:**

```text
ENTITY-A ||--o{ ENTITY-B : label
```

- Left side uses the left-value symbol
- Right side uses the right-value symbol
- `--` solid line (identifying) or `..` dashed line (non-identifying)

**Examples:**

```mermaid
erDiagram
    PERSON ||--|| PASSPORT : has
    PERSON ||--o{ PHONE : owns
    PERSON }o--o{ COUNTRY : lives-in
```

### Identification

- `--` (two dashes) - Identifying relationship: child cannot exist without parent
- `..` (two dots) - Non-identifying relationship: child can exist independently

## Attributes

### Attribute Syntax

```mermaid
erDiagram
    CUSTOMER {
        string name
        string custNumber
        string sector
    }
    ORDER {
        int orderNumber
        string deliveryAddress
    }
    CUSTOMER ||--o{ ORDER : places
```

### Attribute Types

Common types (not enforced by Mermaid, for documentation only):

- `string`
- `int`
- `float`
- `boolean`
- `date`
- `datetime`
- `enum`

### Keys

Mark attributes as keys with a suffix keyword:

```mermaid
erDiagram
    CUSTOMER {
        string name PK "Customer's full name"
        int custNumber PK
        string sector FK
        string custEmail UK "Must be unique"
    }
```

**Key types:**

- `PK` - Primary key
- `FK` - Foreign key
- `UK` - Unique key

### Comments on Attributes

Add a quoted string after the type and optional key:

```mermaid
erDiagram
    CAR {
        string registrationNumber PK "License plate"
        string make "Manufacturer"
        string model "Model name"
    }
```

## Entity Names with Aliases

Use an alias to provide a human-readable label while keeping the internal ID short:

```mermaid
erDiagram
    p[Person] {
        string firstName
        string lastName
    }
    a["Customer Account"] {
        string email
    }
    p ||--o| a : has
```

## Common Patterns

### E-commerce Schema

```mermaid
erDiagram
    CUSTOMER {
        int id PK
        string name
        string email UK
    }
    ORDER {
        int id PK
        int customerId FK
        date orderDate
        string status
    }
    PRODUCT {
        int id PK
        string name
        float price
    }
    ORDER_ITEM {
        int orderId FK
        int productId FK
        int quantity
    }
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ ORDER_ITEM : contains
    PRODUCT ||--o{ ORDER_ITEM : "included in"
```

### Blog Schema

```mermaid
erDiagram
    USER {
        int id PK
        string username UK
        string email UK
    }
    POST {
        int id PK
        int authorId FK
        string title
        string content
        datetime publishedAt
    }
    COMMENT {
        int id PK
        int postId FK
        int authorId FK
        string body
    }
    TAG {
        int id PK
        string name UK
    }
    POST_TAG {
        int postId FK
        int tagId FK
    }
    USER ||--o{ POST : writes
    USER ||--o{ COMMENT : authors
    POST ||--o{ COMMENT : has
    POST }o--o{ TAG : "tagged with"
```

### Authentication Schema

```mermaid
erDiagram
    USER {
        int id PK
        string email UK
        string passwordHash
    }
    SESSION {
        string token PK
        int userId FK
        datetime expiresAt
    }
    ROLE {
        int id PK
        string name UK
    }
    USER_ROLE {
        int userId FK
        int roleId FK
    }
    USER ||--o{ SESSION : "authenticated by"
    USER }o--o{ ROLE : "assigned"
```
