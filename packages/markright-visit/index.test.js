import test from 'node:test'
import assert from 'node:assert/strict'
import { visit, transform } from './index.js'

const sampleAst = {
  front_matter: { raw: 'title: Test' },
  children: [
    {
      type: 'heading',
      level: 1,
      content: [{ type: 'text', value: 'Hello' }],
    },
    {
      type: 'paragraph',
      content: [
        { type: 'text', value: 'See ' },
        { type: 'wiki_link', target: 'Other', fragment: null, alias: null },
        { type: 'text', value: ' for more.' },
      ],
    },
  ],
}

test('visit walks all nodes', () => {
  const types = []
  visit(sampleAst, (node) => types.push(node.type))
  assert.deepEqual(types, [
    'heading',
    'text',
    'paragraph',
    'text',
    'wiki_link',
    'text',
  ])
})

test('visit with type filter', () => {
  const targets = []
  visit(sampleAst, 'wiki_link', (node) => targets.push(node.target))
  assert.deepEqual(targets, ['Other'])
})

test('transform replaces nodes', () => {
  const result = transform(sampleAst, (node) => {
    if (node.type === 'wiki_link') {
      return { ...node, target: 'Resolved', alias: 'resolved link' }
    }
    return node
  })

  // Original unchanged
  assert.equal(sampleAst.children[1].content[1].target, 'Other')

  // Transformed
  assert.equal(result.children[1].content[1].target, 'Resolved')
  assert.equal(result.children[1].content[1].alias, 'resolved link')
})

test('transform preserves structure', () => {
  const result = transform(sampleAst, (node) => node)
  assert.deepEqual(result, sampleAst)
  assert.notStrictEqual(result, sampleAst) // deep clone
})
