-- Pandoc custom writer for MarkRight
-- Usage: pandoc -t markright.lua input.md -o output.right

local layout = pandoc.layout
local literal = layout.literal
local empty = layout.empty
local cr = layout.cr
local concat = layout.concat
local blankline = layout.blankline

local footnotes = {}

local function escape(s)
  return (s:gsub("[\\`%$%*%[%]%^~=!]", "\\%1"))
end

local function escape_autolink(s)
  if s:match("^https?://") then
    return "\\" .. s
  end
  return escape(s)
end

-- Compute backtick fence that doesn't conflict with content
local function backtick_fence(body, min)
  local max_run = 0
  local run = 0
  for i = 1, #body do
    if body:sub(i, i) == "`" then
      run = run + 1
      if run > max_run then max_run = run end
    else
      run = 0
    end
  end
  local n = math.max(max_run + 1, min or 3)
  return string.rep("`", n)
end

local function inlines(ils)
  local result = {}
  for i = 1, #ils do
    local el = ils[i]
    local handler = Inlines[el.tag]
    if handler then
      table.insert(result, handler(el))
    else
      table.insert(result, literal(pandoc.utils.stringify(el)))
    end
  end
  return concat(result)
end

local function blocks(bs, sep)
  local result = {}
  for i = 1, #bs do
    local el = bs[i]
    local handler = Blocks[el.tag]
    if handler then
      table.insert(result, handler(el))
    else
      table.insert(result, literal(pandoc.utils.stringify(el)))
    end
  end
  return concat(result, sep or blankline)
end

local function attr_id(attr)
  if attr and attr.identifier and attr.identifier ~= "" then
    return literal(" {#" .. attr.identifier .. "}")
  end
  return empty
end

-- Inline renderers
Inlines = {}

Inlines.Str = function(el)
  return literal(escape_autolink(el.text))
end

Inlines.Space = function()
  return layout.space
end

Inlines.SoftBreak = function()
  return layout.space
end

Inlines.LineBreak = function()
  return literal("\\\n")
end

Inlines.Emph = function(el)
  return concat{"*", inlines(el.content), "*"}
end

Inlines.Strong = function(el)
  return concat{"**", inlines(el.content), "**"}
end

Inlines.Strikeout = function(el)
  return concat{"~~", inlines(el.content), "~~"}
end

Inlines.Superscript = function(el)
  return concat{"^", inlines(el.content), "^"}
end

Inlines.Subscript = function(el)
  return concat{"~", inlines(el.content), "~"}
end

Inlines.Underline = function(el)
  return inlines(el.content)
end

Inlines.SmallCaps = function(el)
  return inlines(el.content)
end

Inlines.Code = function(el)
  local fence = backtick_fence(el.text, 1)
  return literal(fence .. el.text .. fence)
end

Inlines.Math = function(el)
  if el.mathtype == "InlineMath" then
    return literal("$" .. el.text .. "$")
  else
    local body = el.text:match("^%s*(.-)%s*$")
    return layout.nowrap(literal("$$\n" .. body .. "\n$$"))
  end
end

Inlines.Link = function(el)
  local text = inlines(el.content)
  local url = el.target
  if el.title and el.title ~= "" then
    return concat{"[", text, "](", literal(url), " \"", literal(el.title), "\")"}
  end
  return concat{"[", text, "](", literal(url), ")"}
end

Inlines.Image = function(el)
  local alt = pandoc.utils.stringify(el.caption)
  local url = el.src
  if el.title and el.title ~= "" then
    return literal("![" .. alt .. "](" .. url .. " \"" .. el.title .. "\")")
  end
  return literal("![" .. alt .. "](" .. url .. ")")
end

Inlines.Note = function(el)
  local n = #footnotes + 1
  local label = tostring(n)
  footnotes[n] = el.content
  return literal("[^" .. label .. "]")
end

Inlines.Quoted = function(el)
  local q = pandoc.utils.stringify(el)
  return literal(escape(q))
end

Inlines.Cite = function(el)
  local parts = {}
  for _, cite in ipairs(el.citations) do
    local prefix = ""
    if cite.mode == "SuppressAuthor" then
      prefix = "-"
    end
    local suffix = ""
    if cite.suffix and #cite.suffix > 0 then
      suffix = pandoc.utils.stringify(cite.suffix)
    end
    table.insert(parts, prefix .. "@" .. cite.id .. suffix)
  end
  return literal("[" .. table.concat(parts, "; ") .. "]")
