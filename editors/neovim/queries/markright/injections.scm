; Inject language-specific highlighting for code blocks
((code_block
  (language) @injection.language
  (code) @injection.content))

; Inject YAML into front matter
((front_matter
  (metadata) @injection.content)
  (#set! injection.language "yaml"))

; Inject LaTeX into math blocks
((math_block
  (math) @injection.content)
  (#set! injection.language "latex"))
