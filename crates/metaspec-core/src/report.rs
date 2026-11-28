//! What `metaspec pack check` reports, format `metaspec-pack-check/1`.

use serde::Serialize;

/// The format name of a check report.
pub const FORMAT: &str = "metaspec-pack-check/1";

/// The schema of a check report.
pub const SCHEMA: &str = include_str!("../../../schema/metaspec-pack-check-1.schema.json");

/// One error or warning: a code, the thing it is about, and a sentence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Issue {
    pub code: String,
    pub subject: String,
    pub message: String,
}

impl Issue {
    pub fn new(code: &str, subject: impl Into<String>, message: impl Into<String>) -> Issue {
        Issue {
            code: code.to_string(),
            subject: subject.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    pub blocks: usize,
    pub layers: usize,
    pub kpis: usize,
    pub planned_blocks: usize,
    pub document_classes: usize,
    pub documents: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LayerRow {
    pub number: u32,
    pub name: String,
    pub blocks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ClassRow {
    pub class: String,
    pub shape: &'static str,
    pub documents: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolState {
    /// The manifest names no tool.
    None,
    /// The program exists. metaspec ran nothing.
    Found,
    /// The program cannot be started.
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ToolRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<Vec<String>>,
    pub state: ToolState,
}

/// The result of loading a pack. A refused pack has errors and none of the optional parts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Report {
    pub format: &'static str,
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Counts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<LayerRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<ClassRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<ToolRow>,
    pub errors: Vec<Issue>,
    pub warnings: Vec<Issue>,
}

impl Report {
    /// A report of a pack that was refused for these errors.
    pub fn refused(errors: Vec<Issue>, warnings: Vec<Issue>) -> Report {
        Report {
            format: FORMAT,
            valid: false,
            pack: None,
            title: None,
            visibility: None,
            extends: None,
            level: None,
            counts: None,
            layers: None,
            documents: None,
            tool: None,
            errors,
            warnings,
        }
    }

    /// The report as text, one fact per line, for a person.
    pub fn text(&self) -> String {
        let mut out = String::new();
        if let (Some(pack), Some(visibility)) = (&self.pack, self.visibility) {
            out.push_str(&format!("pack {pack} ({visibility})"));
            if let Some(level) = self.level {
                out.push_str(&format!(", declares {level}"));
            }
            if let Some(extends) = &self.extends {
                out.push_str(&format!(", extends {extends}"));
            }
            out.push('\n');
        }
        if let Some(counts) = &self.counts {
            out.push_str(&format!(
                "blocks: {} in {} layers\n",
                counts.blocks, counts.layers
            ));
            for layer in self.layers.iter().flatten() {
                out.push_str(&format!(
                    "  layer {} {}: {}\n",
                    layer.number,
                    layer.name,
                    layer.blocks.len()
                ));
            }
            out.push_str(&format!(
                "kpis: {} ({} planned blocks)\n",
                counts.kpis, counts.planned_blocks
            ));
            out.push_str(&format!(
                "documents: {} in {} classes\n",
                counts.documents, counts.document_classes
            ));
            for class in self.documents.iter().flatten() {
                out.push_str(&format!(
                    "  {} ({}): {}\n",
                    class.class, class.shape, class.documents
                ));
            }
        }
        if let Some(tool) = &self.tool {
            let state = match tool.state {
                ToolState::None => "none",
                ToolState::Found => "found, not run",
                ToolState::NotFound => "cannot be started",
            };
            match &tool.run {
                Some(run) => out.push_str(&format!("tool: {} ({state})\n", run.join(" "))),
                None => out.push_str(&format!("tool: {state}\n")),
            }
        }
        for issue in &self.errors {
            out.push_str(&format!(
                "error {} {}: {}\n",
                issue.code, issue.subject, issue.message
            ));
        }
        for issue in &self.warnings {
            out.push_str(&format!(
                "warning {} {}: {}\n",
                issue.code, issue.subject, issue.message
            ));
        }
        out.push_str(&format!(
            "{}: {} errors, {} warnings\n",
            if self.valid { "loaded" } else { "refused" },
            self.errors.len(),
            self.warnings.len()
        ));
        out
    }
}
