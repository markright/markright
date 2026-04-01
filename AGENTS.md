# MarkRight

A markup language that distills the best from 44 Markdown flavors into one unambiguous, formally specified language.

## General

- If the developer become mad, write down what mistake you made to this doc, in similar bullet style.
- Be humble.
- Odds are, you will not know everything, and your knowledge cutoff may be aggressive.
- Always search the web whenever possible.
- Never use em-dash.
- NEVER ASSUME ANYTHING. You have an infinite exa, grep, context7 quota. Always online search.
- Use CLI to install stuff so that you get the latest versions.
- This application currently has no external installed user base; optimize for one canonical current-state implementation, not compatibility with historical local states.
- Do not preserve or introduce compatibility bridges, migration shims, fallback paths, compact adapters, or dual behavior for old local states unless the user explicitly asks for that support.
- Prefer:
  - one canonical current-state codepath
  - fail-fast diagnostics
  - explicit recovery steps
- over:
  - automatic migration
  - compatibility glue
  - silent fallbacks
  - "temporary" second paths
- If temporary migration or compatibility code is introduced for debugging or a narrowly scoped transition, it must be called out in the same diff with:
  - why it exists
  - why the canonical path is insufficient
  - exact deletion criteria
  - the ADR/task that tracks its removal
- Default stance across the app: delete old-state compatibility code rather than carrying it forward.
- On every edit, delete:
	- Extra comments that a human wouldn't add or is inconsistent with the rest of the file
	- Extra defensive checks or try/catch blocks that are abnormal for that area of the codebase (especially if called by trusted / validated codepaths)
	- Casts to any to get around type issues
	- Any other style that is inconsistent with the file

## Stack

### Parser (core)

- Language: **Rust**
- Crate location: `crates/markright`
- Hand-written two-phase parser (block scanning, then inline parsing)
- Zero dependencies for the core parser
- WASM build target for browser embedding

### Frontend Rules

- Use pnpm.
- Never use `setState` inside `useEffect`
- Never use `vh`, `vw`, `h-screen`, or similar viewport units. Use `h-full` with `flex-1`
- Never use arbitrary Tailwind values (bracket syntax like `w-[347px]`). Always use Tailwind default values
- Use shadcn components for everything; do not build custom UI primitives
- Use env.ts with zod and t3-oss/env for type-safe env vars.
- If you combine a shadow border (i.e. `ring` in tailwind), with a normal shadow, they'll blend it looks a lot nicer than a normal css `border` with a shadow.

### Task Runner

- Use `moon run` as the canonical task runner (e.g. `moon run :types`, `moon run markright:test`).
- Use pnpm for package installation (e.g. `pnpm install`, `pnpm add <pkg>`).

## Spec

- `spec/markright.right` -- the formal specification

## Key Design Decisions

- File extension: `.right` (canonical) or `.md` with `syntax: markright` front matter
- Bundle format: single file, directory bundle, or `.rightz` zip
- No inline HTML
- No extensions -- fixed spec
- 8 task list states: `[ ]`, `[x]`, `[~]`, `[@]`, `[-]`, `[!]`, `[>]`, `[?]`
- Admonitions use prefix syntax (`N>`, `W>`, `T>`, `!>`, `X>`), not fenced divs
- Fenced divs (`:::`) are for generic containers only
- `$` math delimiters only, not `\(...\)`

## Frontend

- Use Sunghyun Sans (https://github.com/anaclumos/sunghyun-sans) and JetBrains Mono
- Use pnpm.
- Use env.ts with zod and t3-oss/env for type-safe env vars.
- Very selectively use cards. NEVER nest cards.
- Never use vh, h-screen, etc. Use h-full with flex-1. Never use vw either.
- Never use arbitrary Tailwind values (ones with brackets). Always use Tailwind default values.
- Use es-toolkit for most lodash functions.
- Use ky for fetching.
- Better crashing than ducktaping `??` or `null`s.
  - Use latest Zod patterns for child access, instead of chaining.

### Use TanStack CLIs for advanced informations

- pnpx @tanstack/cli
- tanstack_list_libraries: tanstack libraries --json
- tanstack_doc: tanstack doc query framework/react/overview --json
- tanstack_search_docs: tanstack search-docs "server functions" --library start --json
- tanstack_ecosystem: tanstack ecosystem --category database --json

## MCPs and Skills

- You are encouraged to use MCPs and Skills whenever makes sense.
- If you want new MCPs or Skills, ask first.
  - All MCPs and Skills must be installed under this project; nothing should be installed globally.
- All MCPs should be added to all of .mcp.json, opencode.json, and .codex/config.toml.
- All Skills should be installed with `pnpx skills`.

## Secrets

- NEVER read or edit .env or .dev.vars directly!
- Do not read them directly.
- If you need to access them, either source the env and pass it as argument, or use cli to get the value.

## Types

- For type checking, use `moon run :types`
