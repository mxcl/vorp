use std::{
    fmt,
    ops::Range,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use warp_core::ui::{icons::Icon, theme::Fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillProvider {
    Warp,
    Agents,
    Claude,
    Codex,
    Cursor,
    Gemini,
    Copilot,
    Droid,
    Github,
    OpenCode,
}

impl SkillProvider {
    pub fn icon(&self) -> Icon {
        Icon::WarpLogoLight
    }

    pub fn icon_fill(&self, fallback: Fill) -> Fill {
        fallback
    }
}

impl fmt::Display for SkillProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SkillProvider")
    }
}

impl Serialize for SkillProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("disabled")
    }
}

impl<'de> Deserialize<'de> for SkillProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = String::deserialize(deserializer)?;
        Ok(Self::Agents)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SkillScope {
    #[default]
    Home,
    Project,
    Bundled,
}

impl fmt::Display for SkillScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SkillScope")
    }
}

impl Serialize for SkillScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("disabled")
    }
}

impl<'de> Deserialize<'de> for SkillScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = String::deserialize(deserializer)?;
        Ok(Self::Home)
    }
}

pub struct SkillProviderDefinition {
    pub provider: SkillProvider,
    pub skills_path: PathBuf,
}

pub static SKILL_PROVIDER_DEFINITIONS: LazyLock<Vec<SkillProviderDefinition>> =
    LazyLock::new(Vec::new);

pub fn provider_rank(_provider: SkillProvider) -> usize {
    usize::MAX
}

pub fn home_skills_path(_provider: SkillProvider) -> Option<PathBuf> {
    None
}

pub fn get_provider_for_path(_path: &Path) -> Option<SkillProvider> {
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedSkill {
    pub path: PathBuf,
    pub name: String,
    pub description: String,
    pub content: String,
    pub line_range: Option<Range<usize>>,
    pub provider: SkillProvider,
    pub scope: SkillScope,
}

impl ParsedSkill {
    pub fn is_bundled(&self) -> bool {
        self.scope == SkillScope::Bundled
    }
}

impl fmt::Display for ParsedSkill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.display().fmt(f)
    }
}

pub fn parse_skill(path: &Path) -> Result<ParsedSkill> {
    Ok(empty_skill(path, SkillScope::Home))
}

pub fn parse_bundled_skill(path: &Path) -> Result<ParsedSkill> {
    Ok(empty_skill(path, SkillScope::Bundled))
}

pub fn read_skills(_path: &Path) -> Vec<ParsedSkill> {
    Vec::new()
}

fn empty_skill(path: &Path, scope: SkillScope) -> ParsedSkill {
    ParsedSkill {
        path: path.to_path_buf(),
        name: String::new(),
        description: String::new(),
        content: String::new(),
        line_range: None,
        provider: SkillProvider::Agents,
        scope,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SkillReference {
    Path(PathBuf),
    BundledSkillId(String),
}

impl fmt::Display for SkillReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillReference::Path(path) => path.display().fmt(f),
            SkillReference::BundledSkillId(id) => id.fmt(f),
        }
    }
}

impl From<SkillReference> for warp_multi_agent_api::skill_descriptor::SkillReference {
    fn from(reference: SkillReference) -> Self {
        match reference {
            SkillReference::Path(path) => {
                warp_multi_agent_api::skill_descriptor::SkillReference::Path(
                    path.to_string_lossy().to_string(),
                )
            }
            SkillReference::BundledSkillId(id) => {
                warp_multi_agent_api::skill_descriptor::SkillReference::BundledSkillId(id)
            }
        }
    }
}

impl From<SkillScope> for warp_multi_agent_api::skill_descriptor::Scope {
    fn from(scope: SkillScope) -> Self {
        let r#type = match scope {
            SkillScope::Home => warp_multi_agent_api::skill_descriptor::scope::Type::Home(()),
            SkillScope::Project => warp_multi_agent_api::skill_descriptor::scope::Type::Project(()),
            SkillScope::Bundled => warp_multi_agent_api::skill_descriptor::scope::Type::Bundled(()),
        };
        Self {
            r#type: Some(r#type),
        }
    }
}

impl From<SkillProvider> for warp_multi_agent_api::skill_descriptor::Provider {
    fn from(_provider: SkillProvider) -> Self {
        Self {
            r#type: Some(warp_multi_agent_api::skill_descriptor::provider::Type::Agents(())),
        }
    }
}

impl From<ParsedSkill> for warp_multi_agent_api::Skill {
    fn from(skill: ParsedSkill) -> Self {
        Self {
            descriptor: Some(warp_multi_agent_api::SkillDescriptor {
                skill_reference: Some(
                    warp_multi_agent_api::skill_descriptor::SkillReference::Path(
                        skill.path.to_string_lossy().to_string(),
                    ),
                ),
                name: skill.name,
                description: skill.description,
                scope: Some(skill.scope.into()),
                provider: Some(skill.provider.into()),
            }),
            content: Some(warp_multi_agent_api::FileContent {
                file_path: skill.path.to_string_lossy().to_string(),
                content: skill.content,
                line_range: skill.line_range.map(|line_range| {
                    warp_multi_agent_api::FileContentLineRange {
                        start: line_range.start as u32,
                        end: line_range.end as u32,
                    }
                }),
            }),
        }
    }
}
