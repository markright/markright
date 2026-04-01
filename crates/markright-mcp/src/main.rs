use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_router,
    transport::io::stdio,
};
use schemars::JsonSchema;

#[derive(Debug, serde::Deserialize, JsonSchema)]
struct SourceInput {
    /// MarkRight source text to process
    source: String,
}

fn with_doc<T>(source: &str, f: impl FnOnce(&markright::ast::block::Document) -> T) -> T {
    let bump = markright::Bump::new();
    let doc = markright::parse(source, &bump);
    f(&doc)
}

#[derive(Clone)]
#[allow(dead_code)] // tool_router is read by macro-generated code
struct MarkRight {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MarkRight {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Parse MarkRight source to JSON AST")]
    async fn parse(
        &self,
        Parameters(input): Parameters<SourceInput>,
    ) -> Result<CallToolResult, McpError> {
        let json = with_doc(&input.source, |doc| {
            serde_json::to_string_pretty(doc).unwrap()
        });
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Render MarkRight source to HTML")]
    async fn render(
        &self,
        Parameters(input): Parameters<SourceInput>,
    ) -> Result<CallToolResult, McpError> {
        let html = with_doc(&input.source, markright::to_html);
        Ok(CallToolResult::success(vec![Content::text(html)]))
    }

    #[tool(description = "Format MarkRight source to canonical form")]
    async fn format(
        &self,
        Parameters(input): Parameters<SourceInput>,
    ) -> Result<CallToolResult, McpError> {
        let formatted = with_doc(&input.source, markright::to_string);
        Ok(CallToolResult::success(vec![Content::text(formatted)]))
    }

    #[tool(description = "Lint MarkRight source for common issues")]
    async fn lint(
        &self,
        Parameters(input): Parameters<SourceInput>,
    ) -> Result<CallToolResult, McpError> {
        let lints = with_doc(&input.source, markright::lint);
        if lints.is_empty() {
            Ok(CallToolResult::success(vec![Content::text(
                "No issues found.",
            )]))
        } else {
            let messages: Vec<String> = lints.iter().map(|l| l.message.clone()).collect();
            Ok(CallToolResult::success(vec![Content::text(
                messages.join("\n"),
            )]))
        }
    }

    #[tool(description = "Get the JSON Schema for the MarkRight AST")]
    async fn schema(&self) -> Result<CallToolResult, McpError> {
        let json = serde_json::to_string_pretty(&markright::json_schema()).unwrap();
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[rmcp::tool_handler]
impl ServerHandler for MarkRight {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.instructions =
            Some("MarkRight MCP server. Parse, render, format, and lint .right files.".into());
        info.server_info.name = "markright".into();
        info.server_info.version = env!("CARGO_PKG_VERSION").into();
        info
    }
}

#[tokio::main]
async fn main() {
    let server = MarkRight::new();
    let (stdin, stdout) = stdio();
    let service = server.serve((stdin, stdout)).await.unwrap();
    service.waiting().await.unwrap();
}
