type AstNode = { type: string; [key: string]: unknown }

/**
 * Walk a MarkRight AST, calling the visitor for each node.
 * When `typeOrVisitor` is a string, only nodes of that type are visited.
 */
export function visit(node: AstNode, visitor: (node: AstNode) => void): void
export function visit(
  node: AstNode,
  type: string,
  visitor: (node: AstNode) => void
): void

/**
 * Deep-clone and transform a MarkRight AST.
 * The transformer receives each node and returns its replacement.
 */
export function transform(
  node: AstNode,
  fn: (node: AstNode) => AstNode
): AstNode
