import type { Plugin } from 'vite'

export interface ResolvedWikiLink {
  href?: string
  display?: string
  class?: string
}

export interface ResolvedEmbed {
  html: string
}

export interface ClassMap {
  wikilink?: string
  wikilink_broken?: string
  wikiembed?: string
  task_list?: string
  task_icon?: string
  task_content?: string
  math_display?: string
  math_inline?: string
  admonition?: string
  footnote?: string
  footnote_ref?: string
  footnote_range?: string
  footnote_inline?: string
  highlight?: string
}

export interface HtmlOptions {
  wikilinks?: Record<string, ResolvedWikiLink>
  embeds?: Record<string, ResolvedEmbed>
  classes?: ClassMap
}

export default function markright(options?: HtmlOptions): Plugin

export type FrontMatterValue =
  | string
  | number
  | boolean
  | null
  | FrontMatterValue[]
  | { [key: string]: FrontMatterValue }

declare module '*.right' {
  export const frontMatter: Record<string, FrontMatterValue>
  export const html: string
  export default html
}