end

Inlines.Span = function(el)
  local attr = el.attr
  if attr.identifier ~= "" or #attr.classes > 0 or next(attr.attributes) then
    local attrs = {}
    for _, cls in ipairs(attr.classes) do
      table.insert(attrs, "." .. cls)
    end
    if attr.identifier ~= "" then
      table.insert(attrs, "#" .. attr.identifier)
    end
    for k, v in pairs(attr.attributes) do
      table.insert(attrs, k .. "=" .. v)
    end
    return concat{"[", inlines(el.content), "]{", literal(table.concat(attrs, " ")), "}"}
  end
  return inlines(el.content)
end

Inlines.RawInline = function(el)
  if el.format == "markright" then
    return literal(el.text)
  end
  return literal(escape(el.text))
end

-- Block renderers
Blocks = {}

Blocks.Para = function(el)
  return concat{inlines(el.content), cr}
end

Blocks.Plain = function(el)
  return concat{inlines(el.content), cr}
end

Blocks.Header = function(el)
  local prefix = string.rep("#", el.level) .. " "
  return concat{literal(prefix), inlines(el.content), attr_id(el.attr), cr}
end

Blocks.CodeBlock = function(el)
  local fence = backtick_fence(el.text, 3)
  local lang = ""
  if el.classes and el.classes[1] then
    lang = el.classes[1]
  end
  return concat{literal(fence .. lang), cr, literal(el.text), cr, literal(fence), cr}
end

Blocks.BlockQuote = function(el)
  local inner = layout.render(blocks(el.content, blankline))
  local lines = {}
  for line in inner:gmatch("([^\n]*)\n?") do
    if line == "" then
      table.insert(lines, ">")
    else
      table.insert(lines, "> " .. line)
    end
  end
  -- Remove trailing empty ">" if it's just from the final newline
  while #lines > 0 and lines[#lines] == ">" do
    table.remove(lines)
  end
  if #lines == 0 then
    return literal(">\n")
  end
  return literal(table.concat(lines, "\n") .. "\n")
end

Blocks.BulletList = function(el)
  local items = {}
  for _, item in ipairs(el.content) do
    local body = layout.render(blocks(item, blankline))
    local first_line, rest = body:match("^([^\n]*)\n?(.*)")
    local result = "- " .. (first_line or "")
    if rest and rest ~= "" then
      for line in rest:gmatch("([^\n]+)") do
        result = result .. "\n  " .. line
      end
    end
    table.insert(items, result)
  end
  return literal(table.concat(items, "\n") .. "\n")
end

