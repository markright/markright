# MarkRight

A fixed markup language that distills the best of Markdown into one unambiguous, formally specified language.

## Repo Layout

```text
crates/
  markright/          Rust reference parser, formatter, linter, HTML renderer, CLI
  markright-lsp/      Language server (tower-lsp)
  markright-mcp/      MCP server for AI agents
  markright-wasm/     WASM build (wasm-pack)

tree-sitter-markright/  Tree-sitter grammar and highlight queries

editors/
  vscode/             VS Code extension (TextMate grammar + preview)
  neovim/             Neovim ftdetect, ftplugin, Tree-sitter queries
  zed/                Zed extension (Tree-sitter + LSP)
  jetbrains/          JetBrains TextMate bundle

packages/
  vite-plugin-markright/  Vite plugin (.right to HTML modules)

pandoc/
  reader.lua          Pandoc reader (MarkRight to Pandoc AST)
  markright.lua        Pandoc writer (Pandoc AST to MarkRight)

apps/
  playground/         Next.js playground (WASM + Shiki + KaTeX)
spec/markright.right  Formal specification
```

## Getting Started

Install toolchains with [proto](https://moonrepo.dev/proto):

```sh
proto use
```

Install JS dependencies:

```sh
pnpm install
```

## Commands

[Moon](https://moonrepo.dev/moon) is the task runner:

```sh
moon run markright:build    # cargo build (excludes WASM)
moon run markright:check    # cargo check --tests
moon run markright:lint     # cargo clippy --all-targets
moon run markright:fmt      # cargo fmt --check
moon run markright:test     # cargo test
```

Update fixture expectations (never runs in CI):

```sh
moon run markright:test-fixtures-update
```

Run all affected tasks (what CI does):

```sh
moon ci
```

Direct Cargo commands also work:

```sh
cargo build -p markright
cargo run -p markright -- path/to/file.right
cargo test -p markright
```

## Playground

```sh
pnpm --filter playground dev
```

## CLI

```sh
# Parse and print JSON AST
cargo run -p markright -- file.right

# Format
cargo run -p markright -- --format file.right

# Lint
cargo run -p markright -- --lint file.right
```

## Pandoc Integration

Convert MarkRight to any format via Pandoc:

```sh
pandoc -f pandoc/reader.lua input.right -o output.pdf
pandoc -f pandoc/reader.lua input.right -o output.docx
```

Convert Markdown to MarkRight:

```sh
pandoc -f markdown -t pandoc/markright.lua input.md
```

## Tests

1,597 fixture pairs under `crates/markright/tests/fixtures/`. Each `.test.right` file has a matching `.test.ast.json`. Set `UPDATE_FIXTURES=1` to rewrite expected output.

## Parser Surface

**Block:** YAML front matter, headings (`{#id}`), paragraphs, fenced code, diagram blocks (mermaid/dot/graphviz/plantuml), math blocks, thematic breaks, blockquotes, ordered/unordered/task lists, tables (with captions), admonitions, definition lists, fenced divs, footnote definitions, comments, TOC directives, includes.

**Inline:** escapes, hard breaks, italic, bold, bold italic, strikethrough, highlight, superscript, subscript, inline code, inline math, links, images, autolinks, wikilinks, wiki embeds, bracketed spans, footnote refs, range footnotes, inline footnotes, citations.
