const vscode = require('vscode')

function hasMarkrightSyntax(text) {
  if (!text.startsWith('---\n') && !text.startsWith('---\r\n')) return false
  const check = text.slice(0, 512)
  const close = check.indexOf('\n---', 4)
  if (close === -1) return false
  const raw = check.slice(4, close)
  return raw.split('\n').some((line) => {
    const t = line.trim()
    return t === 'syntax: markright' || t === 'syntax:markright'
  })
}

function checkAndSetLanguage(doc) {
  if (doc.languageId !== 'markdown') return
  if (!doc.fileName.endsWith('.md')) return
  if (hasMarkrightSyntax(doc.getText())) {
    vscode.languages.setTextDocumentLanguage(doc, 'markright')
  }
}

function activate(context) {
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(checkAndSetLanguage),
    vscode.workspace.onDidSaveTextDocument(checkAndSetLanguage)
  )
  // Check already-open documents
  for (const doc of vscode.workspace.textDocuments) {
    checkAndSetLanguage(doc)
  }
}

function deactivate() {}

module.exports = { activate, deactivate }
