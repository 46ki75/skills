---
name: information-retrieval-policy
description: >
  Policy for sourcing factual information before answering — invoke before
  `web_search`, any MCP lookup, or other external retrieval, and before
  answering from memory on potentially stale topics. Covers tool-selection
  priority (specialized tools → targeted retrieval → general web search),
  source-authority hierarchy (primary sources like official docs, standards,
  and release notes vs. user-driven sources like Stack Overflow, blogs, and
  GitHub issues), and the rubric for deciding whether a question is
  answerable from training data (stable: history, math, specs, definitions,
  classical CS) or needs a fresh lookup (library versions, prices, current
  officeholders, slang, ongoing events, third-party API behavior). Always
  invoke for factual questions about current state, software versions,
  third-party APIs, pricing, personnel, or anything time-sensitive — even
  if you think you know. Do NOT invoke for creative tasks, opinions, or
  chitchat where no external fact is at stake.
license: MIT
metadata:
  author: "Ikuma Yamashita"
  version: "1.0.0"
---

# Information Retrieval Policy

A policy for how to source factual information: when to look it up, what
tool to use, which sources to trust, and how to report the result.

## When this skill applies

You are about to answer a question that depends on factual information.
Before producing the answer, follow the decision flow below. The goal is
to avoid three common failures:

1. **Over-searching** — calling `web_search` for questions answerable
   from training data, wasting tokens and latency.
2. **Under-searching** — answering from stale internal knowledge for
   fluid topics, producing confidently wrong answers.
3. **Wrong source hierarchy** — leaning too heavily on official docs
   (missing practical context) or trusting forums uncritically.

## Decision flow

### Step 1 — Classify the knowledge type

Ask: is the needed information stable or fluid?

**Stable** (answer from internal knowledge):

- Historical events, dates, established biographies of deceased figures
- Mathematics, logic, fundamental scientific principles
- Definitions, well-established language specifications, dead languages
- Algorithms, data structures, classical computer science

**Fluid** (must verify with external sources):

- Software versions, library APIs, framework features
- Current officeholders, company leadership, organizational status
- Prices, exchange rates, market data
- Recent events, ongoing situations, news
- Slang, memes, trending terminology
- Third-party API behavior, cloud service features
- Anything where "as of [date]" would change the answer

**Tiebreaker**: if you cannot confidently classify, treat as fluid and
verify. The asymmetry of cost favors this default: a false-stable
classification produces a confidently wrong answer (high cost), while
a false-fluid classification produces one extra tool call (low cost).

### Step 2 — Select the tool

Prefer tools in this order:

1. **Specialized tools** — MCP servers, domain-specific APIs, other
   skills matching the topic (e.g., a sports-data MCP for game scores,
   a product-knowledge skill for vendor specifics, `context7` for
   library documentation).
2. **Targeted retrieval** — documentation fetchers, repo search,
   internal knowledge bases.
3. **General web search** — only as a fallback when no specialized tool
   fits.

Rationale: specialized tools return structured, authoritative data with
less hallucination risk than open web results.

### Step 3 — Select the source

Authority hierarchy:

1. **Primary sources** — official documentation, standards (RFC, W3C,
   ISO), peer-reviewed papers, government publications, vendor release
   notes, source code.
2. **User-driven sources** — Stack Overflow, technical blogs, GitHub
   issues and discussions, conference talks, well-known practitioner
   writing.
3. **General secondary sources** — news aggregators, Wikipedia,
   tutorial sites.
4. **Unverified sources** — random forums, social media posts.

**Practical pattern**: user-driven sources are often more digestible
and address the exact question being asked. It is acceptable — often
preferable — to consult them first for orientation, then verify the
specific claims against a primary source before stating them as fact.
This matches how experienced engineers actually research problems.

**Exception for high-stakes domains**: for medical, legal, financial,
or security topics, go directly to primary sources. User-driven content
may only supplement, never substitute. The asymmetric harm profile of
these domains justifies the slower research cost.

## Output requirements

- **Cite sources** for any claim derived from external retrieval.
- **Distinguish facts from inferences.** Use phrasing like "the docs
  state X" vs. "based on X, Y likely follows."
- **State gaps openly.** If retrieval failed to answer part of the
  question, say so rather than filling with plausible-sounding text.
- **Mark freshness** when relevant: "as of [date the source was
  published]."

## Structured output (for agent pipelines)

When the retrieval result will feed a downstream task, return:

- **Conclusion** — the direct answer.
- **Evidence** — the specific facts retrieved.
- **Sources** — URLs or identifiers.
- **Confidence** — high / medium / low, with reason.
- **Open questions** — anything unresolved.

This format makes the output consumable by subsequent agent steps
without re-parsing prose.

## What this skill does not cover

- Creative writing or opinion synthesis.
- Tasks where the user has explicitly provided all needed context.
- Conversational exchanges with no factual stakes.

## Quick reference

| Probe                                                        | Expected behavior                                   |
| ------------------------------------------------------------ | --------------------------------------------------- |
| "What's the capital of France?"                              | Stable; answer from memory.                         |
| "What's the latest version of Next.js?"                      | Fluid; verify via docs fetcher or web search; cite. |
| "How do I do X in Postgres?" (with a Postgres MCP available) | Prefer the Postgres MCP over general search.        |
| "Explain how garbage collection works."                      | Stable CS concept; answer from memory.              |
| "Is Sam Altman still CEO of OpenAI?"                         | Fluid (current officeholder); verify.               |
| "Write me a haiku about autumn."                             | Creative; this skill does not apply.                |
