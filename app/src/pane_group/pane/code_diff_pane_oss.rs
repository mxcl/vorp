use crate::{
    ai::blocklist::inline_action::code_diff_view::CodeDiffView,
    app_state::{CodePaneSnapShot, CodePaneTabSnapshot, LeafContents},
    pane_group::{DetachType, PaneConfiguration, PaneContent, PaneGroup, PaneId},
};
use warpui::{AppContext, ModelHandle, ViewContext, ViewHandle};

use super::{ShareableLink, ShareableLinkError};

pub struct CodeDiffPane {
    id: PaneId,
    diff_view: ViewHandle<CodeDiffView>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl CodeDiffPane {
    pub fn from_view(diff_view: ViewHandle<CodeDiffView>, ctx: &mut AppContext) -> Self {
        let pane_configuration = ctx.add_model(|ctx| {
            let mut config = PaneConfiguration::new("");
            config.set_title("Requested Edit", ctx);
            config
        });

        Self {
            id: PaneId::oss_code_diff_placeholder(diff_view.id()),
            diff_view,
            pane_configuration,
        }
    }

    pub fn diff_view(&self, _ctx: &AppContext) -> ViewHandle<CodeDiffView> {
        self.diff_view.clone()
    }
}

impl PaneContent for CodeDiffPane {
    fn id(&self) -> PaneId {
        self.id
    }

    fn attach(
        &self,
        _group: &PaneGroup,
        _focus_handle: crate::pane_group::focus_state::PaneFocusHandle,
        _ctx: &mut ViewContext<PaneGroup>,
    ) {
    }

    fn detach(
        &self,
        _group: &PaneGroup,
        _detach_type: DetachType,
        _ctx: &mut ViewContext<PaneGroup>,
    ) {
    }

    fn snapshot(&self, _app: &AppContext) -> LeafContents {
        LeafContents::Code(CodePaneSnapShot::Local {
            tabs: vec![CodePaneTabSnapshot { path: None }],
            active_tab_index: 0,
            source: None,
        })
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        ctx.focus(&self.diff_view);
    }

    fn has_application_focus(&self, _ctx: &mut ViewContext<PaneGroup>) -> bool {
        false
    }

    fn shareable_link(
        &self,
        _ctx: &mut ViewContext<PaneGroup>,
    ) -> Result<ShareableLink, ShareableLinkError> {
        Ok(ShareableLink::Base)
    }

    fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    fn is_pane_being_dragged(&self, _ctx: &AppContext) -> bool {
        false
    }
}
