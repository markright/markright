import { createHighlighter, type Highlighter } from "shiki";

let highlighter: Highlighter | null = null;
let highlighterPromise: Promise<Highlighter> | null = null;

const LANGS = [
  "javascript",
  "typescript",
  "rust",
  "python",
  "html",
  "css",
  "json",
  "yaml",
  "bash",
  "markdown",
  "toml",
  "go",
  "lua",
  "c",
  "cpp",
] as const;

export async function getHighlighter() {
  if (highlighter) {
    return highlighter;
  }

  if (!highlighterPromise) {
    highlighterPromise = createHighlighter({
      themes: ["github-dark"],
      langs: [...LANGS],
    }).then((instance) => {
      highlighter = instance;
      return instance;
    });
  }

  return highlighterPromise;
}

export function highlightCode(
  hl: Highlighter,
  code: string,
  lang: string
): string | null {
  const loaded = hl.getLoadedLanguages();
  if (!loaded.includes(lang)) return null;
  return hl.codeToHtml(code, { lang, theme: "github-dark" });
}
