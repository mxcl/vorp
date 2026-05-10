use std::sync::Arc;

#[cfg(feature = "input_classifier_model")]
use input_classifier_crate::{HeuristicClassifier, InputClassifier};
#[cfg(not(feature = "input_classifier_model"))]
use serde::{Deserialize, Serialize};
use warpui::{Entity, ModelContext, SingletonEntity};

#[cfg(feature = "input_classifier_model")]
pub use input_classifier_crate::{Context, InputType};

#[cfg(feature = "input_classifier_model")]
pub mod util {
    pub use input_classifier_crate::util::*;
}

#[cfg(not(feature = "input_classifier_model"))]
/// The type of input the user has provided.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    /// The user input is a shell command.
    #[default]
    Shell,
    /// The user input is a natural language query to AI.
    AI,
}

#[cfg(not(feature = "input_classifier_model"))]
impl InputType {
    pub fn is_ai(&self) -> bool {
        matches!(self, InputType::AI)
    }
}

#[cfg(not(feature = "input_classifier_model"))]
impl std::str::FromStr for InputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "shell" => Ok(InputType::Shell),
            "ai" => Ok(InputType::AI),
            _ => Err(format!("Invalid input type: {s}. Must be 'shell' or 'ai'")),
        }
    }
}

#[cfg(not(feature = "input_classifier_model"))]
impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Shell => write!(f, "Shell"),
            InputType::AI => write!(f, "AI"),
        }
    }
}

#[cfg(not(feature = "input_classifier_model"))]
pub mod util {
    pub fn is_agent_follow_up_input(_: &str) -> bool {
        false
    }

    pub fn is_one_off_natural_language_word(_: &str) -> bool {
        false
    }
}

#[cfg(not(feature = "input_classifier_model"))]
/// Context for the classifier.
pub struct Context {
    /// The current input type.
    pub current_input_type: InputType,
    /// Whether or not the input is a follow-up to an agent query.
    pub is_agent_follow_up: bool,
}

#[cfg(not(feature = "input_classifier_model"))]
/// The result of running inference on some user input.
pub struct ClassificationResult {
    p_shell: f32,
    p_ai: f32,
}

#[cfg(not(feature = "input_classifier_model"))]
impl ClassificationResult {
    pub fn p_shell(&self) -> f32 {
        self.p_shell
    }

    pub fn p_ai(&self) -> f32 {
        self.p_ai
    }

    pub fn confidence(&self) -> f32 {
        self.p_shell.max(self.p_ai)
    }

    pub fn to_input_type(&self) -> InputType {
        if self.p_shell > self.p_ai {
            InputType::Shell
        } else {
            InputType::AI
        }
    }
}

#[cfg(not(feature = "input_classifier_model"))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
pub trait InputClassifier: 'static + Send + Sync {
    async fn detect_input_type(
        &self,
        input: warp_completer::ParsedTokensSnapshot,
        context: &Context,
    ) -> InputType;

    async fn classify_input(
        &self,
        input: warp_completer::ParsedTokensSnapshot,
        context: &Context,
    ) -> anyhow::Result<ClassificationResult>;
}

#[cfg(not(feature = "input_classifier_model"))]
struct ShellClassifier;

#[cfg(not(feature = "input_classifier_model"))]
#[cfg_attr(not(target_family = "wasm"), async_trait::async_trait)]
#[cfg_attr(target_family = "wasm", async_trait::async_trait(?Send))]
impl InputClassifier for ShellClassifier {
    async fn detect_input_type(
        &self,
        _input: warp_completer::ParsedTokensSnapshot,
        _context: &Context,
    ) -> InputType {
        InputType::Shell
    }

    async fn classify_input(
        &self,
        _input: warp_completer::ParsedTokensSnapshot,
        _context: &Context,
    ) -> anyhow::Result<ClassificationResult> {
        Ok(ClassificationResult {
            p_shell: 1.0,
            p_ai: 0.0,
        })
    }
}

pub struct InputClassifierModel {
    pub classifier: Arc<dyn InputClassifier>,
}

impl InputClassifierModel {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        #[cfg(feature = "nld_onnx_model")]
        match input_classifier_crate::OnnxClassifier::new(
            input_classifier_crate::OnnxModel::BertTiny,
        ) {
            Ok(classifier) => {
                log::info!("Loaded onnx classifier");
                return Self {
                    classifier: Arc::new(classifier),
                };
            }
            Err(e) => log::warn!("Failed to load onnx classifier: {e:#}"),
        }

        #[cfg(feature = "nld_fasttext_model")]
        if is_nld_classifier_enabled(_ctx) {
            match input_classifier_crate::FasttextClassifier::new() {
                Ok(classifier) => {
                    log::info!("Loaded fasttext classifier");
                    return Self {
                        classifier: Arc::new(classifier),
                    };
                }
                Err(e) => log::warn!("Failed to load fasttext classifier: {e:#}"),
            }
        }

        #[cfg(feature = "input_classifier_model")]
        {
            return Self {
                classifier: Arc::new(HeuristicClassifier),
            };
        }

        #[cfg(not(feature = "input_classifier_model"))]
        {
            return Self {
                classifier: Arc::new(ShellClassifier),
            };
        }
    }

    pub fn classifier(&self) -> Arc<dyn InputClassifier> {
        self.classifier.clone()
    }
}

impl Entity for InputClassifierModel {
    type Event = ();
}

impl SingletonEntity for InputClassifierModel {}

#[cfg(feature = "nld_fasttext_model")]
/// Returns true iff the NLD classifier model is enabled.
pub fn is_nld_classifier_enabled(ctx: &warpui::AppContext) -> bool {
    use warp_core::user_preferences::GetUserPreferences as _;
    use warp_core::{channel::ChannelState, features::FeatureFlag};

    if ChannelState::channel().is_dogfood() {
        // The `EnableNLDClassifierModel` can be used to force enable / disable
        // use if it is set.
        ctx.private_user_preferences()
            .read_value("EnableNLDClassifierModel")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok())
            .unwrap_or(FeatureFlag::NLDClassifierModelEnabled.is_enabled())
    } else {
        FeatureFlag::NLDClassifierModelEnabled.is_enabled()
    }
}
