use crate::{
    app_state::{LeafContents, NotebookPaneSnapshot},
    drive::OpenWarpDriveObjectSettings,
    notebooks::notebook::NotebookView,
    pane_group::{focus_state::PaneFocusHandle, pane::view::PaneView, PaneGroup},
    server::ids::SyncId,
};
use warpui::{AppContext, ModelHandle, ViewContext, ViewHandle};

use super::{
    DetachType, PaneConfiguration, PaneContent, PaneId, ShareableLink, ShareableLinkError,
};

pub struct NotebookPane {
    view: ViewHandle<PaneView<NotebookView>>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl NotebookPane {
    pub fn new(notebook_view: ViewHandle<NotebookView>, ctx: &mut AppContext) -> Self {
        let pane_configuration = notebook_view.as_ref(ctx).pane_configuration();
        let view = ctx.add_typed_action_view(notebook_view.window_id(ctx), |ctx| {
            let pane_id = PaneId::from_notebook_pane_ctx(ctx);
            PaneView::new(pane_id, notebook_view, (), pane_configuration.clone(), ctx)
        });

        Self {
            view,
            pane_configuration,
        }
    }

    pub fn restore(
        _notebook_id: Option<SyncId>,
        _settings: &OpenWarpDriveObjectSettings,
        ctx: &mut ViewContext<PaneGroup>,
    ) -> anyhow::Result<Self> {
        let notebook_view = ctx.add_typed_action_view(NotebookView::new);
        Ok(Self::new(notebook_view, ctx))
    }

    pub fn notebook_view(&self, ctx: &AppContext) -> ViewHandle<NotebookView> {
        self.view.as_ref(ctx).child(ctx)
    }
}

impl PaneContent for NotebookPane {
    fn id(&self) -> PaneId {
        PaneId::from_notebook_pane_view(&self.view)
    }

    fn snapshot(&self, app: &AppContext) -> LeafContents {
        LeafContents::Notebook(NotebookPaneSnapshot::CloudNotebook {
            notebook_id: self.notebook_view(app).as_ref(app).notebook_id(app),
            settings: OpenWarpDriveObjectSettings::default(),
        })
    }

    fn attach(
        &self,
        _group: &PaneGroup,
        focus_handle: PaneFocusHandle,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        self.view
            .update(ctx, |view, ctx| view.set_focus_handle(focus_handle, ctx));

        let pane_id = self.id();
        ctx.subscribe_to_view(&self.view, move |group, _, event, ctx| {
            group.handle_pane_view_event(pane_id, event, ctx);
        });
    }

    fn detach(
        &self,
        _group: &PaneGroup,
        _detach_type: DetachType,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        ctx.unsubscribe_to_view(&self.view);
    }

    fn has_application_focus(&self, ctx: &mut ViewContext<PaneGroup>) -> bool {
        self.view.is_self_or_child_focused(ctx)
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        self.notebook_view(ctx)
            .update(ctx, |view, ctx| view.focus(ctx));
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

    fn is_pane_being_dragged(&self, ctx: &AppContext) -> bool {
        self.view.as_ref(ctx).is_being_dragged()
    }
}