Blocks.OrderedList = function(el)
  local start = el.start or 1
  local items = {}
  for i, item in ipairs(el.content) do
    local num = start + i - 1
    local prefix = tostring(num) .. ". "
    local indent = string.rep(" ", #prefix)
    local body = layout.render(blocks(item, blankline))
    local first_line, rest = body:match("^([^\n]*)\n?(.*)")
    local result = prefix .. (first_line or "")
    if rest and rest ~= "" then
      for line in rest:gmatch("([^\n]+)") do
        result = result .. "\n" .. indent .. line
      end
    end
    table.insert(items, result)
  end
  return literal(table.concat(items, "\n") .. "\n")
end

Blocks.DefinitionList = function(el)
  local groups = {}
  for _, item in ipairs(el.content) do
    local lines = {}
    local term = pandoc.utils.stringify(item[1])
    table.insert(lines, escape(term))
    for _, def in ipairs(item[2]) do
      local body = layout.render(inlines(def[1].content))
      table.insert(lines, ": " .. body)
    end
    table.insert(groups, table.concat(lines, "\n"))
  end
  return literal(table.concat(groups, "\n\n") .. "\n")
end

Blocks.Table = function(el)
  local lines = {}

  -- Extract headers
  local head_row = el.head.rows[1]
  if not head_row then return empty end

  local headers = {}
  local aligns = {}
  for i, cell in ipairs(head_row.cells) do
    table.insert(headers, pandoc.utils.stringify(cell.contents))
    local spec = el.colspecs[i]
    if spec then
      table.insert(aligns, spec[1])
    else
      table.insert(aligns, "AlignDefault")
    end
  end

  -- Header row
  table.insert(lines, "| " .. table.concat(headers, " | ") .. " |")

  -- Separator row
  local seps = {}
  for _, align in ipairs(aligns) do
    if align == "AlignLeft" then
      table.insert(seps, ":---")
    elseif align == "AlignCenter" then
      table.insert(seps, ":---:")
    elseif align == "AlignRight" then
      table.insert(seps, "---:")
    else
      table.insert(seps, "---")
    end
  end
  table.insert(lines, "| " .. table.concat(seps, " | ") .. " |")

  -- Body rows
  for _, body in ipairs(el.bodies) do
    for _, row in ipairs(body.body) do
      local cells = {}
      for _, cell in ipairs(row.cells) do
        table.insert(cells, pandoc.utils.stringify(cell.contents))
      end
      table.insert(lines, "| " .. table.concat(cells, " | ") .. " |")
    end
  end

  -- Caption
  if el.caption and el.caption.long and #el.caption.long > 0 then
    table.insert(lines, "")
    local cap_text = pandoc.utils.stringify(el.caption.long)
    table.insert(lines, "Table: " .. cap_text)
  end

  return literal(table.concat(lines, "\n") .. "\n")
end

Blocks.HorizontalRule = function()
  return literal("---\n")
end

Blocks.Div = function(el)
  local classes = el.classes or {}
  -- Map common admonition classes
  local admonition_map = {
    note = "N>", tip = "T>", warning = "W>",
    important = "!>", caution = "X>",
  }
  for _, cls in ipairs(classes) do
    local prefix = admonition_map[cls]
    if prefix then
      local inner = layout.render(blocks(el.content, blankline))
      local lines = {}
      for line in inner:gmatch("([^\n]*)\n?") do
        if line ~= "" then
          table.insert(lines, prefix .. " " .. line)
        elseif #lines > 0 then
          table.insert(lines, prefix)
        end
      end
      return literal(table.concat(lines, "\n") .. "\n")
    end
  end

  -- Generic fenced div
  local name = classes[1] or ""
  local inner = blocks(el.content, blankline)
  return concat{literal(":::" .. name), cr, inner, cr, literal(":::"), cr}
end

Blocks.RawBlock = function(el)
  if el.format == "markright" then
    return literal(el.text .. "\n")
  end
  -- Wrap in a comment
  return concat{literal("%%"), cr, literal(el.text), cr, literal("%%"), cr}
end

Blocks.LineBlock = function(el)
  local lines = {}
  for _, line in ipairs(el.content) do
    table.insert(lines, pandoc.utils.stringify(line))
  end
  return literal(table.concat(lines, "\\\n") .. "\n")
end

Blocks.Figure = function(el)
  return blocks(el.content, blankline)
end

Blocks.Null = function()
  return empty
end

-- Main writer function
function Writer(doc, opts)
  footnotes = {}
  local body = blocks(doc.blocks, blankline)

  -- Append footnote definitions
  if #footnotes > 0 then
    local fn_blocks = {}
    for i, content in ipairs(footnotes) do
      local label = tostring(i)
      local inner = layout.render(blocks(content, blankline))
      local first_line, rest = inner:match("^([^\n]*)\n?(.*)")
      local result = "[^" .. label .. "]: " .. (first_line or "")
      if rest and rest ~= "" then
        for line in rest:gmatch("([^\n]+)") do
          result = result .. "\n    " .. line
        end
      end
      table.insert(fn_blocks, result)
    end
    body = concat{body, blankline, literal(table.concat(fn_blocks, "\n") .. "\n")}
  end

  -- Handle metadata as front matter
  local meta = doc.meta
  if meta and next(meta) then
    local fm_lines = {}
    for key, val in pairs(meta) do
      local v = pandoc.utils.stringify(val)
      if v ~= "" then
        table.insert(fm_lines, key .. ": " .. v)
      end
    end
    if #fm_lines > 0 then
      table.sort(fm_lines)
      local fm = "---\n" .. table.concat(fm_lines, "\n") .. "\n---\n"
      return fm .. "\n" .. layout.render(body, opts.columns)
    end
  end

  return layout.render(body, opts.columns)
end
