//! Hermes-inspired Skills Loader
//! Progressive disclosure: only name + description in context until needed
//! Compatible with agentskills.io / SKILL.md format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub version: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub meta: SkillMeta,
    pub content: String,
}

pub struct SkillsLoader {
    skills_dir: PathBuf,
    index: HashMap<String, SkillMeta>,
}

impl SkillsLoader {
    pub fn new(skills_dir: impl Into<PathBuf>) -> Self {
        Self {
            skills_dir: skills_dir.into(),
            index: HashMap::new(),
        }
    }

    /// Scan skills directory and build lightweight index (name + description only)
    pub async fn index(&mut self) -> anyhow::Result<()> {
        self.index.clear();
        if !self.skills_dir.exists() {
            fs::create_dir_all(&self.skills_dir).await?;
            return Ok(());
        }

        let mut entries = fs::read_dir(&self.skills_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let skill_md = path.join("SKILL.md");
                if skill_md.exists() {
                    if let Ok(meta) = self.parse_meta(&skill_md).await {
                        self.index.insert(meta.name.clone(), meta);
                    }
                }
            }
        }
        tracing::info!("Indexed {} skills", self.index.len());
        Ok(())
    }

    async fn parse_meta(&self, path: &Path) -> anyhow::Result<SkillMeta> {
        let content = fs::read_to_string(path).await?;
        // Simple frontmatter parser (YAML between ---)
        let mut name = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let mut description = String::new();
        let mut version = None;
        let platforms = None;
        let tags = vec![];

        if let Some(rest) = content.strip_prefix("---") {
            if let Some(end) = rest.find("---") {
                let frontmatter = &rest[..end];
                for line in frontmatter.lines() {
                    let line = line.trim();
                    if let Some(v) = line.strip_prefix("name:") {
                        name = v.trim().trim_matches('"').to_string();
                    } else if let Some(v) = line.strip_prefix("description:") {
                        description = v.trim().trim_matches('"').to_string();
                    } else if let Some(v) = line.strip_prefix("version:") {
                        version = Some(v.trim().trim_matches('"').to_string());
                    }
                }
            }
        }

        Ok(SkillMeta {
            name,
            description,
            version,
            platforms,
            tags,
            path: path.to_path_buf(),
        })
    }

    /// Return compact index for system prompt (progressive disclosure level 0)
    pub fn list_summary(&self) -> String {
        if self.index.is_empty() {
            return "No skills available.".into();
        }
        self.index
            .values()
            .map(|s| format!("- **{}**: {}", s.name, s.description))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Load full skill content on demand (level 1)
    pub async fn load(&self, name: &str) -> anyhow::Result<Option<Skill>> {
        if let Some(meta) = self.index.get(name) {
            let content = fs::read_to_string(&meta.path).await?;
            Ok(Some(Skill {
                meta: meta.clone(),
                content,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn names(&self) -> Vec<String> {
        self.index.keys().cloned().collect()
    }
}

/// Example skill creation helper
pub async fn create_example_skill(dir: &Path) -> anyhow::Result<()> {
    let skill_dir = dir.join("computer-use-safety");
    fs::create_dir_all(&skill_dir).await?;
    let skill_md = r#"---
name: computer-use-safety
description: Safe patterns for screenshot, mouse and keyboard tools with human approval.
version: 1.0.0
metadata:
  hermes:
    tags: [computer-use, safety, guardrails]
---

# Computer Use Safety

## When to Use
Whenever the agent needs to interact with the desktop via screenshot, mouse or keyboard.

## Procedure
1. Always take a screenshot first to understand the current state.
2. Describe the intended action clearly before executing.
3. Wait for tool approval if required by guardrails.
4. Verify the result with another screenshot.

## Pitfalls
- Never click without confirming coordinates from a recent screenshot.
- Avoid destructive actions without explicit confirmation.

## Verification
Confirm the expected UI change occurred via screenshot comparison.
"#;
    fs::write(skill_dir.join("SKILL.md"), skill_md).await?;
    Ok(())
}
