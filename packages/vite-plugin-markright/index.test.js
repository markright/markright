import test from 'node:test'
import assert from 'node:assert/strict'
import markright from './index.js'

async function transform(source, id = 'fixture.right') {
  const plugin = markright()
  const result = await plugin.transform(source, id)
  assert.ok(result)
  assert.equal(result.map, null)
  return result.code
}

function readExport(code, name) {
  const pattern = new RegExp(`export const ${name} = (.*);`)
  const match = code.match(pattern)
  assert.ok(match, `missing export: ${name}`)
  return JSON.parse(match[1])
}

test('parses nested YAML front matter with MarkRight delimiters', async () => {
  const code = await transform(`---
title: Hello
author:
  name: SC
tags:
  - one
  - two
published: true
---
# Hello
`)

  assert.deepEqual(readExport(code, 'frontMatter'), {
    title: 'Hello',
    author: { name: 'SC' },
    tags: ['one', 'two'],
    published: true,
  })
  assert.equal(readExport(code, 'html'), '<h1>Hello</h1>\n')
})

test('ignores thematic breaks without closing front matter fence', async () => {
  const code = await transform(`---

# Hello
`)

  assert.deepEqual(readExport(code, 'frontMatter'), {})
  assert.equal(readExport(code, 'html'), '<hr>\n<h1>Hello</h1>\n')
})

test('rejects malformed YAML in front matter', async () => {
  const plugin = markright()

  await assert.rejects(
    plugin.transform(`---
title: [oops
---
# Hello
`, 'broken.right')
  )
})
