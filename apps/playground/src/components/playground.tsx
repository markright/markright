"use client";

import {
  Suspense,
  use,
  useDeferredValue,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { EXAMPLE } from "@/lib/example";
import { getWasm } from "@/lib/wasm";
import { getHighlighter, highlightCode } from "@/lib/highlighter";
import katex from "katex";

const runtimePromise = Promise.all([getWasm(), getHighlighter()]);

export function Playground() {
  return (
    <Suspense fallback={<PlaygroundFallback />}>
      <PlaygroundContent />
    </Suspense>
  );
}

function PlaygroundFallback() {
  return (
    <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
      Loading playground...
    </div>
  );
}

function PlaygroundContent() {
  const [wasm, highlighter] = use(runtimePromise);
  const [source, setSource] = useState(EXAMPLE);
  const [view, setView] = useState<"preview" | "html" | "ast">("preview");
  const deferredSource = useDeferredValue(source);
  const previewRef = useRef<HTMLDivElement>(null);

  const { html, rawAst, warnings } = useMemo(() => {
    const html = wasm.parse_to_html(deferredSource);
    const lintOutput = wasm.lint(deferredSource);
    const warnings = lintOutput ? lintOutput.split("\n") : [];
    const rawAst =
      view === "ast"
        ? JSON.stringify(JSON.parse(wasm.parse(deferredSource)), null, 2)
        : "";

    return { html, rawAst, warnings };
  }, [deferredSource, view, wasm]);

  useEffect(() => {
    if (!previewRef.current || view !== "preview") {
      return;
    }

    const el = previewRef.current;

    for (const pre of el.querySelectorAll("pre")) {
      const code = pre.querySelector("code");
      if (!code) continue;

      const match = code.className.match(/language-(\S+)/);
      if (!match) continue;

      const highlighted = highlightCode(
        highlighter,
        code.textContent ?? "",
        match[1]
      );
      if (!highlighted) continue;

      const temp = document.createElement("div");
      temp.innerHTML = highlighted;

      const newPre = temp.querySelector("pre");
      if (!newPre) continue;

      newPre.style.background = "";
      newPre.className =
        newPre.className +
        " my-4 rounded-lg border border-border p-4 overflow-x-auto";
      pre.replaceWith(newPre);
    }

    for (const mathEl of el.querySelectorAll(".math-display")) {
      const tex = mathEl.textContent?.replace(/^\\\[/, "").replace(/\\\]$/, "");
      if (!tex) continue;

      katex.render(tex, mathEl as HTMLElement, {
        displayMode: true,
        throwOnError: false,
      });
    }

    for (const mathEl of el.querySelectorAll(".math-inline")) {
      const tex = mathEl.textContent?.replace(/^\\\(/, "").replace(/\\\)$/, "");
      if (!tex) continue;

      katex.render(tex, mathEl as HTMLElement, {
        displayMode: false,
        throwOnError: false,
      });
    }
  }, [highlighter, html, view]);

  const handleFormat = () => {
    setSource(wasm.format(source));
  };

  return (
    <div className="flex h-full flex-col">
      <header className="flex shrink-0 items-center justify-between border-b border-border px-4 py-2">
        <div className="flex items-center gap-3">
          <h1 className="text-sm font-semibold tracking-tight">
            MarkRight Playground
          </h1>
          {warnings.length > 0 && (
            <Tooltip>
              <TooltipTrigger className="cursor-help font-mono text-xs text-yellow-500">
                {warnings.length} warning{warnings.length > 1 ? "s" : ""}
              </TooltipTrigger>
              <TooltipContent
                side="bottom"
                className="max-w-sm font-mono text-xs"
              >
                {warnings.map((warning, index) => (
                  <div key={index}>{warning}</div>
                ))}
              </TooltipContent>
            </Tooltip>
          )}
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={handleFormat}>
            Format
          </Button>
        </div>
      </header>

      <ResizablePanelGroup orientation="horizontal" className="flex-1">
        <ResizablePanel defaultSize={50} minSize={25}>
          <div className="flex h-full flex-col">
            <div className="shrink-0 border-b border-border px-4 py-1.5">
              <span className="text-xs font-semibold uppercase tracking-widest text-muted-foreground">
                Source
              </span>
            </div>
            <textarea
              value={source}
              onChange={(e) => setSource(e.target.value)}
              spellCheck={false}
              className="flex-1 resize-none border-none bg-muted/30 p-4 font-mono text-sm leading-relaxed text-foreground outline-none"
              style={{ tabSize: 2 }}
              placeholder="Type MarkRight here..."
            />
          </div>
        </ResizablePanel>

        <ResizableHandle withHandle />

        <ResizablePanel defaultSize={50} minSize={25}>
          <div className="flex h-full flex-col">
            <div className="shrink-0 border-b border-border px-4 py-1.5">
              <Tabs
                value={view}
                onValueChange={(value: string | number | null) =>
                  setView(value as "preview" | "html" | "ast")
                }
              >
                <TabsList className="h-auto gap-1 bg-transparent p-0">
                  <TabsTrigger
                    value="preview"
                    className="h-auto rounded px-2 py-0.5 text-xs font-semibold uppercase tracking-widest data-[state=active]:bg-muted"
                  >
                    Preview
                  </TabsTrigger>
                  <TabsTrigger
                    value="html"
                    className="h-auto rounded px-2 py-0.5 text-xs font-semibold uppercase tracking-widest data-[state=active]:bg-muted"
                  >
                    HTML
                  </TabsTrigger>
                  <TabsTrigger
                    value="ast"
                    className="h-auto rounded px-2 py-0.5 text-xs font-semibold uppercase tracking-widest data-[state=active]:bg-muted"
                  >
                    AST
                  </TabsTrigger>
                </TabsList>
              </Tabs>
            </div>

            {view === "preview" && (
              <div
                ref={previewRef}
                className="markright-preview flex-1 overflow-y-auto p-6 text-sm leading-relaxed"
                dangerouslySetInnerHTML={{ __html: html }}
              />
            )}
            {view === "html" && (
              <pre className="flex-1 overflow-auto bg-muted/30 p-4 font-mono text-xs text-foreground">
                {html}
              </pre>
            )}
            {view === "ast" && (
              <pre className="flex-1 overflow-auto bg-muted/30 p-4 font-mono text-xs text-foreground">
                {rawAst}
              </pre>
            )}
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
