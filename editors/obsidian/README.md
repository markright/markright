# MarkRight Obsidian Plugin

Adds support for `.right` files in Obsidian.

## Features

- Opens and edits `.right` files using the markdown editor
- Renders MarkRight syntax in reading view via WASM parser
- Styles for admonitions, task lists, and wikilinks

## Installation

1. Copy this plugin folder to your vault's `.obsidian/plugins/markright/`
2. Copy the WASM files from `crates/markright-wasm/pkg/`:
   - `markright_wasm.js`
   - `markright_wasm_bg.wasm`
3. Enable the plugin in Settings > Community Plugins

## Limitations

- Editing view uses Obsidian's markdown editor (not MarkRight-specific syntax highlighting)
- Wikilink resolution uses Obsidian's built-in resolution, not MarkRight's lookup tables
- Some MarkRight-specific features (citations, bracketed spans) fall back to markdown rendering in edit mode
