use std::collections::HashMap;
use std::sync::RwLock;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use markright::ast::block::Block;
use markright::ast::inline::Inline;

#[derive(Debug)]
struct Backend {
    client: Client,
    docs: RwLock<HashMap<Url, String>>,
}

impl Backend {
    fn get_doc(&self, uri: &Url) -> String {
        self.docs
            .read()
            .unwrap()
            .get(uri)
            .cloned()
            .unwrap_or_default()
    }
}

fn heading_symbols(text: &str) -> Vec<DocumentSymbol> {
    let bump = markright::Bump::new();
    let doc = markright::parse(text, &bump);
    let lines: Vec<&str> = text.lines().collect();
    let mut line_num = 0;
    let mut symbols = Vec::new();

    for block in &doc.children {
        if let Block::Heading { content, .. } = block {
            let name: String = content.iter().map(|i| inline_text(i)).collect();

            while line_num < lines.len() {
                if lines[line_num].starts_with('#') && lines[line_num].contains(&name) {
                    break;
                }
                line_num += 1;
            }

            let pos = Position::new(line_num as u32, 0);
            let end_col = lines.get(line_num).map_or(0, |l| l.len() as u32);
            let range = Range::new(pos, Position::new(line_num as u32, end_col));

            #[allow(deprecated)]
            symbols.push(DocumentSymbol {
                name,
                detail: None,
                kind: SymbolKind::STRING,
                tags: None,
                deprecated: None,
                range,
                selection_range: range,
                children: None,
            });
        }
    }
    symbols
}

fn inline_text(inline: &Inline) -> String {
    match inline {
        Inline::Text { value } => value.to_string(),
        Inline::InlineCode { value } | Inline::InlineMath { value } => value.to_string(),
        Inline::Bold { children }
        | Inline::Italic { children }
        | Inline::BoldItalic { children }
        | Inline::Strikethrough { children }
        | Inline::Highlight { children }
        | Inline::Superscript { children }
        | Inline::Subscript { children }
        | Inline::Link { children, .. }
        | Inline::BracketedSpan { children, .. } => {
            children.iter().map(|i| inline_text(i)).collect()
        }
        _ => String::new(),
    }
}

fn find_footnote_target(text: &str, line: u32, col: u32) -> Option<(u32, u32)> {
    let lines: Vec<&str> = text.lines().collect();
    let cur = lines.get(line as usize)?;

    // Find [^label] around cursor
    let col = col as usize;
    let start = cur[..col].rfind("[^")?;
    let end = cur[start..].find(']')? + start;
    let label = &cur[start + 2..end];
    if label.is_empty() {
        return None;
    }

    // If cursor is on a ref [^label], find the def [^label]:
    // If cursor is on a def [^label]:, find the first ref [^label]
    let is_def = cur[end..].starts_with("]:");
    let needle = if is_def {
        format!("[^{label}]")
    } else {
        format!("[^{label}]:")
    };

    for (i, line_text) in lines.iter().enumerate() {
        if i == line as usize && is_def {
            continue;
        }
        if let Some(c) = line_text.find(&needle) {
            if is_def && line_text[c + needle.len()..].starts_with(':') {
                continue;
            }
            return Some((i as u32, c as u32));
        }
    }
    None
}

fn lint_diagnostics(text: &str) -> Vec<Diagnostic> {
    let bump = markright::Bump::new();
    let doc = markright::parse(text, &bump);
    markright::lint(&doc)
        .into_iter()
        .map(|l| Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 0)),
            severity: Some(DiagnosticSeverity::WARNING),
            message: l.message,
            ..Diagnostic::default()
        })
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "markright-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "markright-lsp ready")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;
        let diags = lint_diagnostics(&text);
        self.docs.write().unwrap().insert(uri.clone(), text);
        self.client.publish_diagnostics(uri, diags, None).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next() {
            let uri = params.text_document.uri.clone();
            let diags = lint_diagnostics(&change.text);
            self.docs.write().unwrap().insert(uri.clone(), change.text);
            self.client.publish_diagnostics(uri, diags, None).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.docs.write().unwrap().remove(&params.text_document.uri);
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let text = self.get_doc(&uri);

        if let Some((line, col)) = find_footnote_target(&text, pos.line, pos.character) {
            let target_pos = Position::new(line, col);
            let loc = Location::new(uri, Range::new(target_pos, target_pos));
            return Ok(Some(GotoDefinitionResponse::Scalar(loc)));
        }
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let text = self.get_doc(&params.text_document.uri);
        Ok(Some(DocumentSymbolResponse::Nested(heading_symbols(&text))))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let text = self.get_doc(&params.text_document.uri);
        let bump = markright::Bump::new();
        let doc = markright::parse(&text, &bump);
        let formatted = markright::to_string(&doc);

        let line_count = text.lines().count() as u32;
        let last_len = text.lines().last().map_or(0, |l| l.len() as u32);

        Ok(Some(vec![TextEdit {
            range: Range::new(Position::new(0, 0), Position::new(line_count, last_len)),
            new_text: formatted,
        }]))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend {
        client,
        docs: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
