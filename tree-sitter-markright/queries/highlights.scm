; Headings
(heading (heading_marker) @markup.heading.marker)
(heading (inline) @markup.heading)
(block_id) @tag.attribute

; Code
(code_fence (fence_marker) @punctuation.delimiter)
(code_fence (language) @tag)
(inline_code) @markup.raw

; Math
(math_delimiter (math_marker) @punctuation.delimiter)
(inline_math) @markup.math

; Emphasis
(bold_italic) @markup.bold @markup.italic
(bold) @markup.bold
(italic) @markup.italic
(strikethrough) @markup.strikethrough
(superscript) @markup.superscript
(subscript) @markup.subscript

; Links
(link) @markup.link
(image) @markup.link
(wiki_link) @markup.link
(wiki_embed) @markup.link
(autolink) @markup.link.url

; Footnotes and citations
(footnote_ref) @markup.link
(inline_footnote) @markup.link
(citation) @markup.link
(footnote_def (footnote_marker) @punctuation.delimiter)

; Lists
(task_list_item (list_marker) @punctuation.delimiter)
(task_list_item (task_state) @constant)
(list_item_unordered (list_marker) @punctuation.delimiter)
(list_item_ordered (list_marker) @punctuation.delimiter)

; Table
(table_row (table_delimiter) @punctuation.delimiter)
(table_caption (caption_marker) @keyword)

; Blockquote
(blockquote_line (quote_marker) @punctuation.delimiter)

; Admonition
(admonition_line (admonition_marker) @keyword)

; Thematic break
(thematic_break (break_marker) @punctuation.delimiter)

; Comment
(comment_line (comment_content) @comment)

; Fenced div
(fenced_div_marker (div_marker) @punctuation.delimiter)
(fenced_div_marker (div_name) @tag)

; TOC
(toc (toc_marker) @function.macro)

; Include
(include (include_marker) @punctuation.delimiter)
(include (include_path) @string)

; Definition list
(definition_value (definition_marker) @punctuation.delimiter)

; Bracketed span
(bracketed_span) @markup.link

; Escape
(escape) @constant.character.escape
