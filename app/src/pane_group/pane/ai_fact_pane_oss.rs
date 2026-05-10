use crate::{
    app_state::{AIFactPaneSnapshot, LeafContents},
    pane_group::{DetachType, PaneConfiguration, PaneContent, PaneGroup, PaneId},
};
use warpui::{AppContext, ModelHandle, View, ViewContext};

use super::{ShareableLink, ShareableLinkError};

pub struct AIFactPane {
    id: PaneId,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl AIFactPane {
    pub fn new<V: View>(ctx: &mut ViewContext<V>) -> Self {
        Self {
            id: PaneId::oss_ai_fact_placeholder(ctx.view_id()),
            pane_configuration: ctx.add_model(|_| PaneConfiguration::new("")),
        }
    }
}

impl PaneContent for AIFactPane {
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
        LeafContents::AIFact(AIFactPaneSnapshot::Personal)
    }

    fn has_application_focus(&self, _ctx: &mut ViewContext<PaneGroup>) -> bool {
        false
    }

    fn focus(&self, _ctx: &mut ViewContext<PaneGroup>) {}

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
