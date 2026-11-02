use crate::tools::ToolSpec;

pub fn computer_use_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "screenshot",
            source: "computer-use",
        },
        ToolSpec {
            name: "mouse",
            source: "computer-use",
        },
        ToolSpec {
            name: "keyboard",
            source: "computer-use",
        },
    ]
}
