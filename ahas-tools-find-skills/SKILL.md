---
name: ahas-tools-find-skills
bundle: worker
description: Helps users discover and install agent skills. Use when they ask "how do I do X", "find a skill for X" or "is there a skill that can...", or look for functionality that might exist as an installable skill.
---

# Find Skills

This skill helps you discover skills in the open agent skills ecosystem and report the ones worth adopting.

## When to Use This Skill

Use this skill when the user:

- Asks "how do I do X" where X might be a common task with an existing skill
- Says "find a skill for X" or "is there a skill for X"
- Asks "can you do X" where X is a specialized capability
- Expresses interest in extending agent capabilities
- Wants to search for tools, templates, or workflows
- Mentions they wish they had help with a specific domain (design, testing, deployment, etc.)

## Where skills are found

The Skills CLI (`npx skills`) is the package manager for the open agent skills ecosystem. Skills are modular packages that extend agent capabilities with specialized knowledge, workflows, and tools. Browse them at https://skills.sh/.

- `npx skills find [query] [--owner <owner>]` searches interactively or by keyword, optionally scoped to a GitHub owner.
- **Nothing is installed or updated through it here.** `npx skills add` (with `-g` or without), `npx skills update` and `npx skills init` write a machine's or a project's skill folder; a worker's skills are the bmo-skills repository's own files. An adopted skill is written into its own folder `<name>/` of that repository by a job, under that repository's `CLAUDE.md` 「Writing a skill」 — a skill copied whole keeps its origin and licence in its frontmatter alone — and `~/.claude` is left alone.

## How to Help Users Find Skills

### Step 1: Understand What They Need

Identify:

1. The domain (e.g., React, testing, design, deployment)
2. The specific task (e.g., writing tests, creating animations, reviewing PRs)
3. Whether this is a common enough task that a skill likely exists

### Step 2: Leaderboard first, then search

Before running a CLI search, check the [skills.sh leaderboard](https://skills.sh/): it ranks skills by total installs, surfacing the most popular and battle-tested options. For web development, `vercel-labs/agent-skills` (React, Next.js, web design) and `anthropics/skills` (frontend design, document processing) each pass 100K installs.

If the leaderboard doesn't cover the need, run `npx skills find <query>` with the task's own words — "how do I make my React app faster?" → `npx skills find react performance`.

### Step 3: Verify Quality Before Recommending

**Do not recommend a skill based solely on search results.** Always verify:

1. **Install count** — Prefer skills with 1K+ installs. Be cautious with anything under 100.
2. **Source reputation** — Official sources (`vercel-labs`, `anthropics`, `microsoft`) are more trustworthy than unknown authors.
3. **GitHub stars** — Check the source repository. A skill from a repo with <100 stars should be treated with skepticism.

### Step 4: Report the options

For each relevant skill, give:

1. The skill name and what it does
2. The install count and source
3. A link to learn more at `https://skills.sh/<owner>/<repo>/<skill>`

Where one is to be adopted, it is written into this repository as above, never installed.

## Common Skill Categories

| Category        | Example Queries                          |
| --------------- | ---------------------------------------- |
| Web Development | react, nextjs, typescript, css, tailwind |
| Testing         | testing, jest, playwright, e2e           |
| DevOps          | deploy, docker, kubernetes, ci-cd        |
| Documentation   | docs, readme, changelog, api-docs        |
| Code Quality    | review, lint, refactor, best-practices   |
| Design          | ui, ux, design-system, accessibility     |
| Productivity    | workflow, automation, git                |

## Tips for Effective Searches

1. **Use specific keywords**: "react testing" is better than just "testing"
2. **Try alternative terms**: If "deploy" doesn't work, try "deployment" or "ci-cd"
3. **Check popular sources**: Many skills come from `vercel-labs/agent-skills` or `ComposioHQ/awesome-claude-skills`

## When No Skills Are Found

1. Say that no existing skill was found
2. Offer to help with the task directly using your general capabilities
3. Where the task recurs, suggest a skill of the bmo-skills repository's own for it, written into a folder of its own there by a job under that repository's `CLAUDE.md` 「Writing a skill」
