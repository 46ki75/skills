"""Prompt evaluation template using the Anthropic SDK.

Usage:
    export ANTHROPIC_API_KEY=...
    python python_eval.template.py
"""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from typing import Callable

from anthropic import Anthropic

CLIENT = Anthropic()
MODEL = "claude-haiku-4-5-20251001"


# --- prompt versions ---------------------------------------------------

def v1_prompt(item: dict) -> str:
    return (
        f"Classify the following customer complaint into one of: "
        f"Software Bug, Hardware Malfunction, User Error, "
        f"Feature Request, Service Outage. "
        f"Respond with only the category name and nothing else.\n\n"
        f"Complaint: {item['complaint']}\n"
        f"Classification:"
    )


def v2_prompt(item: dict) -> str:
    return (
        f"You are a customer support triage assistant. Classify the "
        f"complaint into one or more of: Software Bug, Hardware "
        f"Malfunction, User Error, Feature Request, Service Outage. "
        f"Prefer a single category when possible. Respond with only "
        f"the category name(s), comma-separated.\n\n"
        f"Complaint: {item['complaint']}\n"
        f"Classification:"
    )


# --- dataset ----------------------------------------------------------

EVAL_DATA: list[dict] = [
    {"complaint": "The app crashes when I upload a photo",
     "golden_answer": ["Software Bug"]},
    {"complaint": "My printer isn't recognized by my computer",
     "golden_answer": ["Hardware Malfunction"]},
    {"complaint": "I can't figure out how to change my password",
     "golden_answer": ["User Error"]},
    # ... add more
]


# --- grader -----------------------------------------------------------

def set_match(output: str, golden: list[str]) -> bool:
    predicted = {c.strip().lower() for c in output.split(",")}
    expected = {c.lower() for c in golden}
    return predicted == expected


# --- runner -----------------------------------------------------------

@dataclass
class Result:
    item: dict
    output: str
    passed: bool


def get_completion(prompt: str) -> str:
    resp = CLIENT.messages.create(
        model=MODEL,
        max_tokens=200,
        messages=[{"role": "user", "content": prompt}],
    )
    return resp.content[0].text


def run(prompt_fn: Callable[[dict], str], data: list[dict]) -> list[Result]:
    results = []
    for item in data:
        output = get_completion(prompt_fn(item))
        passed = set_match(output, item["golden_answer"])
        results.append(Result(item=item, output=output, passed=passed))
    return results


def report(label: str, results: list[Result]) -> None:
    passes = sum(r.passed for r in results)
    total = len(results)
    print(f"\n=== {label}: {passes}/{total} = {passes/total:.1%} ===")
    for r in results:
        if not r.passed:
            print(f"  FAIL: {r.item['complaint']!r}")
            print(f"    expected={r.item['golden_answer']}")
            print(f"    got     ={r.output!r}")


if __name__ == "__main__":
    v1 = run(v1_prompt, EVAL_DATA)
    v2 = run(v2_prompt, EVAL_DATA)
    report("v1", v1)
    report("v2", v2)
