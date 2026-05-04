# User Journey Diagram Reference

User journey diagrams map the experience of a user completing a task — showing steps, satisfaction scores, and which actors are involved.

## Basic Syntax

```mermaid
journey
    title My working day
    section Go to work
        Make tea: 5: Me
        Go upstairs: 3: Me
        Do work: 1: Me, Cat
    section Go home
        Go downstairs: 5: Me
        Sit down: 5: Me
```

Each task line follows the format: `Task description: score: Actor1, Actor2, ...`

## Structure

### title

Optional diagram title:

```text
title User Journey: Checkout Flow
```

### section

Group related steps under a named phase:

```text
section Phase Name
    Step 1: score: Actor
    Step 2: score: Actor
```

### Tasks

Each task entry has three parts:

```text
<description>: <score>: <actors>
```

- **description** — What the user is doing (free text)
- **score** — Satisfaction rating from 1 (very negative) to 5 (very positive)
- **actors** — Comma-separated list of participants (person, system, team, etc.)

**Score visual meaning:**

- `1`–`2` — Negative experience (red tones)
- `3` — Neutral
- `4`–`5` — Positive experience (green tones)

## Multiple Actors

Tasks can involve multiple actors, rendered as stacked blocks:

```mermaid
journey
    title Order Fulfillment
    section Customer Actions
        Browse catalog: 5: Customer
        Add to cart: 4: Customer
        Checkout: 3: Customer, Website
    section Backend Processing
        Validate payment: 4: Payment Service
        Allocate stock: 4: Warehouse System
        Confirm order: 5: Customer, Email Service
```

## Common Patterns

### E-Commerce Purchase Journey

```mermaid
journey
    title E-Commerce Purchase Journey
    section Discovery
        Search for product: 4: Customer
        Read reviews: 4: Customer
        Compare options: 3: Customer
    section Purchase
        Add to cart: 5: Customer
        Enter shipping details: 2: Customer
        Enter payment: 2: Customer, Payment Gateway
        Confirm order: 5: Customer
    section Fulfillment
        Order processed: 5: Warehouse
        Item shipped: 4: Logistics
        Delivery notification: 5: Customer, Email Service
        Item received: 5: Customer
```

### User Onboarding

```mermaid
journey
    title New User Onboarding
    section Sign Up
        Visit landing page: 5: Visitor
        Click sign up: 5: Visitor
        Fill in details: 3: User
        Verify email: 2: User, Email Service
    section First Use
        Complete profile: 3: User
        Tour product features: 4: User, App
        Create first project: 4: User
    section Activation
        Invite team member: 5: User
        Publish first result: 5: User
        Return next day: 4: User
```

### Support Request Journey

```mermaid
journey
    title Customer Support Flow
    section Problem Discovery
        Encounter issue: 1: Customer
        Search help docs: 2: Customer
        Docs don't help: 1: Customer
    section Contact Support
        Submit ticket: 3: Customer, Support Portal
        Receive acknowledgment: 3: Customer
        Wait for response: 2: Customer
    section Resolution
        Agent responds: 4: Customer, Support Agent
        Provide more info: 3: Customer, Support Agent
        Issue resolved: 5: Customer
        Rate experience: 5: Customer
```

## Tips

- Aim for 5–10 steps per section for readability
- Use scores honestly to highlight pain points — low scores show where to improve
- Include all actors that meaningfully participate in each step
- Sections should map to distinct phases of the user's experience
