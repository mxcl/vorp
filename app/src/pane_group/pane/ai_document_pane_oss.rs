use crate::{
    ai::ai_document_view::AIDocumentView,
    app_state::{AIDocumentPaneSnapshot, LeafContents},
    pane_group::{DetachType, PaneConfiguration, PaneContent, PaneGroup, PaneId},
};
use warpui::{AppContext, ModelHandle, ViewContext, ViewHandle};

use super::{ShareableLink, ShareableLinkError};

pub struct AIDocumentPane {
    id: PaneId,
    document_view: ViewHandle<AIDocumentView>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl AIDocumentPane {
    pub fn new(document_view: ViewHandle<AIDocumentView>, ctx: &mut AppContext) -> Self {
        let pane_configuration = document_view.as_ref(ctx).pane_configuration().to_owned();
        Self {
            id: PaneId::oss_ai_document_placeholder(document_view.id()),
            document_view,
            pane_configuration,
        }
    }

    pub fn document_view(&self, _ctx: &AppContext) -> ViewHandle<AIDocumentView> {
        self.document_view.clone()
    }
}

impl PaneContent for AIDocumentPane {
    fn id(&self) -> PaneId {
        self.id
    }

    fn snapshot(&self, app: &AppContext) -> LeafContents {
        let document_view = self.document_view.as_ref(app);
        LeafContents::AIDocument(AIDocumentPaneSnapshot::Local {
            document_id: document_view.document_id().to_string(),
            version: document_view.document_version().0 as i32,
            content: None,
            title: None,
        })
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

    fn has_application_focus(&self, _ctx: &mut ViewContext<PaneGroup>) -> bool {
        false
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        self.document_view
            .update(ctx, |document_view, ctx| document_view.focus(ctx));
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
