const obsidian = require('obsidian')

let wasmModule = null

async function loadWasm(vault) {
  if (wasmModule) return wasmModule
  const adapter = vault.adapter
  const pluginDir = `${vault.configDir}/plugins/markright`
  const wasmPath = `${pluginDir}/markright_wasm_bg.wasm`

  try {
    const wasmBinary = await adapter.readBinary(wasmPath)
    const mod = await import(`${pluginDir}/markright_wasm.js`)
    await mod.default(wasmBinary)
    wasmModule = mod
    return mod
  } catch {
    return null
  }
}

class MarkRightPlugin extends obsidian.Plugin {
  async onload() {
    // Register .right files to use the markdown view
    this.registerExtensions(['right'], 'markdown')

    // Add post-processor for reading view rendering
    this.registerMarkdownPostProcessor(async (el, ctx) => {
      const file = this.app.vault.getFileByPath(ctx.sourcePath)
      if (!file || !file.path.endsWith('.right')) return

      const wasm = await loadWasm(this.app.vault)
      if (!wasm) return

      const source = await this.app.vault.cachedRead(file)
      const html = wasm.parse_to_html(source)

      // Use Obsidian's sanitized rendering
      el.empty()
      obsidian.sanitizeHTMLToDom(html).childNodes.forEach((node) => {
        el.appendChild(node.cloneNode(true))
      })
    })
  }

  onunload() {
    wasmModule = null
  }
}

module.exports = MarkRightPlugin
