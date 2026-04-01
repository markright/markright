export const EXAMPLE = `---
title: MarkRight Feature Tour
author: MarkRight Team
syntax: markright
---

# MarkRight Feature Tour

A complete tour of every syntax feature in MarkRight -- one unambiguous markup language distilled from 44 Markdown flavors.

## Inline Formatting

**Bold text**, *italic text*, ***bold italic text***, ~~strikethrough~~, ==highlighted text==, and \`inline code\`.

Superscript: x^2^ + y^2^ = z^2^. Subscript: H~2~O, CO~2~.

Inline math: $E = mc^2$, $\\sum_{i=1}^{n} i = \\frac{n(n+1)}{2}$.

## Links and Images

Standard links: [MarkRight on GitHub](https://github.com/markright/markright "MarkRight repository").

Bare URLs are auto-linked: https://markright.dev

Wikilinks: [[Getting Started]], [[Guide#installation|Install Guide]].

Images: ![MarkRight logo](https://via.placeholder.com/200x50 "Logo")

Wiki embeds with dimensions: ![[diagram.png|640x480]]

## Footnotes

Three footnote styles: regular[^1], range[^2>covering multiple words], and inline^[this footnote is defined right here].

[^1]: A regular footnote definition.

[^2]: A range footnote -- the superscript covers the annotated phrase.

## Citations

Cite sources with [@knuth1984, p. 42], multiple at once [@lamport1994; @knuth1984], or suppress the author with [-@knuth1984].

## Bracketed Spans

Apply attributes to inline content: [important text]{.highlight #key-point} or [styled]{.badge .primary data-tooltip=hover}.

## Code Blocks

\`\`\`rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
\`\`\`

\`\`\`typescript
interface Document {
  title: string;
  children: Block[];
  frontMatter?: string;
}

function parse(input: string): Document {
  const bump = new Bump();
  return markright.parse(input, bump);
}
\`\`\`

\`\`\`python
def quicksort(arr: list[int]) -> list[int]:
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)
\`\`\`

## Math Blocks

$$
\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}
$$

$$
\\mathbf{A} = \\begin{pmatrix} a_{11} & a_{12} \\\\ a_{21} & a_{22} \\end{pmatrix}
$$

## Lists

Unordered:

- First item
- Second item
- Third item

Ordered:

1. Step one
2. Step two
3. Step three

Auto-numbered:

#. First
#. Second
#. Third

## Task Lists

All eight task states:

- [ ] Open -- not yet started
- [x] Done -- completed
- [~] Active -- currently in progress
- [@] Review -- awaiting review
- [-] Cancelled -- no longer needed
- [!] Blocked -- waiting on a dependency
- [>] Deferred -- postponed to later
- [?] Question -- needs clarification

## Tables

| Feature | Syntax | Status |
| :--- | :---: | ---: |
| Bold | \`**text**\` | Done |
| Italic | \`*text*\` | Done |
| Strikethrough | \`~~text~~\` | Done |
| Highlight | \`==text==\` | Done |
| Superscript | \`x^2^\` | Done |
| Subscript | \`H~2~O\` | Done |
| Inline math | \`$E=mc^2$\` | Done |

Table: Inline formatting features and their syntax

## Blockquotes

> The best way to predict the future is to invent it.
>
> -- Alan Kay

## Admonitions

Five built-in admonition types:

N> **Note:** This is a note admonition. Use for supplementary information.

T> **Tip:** This is a tip. Use for helpful suggestions.

W> **Warning:** This is a warning. Use for potential pitfalls.

!> **Important:** This is important. Use for critical information.

X> **Caution:** This is a caution. Use for dangerous actions.

## Definition Lists

MarkRight
: A formally specified markup language
: Designed for both humans and AI agents

Parser
: A program that analyzes source text and produces a structured representation

AST
: Abstract Syntax Tree -- the structured output of parsing

## Fenced Divs

:::info
This is a generic fenced div with the class "info". Fenced divs use \`:::\` delimiters and can wrap any block content.
:::

## Diagram Blocks

\`\`\`mermaid
graph LR
    A[Source] --> B[Parser]
    B --> C[AST]
    C --> D[Renderer]
    D --> E[HTML]
\`\`\`

## Thematic Breaks

Content above the break.

---

Content below the break.

## Comments

%% This comment is invisible in the rendered output. %%

## Hard Breaks

Line one\\
Line two (hard break via trailing backslash)

## Headings with IDs

### Custom ID Example {#custom-id}

Link to it: [jump to custom ID](#custom-id).
`;
