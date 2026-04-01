-- Pandoc custom reader for MarkRight
-- Usage: pandoc -f reader.lua input.right -t docx -o output.docx
-- Requires: `markright` binary in PATH

function Reader(input, opts)
  local cmd = os.getenv("MARKRIGHT_BIN") or "markright"
  local json = pandoc.pipe(cmd, {}, tostring(input))
  local ast = pandoc.json.decode(json)

  local function convert_inlines(nodes)
    local result = pandoc.Inlines{}
    if not nodes then return result end
    for _, node in ipairs(nodes) do
      local t = node.type
      if t == "text" then
        local s = node.value
        local pos = 1
        while pos <= #s do
          local ws_start, ws_end = s:find("^%s+", pos)
          if ws_start then
            result:insert(pandoc.Space())
            pos = ws_end + 1
          else
            local word_end = s:find("%s", pos) or (#s + 1)
            result:insert(pandoc.Str(s:sub(pos, word_end - 1)))
            pos = word_end
          end
        end
      elseif t == "bold" then
        result:insert(pandoc.Strong(convert_inlines(node.children)))
      elseif t == "italic" then
        result:insert(pandoc.Emph(convert_inlines(node.children)))
      elseif t == "bold_italic" then
        result:insert(pandoc.Strong(pandoc.Emph(convert_inlines(node.children))))
      elseif t == "strikethrough" then
        result:insert(pandoc.Strikeout(convert_inlines(node.children)))
      elseif t == "highlight" then
        result:insert(pandoc.Span(convert_inlines(node.children), {class = "highlight"}))
      elseif t == "superscript" then
        result:insert(pandoc.Superscript(convert_inlines(node.children)))
      elseif t == "subscript" then
        result:insert(pandoc.Subscript(convert_inlines(node.children)))
      elseif t == "inline_code" then
        result:insert(pandoc.Code(node.value))
      elseif t == "inline_math" then
        result:insert(pandoc.Math("InlineMath", node.value))
      elseif t == "link" then
        result:insert(pandoc.Link(convert_inlines(node.children), node.url, node.title or ""))
      elseif t == "image" then
        result:insert(pandoc.Image(node.alt or "", node.url, node.title or ""))
      elseif t == "wiki_link" then
        local target = node.target or ""
        if node.fragment then target = target .. "#" .. node.fragment end
        local text = node.alias or node.target or ""
        result:insert(pandoc.Link(pandoc.Str(text), target))
      elseif t == "footnote_ref" then
        result:insert(pandoc.Str("[^" .. node.label .. "]"))
      elseif t == "hard_break" then
        result:insert(pandoc.LineBreak())
      elseif t == "citation" then
        local cites = {}
        local parts = {}
        for _, key in ipairs(node.keys) do
          local mode = "NormalCitation"
          if key.suppress_author then mode = "SuppressAuthor" end
          local suffix = pandoc.Inlines{}
          if key.locator then
            suffix:insert(pandoc.Str(", " .. key.locator))
          end
          table.insert(cites, pandoc.Citation(key.key, mode, {}, suffix))
          local prefix = key.suppress_author and "-@" or "@"
          local loc = key.locator and (", " .. key.locator) or ""
          table.insert(parts, prefix .. key.key .. loc)
        end
        local cite_text = "[" .. table.concat(parts, "; ") .. "]"
        result:insert(pandoc.Cite(cites, pandoc.Inlines{pandoc.Str(cite_text)}))
      else
        result:insert(pandoc.Str(pandoc.utils.stringify(node) or ""))
      end
    end
    return result
  end

  local function convert_blocks(nodes)
    local result = pandoc.Blocks{}
    if not nodes then return result end
    for _, node in ipairs(nodes) do
      local t = node.type
      if t == "heading" then
        local attr = pandoc.Attr(node.id or "")
        result:insert(pandoc.Header(node.level, convert_inlines(node.content), attr))
      elseif t == "paragraph" then
        result:insert(pandoc.Para(convert_inlines(node.content)))
      elseif t == "code_block" then
        local attr = pandoc.Attr(node.id or "", node.lang and {node.lang} or {})
        result:insert(pandoc.CodeBlock(node.body, attr))
      elseif t == "diagram_block" then
        local attr = pandoc.Attr(node.id or "", {node.lang})
        result:insert(pandoc.CodeBlock(node.body, attr))
      elseif t == "math_block" then
        result:insert(pandoc.Para{pandoc.Math("DisplayMath", node.body)})
      elseif t == "thematic_break" then
        result:insert(pandoc.HorizontalRule())
      elseif t == "blockquote" then
        result:insert(pandoc.BlockQuote(convert_blocks(node.children)))
      elseif t == "unordered_list" then
        local items = {}
        for _, item in ipairs(node.items) do
          table.insert(items, convert_blocks(item.children))
        end
        result:insert(pandoc.BulletList(items))
      elseif t == "ordered_list" then
        local items = {}
        for _, item in ipairs(node.items) do
          table.insert(items, convert_blocks(item.children))
        end
        result:insert(pandoc.OrderedList(items, pandoc.ListAttributes(node.start or 1)))
      elseif t == "task_list" then
        local items = {}
        for _, item in ipairs(node.items) do
          local checked = item.state == "done" or item.state == "cancelled"
          local marker = checked and "☑" or "☐"
          local content = convert_blocks(item.children)
          if #content > 0 and content[1].tag == "Para" then
            content[1].content:insert(1, pandoc.Space())
            content[1].content:insert(1, pandoc.Str(marker))
          end
          table.insert(items, content)
        end
        result:insert(pandoc.BulletList(items))
      elseif t == "table" then
        local headers = {}
        for _, h in ipairs(node.headers) do
          table.insert(headers, {pandoc.Plain(convert_inlines(h))})
        end
        local rows = {}
        for _, row in ipairs(node.rows) do
          local cells = {}
          for _, cell in ipairs(row) do
            table.insert(cells, {pandoc.Plain(convert_inlines(cell))})
          end
          table.insert(rows, cells)
        end
        local aligns = {}
        local widths = {}
        for _, a in ipairs(node.alignments) do
          if a == "left" then table.insert(aligns, pandoc.AlignLeft)
          elseif a == "center" then table.insert(aligns, pandoc.AlignCenter)
          elseif a == "right" then table.insert(aligns, pandoc.AlignRight)
          else table.insert(aligns, pandoc.AlignDefault)
          end
          table.insert(widths, 0)
        end
        local caption = node.caption and convert_inlines(node.caption) or pandoc.Inlines{}
        result:insert(pandoc.utils.from_simple_table(
          pandoc.SimpleTable(caption, aligns, widths, headers, rows)
        ))
      elseif t == "admonition" then
        local kind_map = {note="note", tip="tip", warning="warning", important="important", caution="caution"}
        local cls = kind_map[node.kind] or "note"
        result:insert(pandoc.Div(convert_blocks(node.children), {class = cls}))
      elseif t == "definition_list" then
        local items = {}
        for _, item in ipairs(node.items) do
          local defs = {}
          for _, def in ipairs(item.definitions) do
            table.insert(defs, {pandoc.Para(convert_inlines(def))})
          end
          table.insert(items, {convert_inlines(item.term), defs})
        end
        result:insert(pandoc.DefinitionList(items))
      elseif t == "fenced_div" then
        result:insert(pandoc.Div(convert_blocks(node.children), {class = node.name or ""}))
      elseif t == "footnote_def" then
        -- Footnotes are handled by Pandoc differently; store as note block
        result:insert(pandoc.Div(convert_blocks(node.children), {id = "fn-" .. node.label}))
      elseif t == "comment" then
        -- Skip comments
      elseif t == "toc" then
        -- Skip TOC directives
      elseif t == "include" then
        result:insert(pandoc.Para{pandoc.Str("{{" .. node.path .. "}}")})
      end
    end
    return result
  end

  local meta = {}
  if ast.front_matter then
    for line in ast.front_matter.raw:gmatch("[^\n]+") do
      local key, val = line:match("^(%S+):%s*(.+)$")
      if key then
        meta[key] = val
      end
    end
  end

  return pandoc.Pandoc(convert_blocks(ast.children), meta)
end
