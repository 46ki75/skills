---
Slug: my-slug-here
Title: My Title Here
Event Date: YYYY-MM-DDTHH:MM:SSZ
Severity: Low | Medium | High
Status: Not Started | In Progress | Resolved | Resolved (Workaround) | Resolved (Cause Unknown)
---

## Description

Brief summary of the problem from the user's perspective — what they experience
and when it occurs. State the symptom in concrete, searchable terms.

## Service Affected

Identify which service, application, system, or infrastructure component is
impacted. Include component names, repositories, and ownership where relevant.

## Context (environment, stack, version)

Specify the relevant environment (prod / staging / dev), region, runtime/stack,
dependencies, and exact version numbers where the problem was observed. Include
configuration flags or feature toggles if applicable. For third-party issues,
the affected version range is REQUIRED.

## Workaround

Step-by-step temporary solution that allows users to continue working while a
permanent fix is being developed. Must be clear enough for service desk staff
to execute or explain to users. If no workaround exists yet, state that
explicitly and link to investigation status.

## Root Cause

The underlying technical reason why the problem occurs. Include investigation
findings, references to issues/PRs, and why the workaround is necessary. If
the root cause is unknown, set `Status` to `Resolved (Cause Unknown)` (if the
symptom no longer occurs) or `In Progress` (if investigation continues), and
document what has been ruled out.

## Solution

The permanent fix that will eliminate the problem entirely. Include
implementation status, timeline, and what will change. For third-party
issues, record the fixed version. If no permanent fix is planned, state that
and explain why the workaround is the long-term answer.
