const CHILD_FIELDS = [
  'children', 'content', 'items', 'headers', 'rows',
  'caption', 'term', 'definitions', 'keys',
]

/**
 * Walk a MarkRight AST, calling the visitor for each node.
 * When typeOrVisitor is a string, only nodes of that type are visited.
 */
export function visit(node, typeOrVisitor, fn) {
  const visitor = typeof typeOrVisitor === 'function' ? typeOrVisitor : fn
  const typeFilter = typeof typeOrVisitor === 'string' ? typeOrVisitor : null

  function walk(n) {
    if (!n || typeof n !== 'object') return
    if (Array.isArray(n)) { for (const c of n) walk(c); return }
    if (n.type && (!typeFilter || n.type === typeFilter)) visitor(n)
    for (const f of CHILD_FIELDS) if (n[f]) walk(n[f])
  }

  walk(node)
}

/**
 * Deep-clone and transform a MarkRight AST.
 * The transformer receives each node and returns its replacement.
 */
export function transform(node, fn) {
  function walk(n) {
    if (!n || typeof n !== 'object') return n
    if (Array.isArray(n)) return n.map(walk)
    const clone = { ...n }
    for (const f of CHILD_FIELDS) if (clone[f]) clone[f] = walk(clone[f])
    return fn(clone)
  }

  return walk(node)
}
