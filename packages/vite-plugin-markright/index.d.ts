import type { Plugin } from 'vite'

export default function markright(): Plugin

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
