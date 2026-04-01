/// <reference types="tree-sitter-cli/dsl" />

module.exports = grammar({
  name: "markright",

  extras: ($) => [],

  conflicts: ($) => [[$.bold, $.italic]],

  rules: {
    document: ($) => repeat(choice($.block, $._newline)),

    _newline: () => /\n/,

    block: ($) =>
      choice(
        $.heading,
        $.code_fence,
        $.math_delimiter,
        $.thematic_break,
        $.blockquote_line,
        $.admonition_line,
        $.task_list_item,
        $.list_item_unordered,
        $.list_item_ordered,
        $.table_row,
        $.table_caption,
        $.comment_line,
        $.fenced_div_marker,
        $.footnote_def,
        $.toc,
        $.include,
        $.definition_value,
        $.paragraph,
      ),

    // Heading: # ... ######
    heading: ($) =>
      seq(
        alias(token(prec(10, /#{1,6} /)), $.heading_marker),
        optional($.inline),
        optional($.block_id),
        $._newline,
      ),
    block_id: () => /\{#[\w-]+\}/,

    // Code fence (opening or closing -- content is unparsed lines between them)
    code_fence: ($) =>
      seq(
        alias(token(prec(10, /`{3,}/)), $.fence_marker),
        optional(alias(/[^\n]+/, $.language)),
        $._newline,
      ),

    // Math block delimiter ($$ on its own line)
    math_delimiter: ($) =>
      seq(alias(token(prec(10, /\s*\$\$\s*/)), $.math_marker), $._newline),

    // Thematic break: ---
    thematic_break: ($) =>
      seq(alias(token(prec(5, /\s*---\s*/)), $.break_marker), $._newline),

    // Blockquote line: > content
    blockquote_line: ($) =>
      seq(
        alias(token(prec(8, /> ?/)), $.quote_marker),
        optional($.inline),
        $._newline,
      ),

    // Admonition line: N> W> T> !> X> (optionally with -)
    admonition_line: ($) =>
      seq(
        alias(token(prec(9, /[NTWX!]>-? ?/)), $.admonition_marker),
        optional($.inline),
        $._newline,
      ),

    // Task list: - [ ] - [x] etc.
    task_list_item: ($) =>
      seq(
        alias(token(prec(7, "- [")), $.list_marker),
        alias(/[ x~@\-!>?]/, $.task_state),
        alias("] ", $.list_marker),
        optional($.inline),
        $._newline,
      ),

    // Unordered list: - item
    list_item_unordered: ($) =>
      seq(
        alias(token(prec(3, "- ")), $.list_marker),
        optional($.inline),
        $._newline,
      ),

    // Ordered list: 1. item or #. item
    list_item_ordered: ($) =>
      seq(
        alias(token(prec(3, /(?:\d+|#)\. /)), $.list_marker),
        optional($.inline),
        $._newline,
      ),

    // Table row: | cell | cell |
    table_row: ($) =>
      seq(
        alias(token(prec(4, "|")), $.table_delimiter),
        repeat1(seq(optional(/[^|\n]+/), alias("|", $.table_delimiter))),
        $._newline,
      ),

    // Table caption: Table: text
    table_caption: ($) =>
      seq(
        alias(token(prec(4, "Table: ")), $.caption_marker),
        $.inline,
        optional($.block_id),
        $._newline,
      ),

    // Comment: %% text %%  or  %% (multiline delimiter)
    comment_line: ($) =>
      seq(alias(token(prec(10, /%%[^\n]*/)), $.comment_content), $._newline),

    // Fenced div marker: ::: or :::name
    fenced_div_marker: ($) =>
      seq(
        alias(token(prec(6, /:{3,}/)), $.div_marker),
        optional(alias(/\w+/, $.div_name)),
        $._newline,
      ),

    // Footnote definition: [^label]: text
    footnote_def: ($) =>
      seq(
        alias(token(prec(6, /\[\^[\w-]+\]: /)), $.footnote_marker),
        optional($.inline),
        $._newline,
      ),

    // TOC: [TOC] or [TOC:1-3]
    toc: ($) =>
      seq(
        alias(token(prec(6, /\s*\[TOC(?::\d-\d)?\]/)), $.toc_marker),
        $._newline,
      ),

    // Include: {{path}} or {{path#fragment}}
    include: ($) =>
      seq(
        alias(token(prec(6, "{{" )), $.include_marker),
        alias(/[^}\n]+/, $.include_path),
        alias("}}", $.include_marker),
        $._newline,
      ),

    // Definition value: : text
    definition_value: ($) =>
      seq(
        alias(token(prec(2, ": ")), $.definition_marker),
        $.inline,
        $._newline,
      ),

    // Paragraph: fallback for any line with inline content
    paragraph: ($) => prec(-1, seq($.inline, $._newline)),

    // Inline content
    inline: ($) =>
      repeat1(
        choice(
          $.bold_italic,
          $.bold,
          $.italic,
          $.strikethrough,
          $.highlight,
          $.inline_code,
          $.inline_math,
          $.wiki_embed,
          $.wiki_link,
          $.image,
          $.link,
          $.footnote_ref,
          $.inline_footnote,
          $.citation,
          $.bracketed_span,
          $.superscript,
          $.subscript,
          $.escape,
          $.autolink,
          $.text,
        ),
      ),

    bold_italic: ($) =>
      prec(3, seq("***", repeat1(choice($.inline_code, $.text)), "***")),
    bold: ($) =>
      prec(2, seq("**", repeat1(choice($.italic, $.inline_code, $.text)), "**")),
    italic: ($) =>
      prec(1, seq("*", repeat1(choice($.inline_code, $.text)), "*")),
    strikethrough: ($) => seq("~~", repeat1($.text), "~~"),
    highlight: ($) => seq("==", repeat1($.text), "=="),
    superscript: () => /\^[^\s^]+\^/,
    subscript: () => /~[^\s~]+~/,

    inline_code: () => /`[^`\n]+`/,
    inline_math: () => /\$[^$\n]+\$/,

    link: ($) =>
      seq("[", repeat1(choice($.escape, /[^\]\n]+/)), "](", /[^)\n]*/, ")"),
    image: ($) => seq("![", /[^\]\n]*/, "](", /[^)\n]*/, ")"),
    wiki_link: () => /\[\[[^\]]+\]\]/,
    wiki_embed: () => /!\[\[[^\]]+\]\]/,

    footnote_ref: () => /\[\^[\w-]+\]/,
    inline_footnote: ($) => seq("^[", repeat1($.text), "]"),
    citation: () => /\[@[^\]]+\]/,
    bracketed_span: ($) =>
      seq("[", repeat1(choice($.escape, /[^\]\n]+/)), "]{", /[^}\n]*/, "}"),

    autolink: () => /https?:\/\/[^\s)\]>]+/,
    escape: () => /\\./,
    text: () => /[^\n*~=`$\[!\]^\\{}]+|./,
  },
});
