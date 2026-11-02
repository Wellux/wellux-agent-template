#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolSpec {
    pub name: &'static str,
    pub source: &'static str,
}

pub fn load_engram_memory() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "mem_save",
            source: "engram",
        },
        ToolSpec {
            name: "mem_search",
            source: "engram",
        },
        ToolSpec {
            name: "mem_context",
            source: "engram",
        },
        ToolSpec {
            name: "mem_timeline",
            source: "engram",
        },
    ]
}

pub fn load_hermes_mcp() -> Vec<ToolSpec> {
    vec![ToolSpec {
        name: "hermes",
        source: "mcp",
    }]
}
