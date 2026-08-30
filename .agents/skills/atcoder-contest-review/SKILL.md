---
name: atcoder-contest-review
description: Review solutions in this repository for a completed AtCoder contest by comparing them with the official problems, editorials, and useful accepted approaches. Use for requests such as "ABC473見直し", "ABC473を見直して", or requests to review an ABC/ARC/AGC contest; focus on improvements useful during an actual contest, not template length or code golf.
---

# AtCoder Contest Review

Review the user's solutions for the requested completed AtCoder contest. The goal is to improve future contest performance: simpler reasoning, faster implementation, fewer failure modes, and better choices of algorithms and data structures.

## Scope and safety

- Determine the contest identifier from the request, then locate its source directory and relevant files in this repository.
- Confirm from AtCoder that the contest has ended before analyzing its problems. If it is ongoing, do not analyze or solve the problems; briefly explain that AtCoder's generative-AI rules prohibit assistance during the contest.
- Treat the task as review-only unless the user explicitly asks to modify code. Follow repository instructions and approval requirements before any edit.
- Preserve the user's template conventions. Ignore generic template code, unused template imports, helper-library length, formatting, and purely cosmetic shortening unless they materially affect contest-time implementation or correctness.

## Research

- Read the local solution body for every solved problem in the requested contest. Distinguish problem-specific code from injected or reusable templates.
- Browse the official AtCoder problem statements and official editorials. Use other accepted submissions or user editorials only when they reveal a materially simpler approach, implementation, or invariant.
- Check constraints, intended complexity, edge cases, and whether the local implementation actually matches the intended invariant.
- Do not equate fewer lines with a better contest solution. Prefer ideas that reduce what the contestant must derive, type, debug, or remember under time pressure.

## Review criteria

For each solved problem, assess:

- correctness and complexity;
- whether the chosen algorithm is the simplest practical contest approach;
- unnecessary state, special cases, branches, or data structures;
- whether a standard observation, formulation, or Rust facility makes the implementation meaningfully easier or safer;
- naming or indexing only when it obscures an invariant or is likely to cause a bug;
- reusable lessons that transfer to future contests.

Do not recommend iterator chains, abstractions, external crates, or compressed syntax merely to reduce line count. A straightforward loop is often preferable when it is faster to write or easier to verify.

## Response

- Lead with an overall assessment and the highest-impact lessons.
- Review problems individually, but keep praise or commentary brief where the existing approach is already contest-optimal.
- For each suggested improvement, explain why it helps in an actual contest. Label low-impact stylistic alternatives as optional.
- Include short replacement snippets only when they clarify a materially better implementation pattern. Exclude template sections from snippets and discussion.
- Cite the official problem or editorial near claims derived from it, and link local files when useful.
- Do not edit the solutions as part of a review unless the user separately requests implementation and grants any approval required by repository instructions.
