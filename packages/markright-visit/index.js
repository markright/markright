/**
 * Walk a MarkRight AST node tree, calling the visitor for each node.
 *
 * @param {object} node - AST node (or document root with `children`)
 * @param {string|function} typeOrVisitor - node type string, or visitor function
 * @param {function} [fn] - visitor function when first arg is a type filter
 */
export function visit(node, typeOrVisitor, fn) {
  const visitor = typeof typeOrVisitor === 'function' ? typeOrVisitor : fn
  const typeFilter = typeof typeOrVisitor === 'string' ? typeOrVisitor : null

  function walk(n) {
    if (!n || typeof n !== 'object') return
    if (Array.isArray(n)) {
      for (const child of n) walk(child)
      return
    }

    if (n.type && (!typeFilter || n.type === typeFilter)) {
      visitor(n)
    }

    // Walk all arrays that could contain child nodes
    if (n.children) walk(n.children)
    if (n.content) walk(n.content)
    if (n.items) walk(n.items)
    if (n.headers) walk(n.headers)
    if (n.rows) walk(n.rows)
    if (n.caption) walk(n.caption)
    if (n.term) walk(n.term)
    if (n.definitions) walk(n.definitions)
    if (n.keys) walk(n.keys)
  }

  walk(node)
}

/**
 * Deep-clone and transform a MarkRight AST. The transformer receives each node
 * and returns a replacement (or the same node to keep it).
 *
 * @param {object} node - AST node (or document root)
 * @param {function} fn - (node) => node transformer
 * @returns {object} new AST
 */
export function transform(node, fn) {
  function walk(n) {
    if (!n || typeof n !== 'object') return n
    if (Array.isArray(n)) return n.map(walk)

    const clone = { ...n }

    if (clone.children) clone.children = clone.children.map(walk)
    if (clone.content) clone.content = clone.content.map(walk)
    if (clone.items) clone.items = clone.items.map(walk)
    if (clone.headers) clone.headers = clone.headers.map((h) => h.map(walk))
    if (clone.rows) clone.rows = clone.rows.map((r) => r.map((c) => c.map(walk)))
    if (clone.caption) clone.caption = clone.caption.map(walk)
    if (clone.term) clone.term = clone.term.map(walk)
    if (clone.definitions) clone.definitions = clone.definitions.map((d) => d.map(walk))
    if (clone.keys) clone.keys = clone.keys.map(walk)

    return fn(clone)
  }

  return walk(node)
}
