import { readFile } from 'node:fs/promises'
import matter from 'gray-matter'
import init, { parse_to_html } from '@markright/markright-wasm'

let initPromise

async function ensureInit() {
  if (!initPromise) {
    const wasmUrl = import.meta.resolve('@markright/markright-wasm/markright_wasm_bg.wasm')
    initPromise = readFile(new URL(wasmUrl)).then((wasm) =>
      init({ module_or_path: wasm })
    )
  }
  await initPromise
}

function extractFrontMatter(source) {
  if (!source.startsWith('---\n')) {
    return {}
  }

  const end = source.indexOf('\n---', 4)
  if (end === -1) {
    return {}
  }

  const raw = source.slice(4, end)
  return matter(`---\n${raw}\n---`).data
}

export default function markright() {
  return {
    name: 'vite-plugin-markright',

    async transform(code, id) {
      if (!id.endsWith('.right')) return null
      await ensureInit()

      const frontMatter = extractFrontMatter(code)
      const html = parse_to_html(code)

      return {
        code: [
          `export const frontMatter = ${JSON.stringify(frontMatter)};`,
          `export const html = ${JSON.stringify(html)};`,
          `export default html;`,
        ].join('\n'),
        map: null,
      }
    },
  }
}
