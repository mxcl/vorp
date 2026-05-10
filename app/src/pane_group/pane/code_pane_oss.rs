use warp_util::path::LineAndColumnArg;
use warpui::{AppContext, ModelHandle, View, ViewContext, ViewHandle};

use crate::{
    app_state::{CodePaneSnapShot, CodePaneTabSnapshot, LeafContents},
    code::{
        editor_management::{CodeEditorStatus, CodeSource},
        view::CodeView,
    },
    pane_group::PaneGroup,
};

use super::{
    DetachType, PaneConfiguration, PaneContent, PaneId, ShareableLink, ShareableLinkError,
};

pub struct CodePane {
    id: PaneId,
    file_view: ViewHandle<CodeView>,
    pane_configuration: ModelHandle<PaneConfiguration>,
}

impl CodePane {
    pub fn from_view(file_view: ViewHandle<CodeView>, ctx: &mut AppContext) -> Self {
        Self {
            id: PaneId::oss_code_placeholder(file_view.id()),
            pane_configuration: file_view.as_ref(ctx).pane_configuration(),
            file_view,
        }
    }

    pub fn new<V: View>(
        source: CodeSource,
        line_col: Option<LineAndColumnArg>,
        ctx: &mut ViewContext<V>,
    ) -> Self {
        let view = ctx.add_typed_action_view(move |ctx| CodeView::new(source, line_col, ctx));
        Self::from_view(view, ctx)
    }

    #[cfg(feature = "local_fs")]
    pub fn new_preview<V: View>(source: CodeSource, ctx: &mut ViewContext<V>) -> Self {
        let view = ctx.add_typed_action_view(move |ctx| CodeView::new_preview(source, ctx));
        Self::from_view(view, ctx)
    }

    pub fn file_view(&self, _ctx: &AppContext) -> ViewHandle<CodeView> {
        self.file_view.clone()
    }

    pub fn editor_status(&self, app: &AppContext) -> CodeEditorStatus {
        CodeEditorStatus::editor_status(&self.file_view, app)
    }
}

impl PaneContent for CodePane {
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
        detach_type: DetachType,
        ctx: &mut ViewContext<PaneGroup>,
    ) {
        #[cfg(feature = "local_fs")]
        if matches!(detach_type, DetachType::Closed) {
            self.file_view
                .update(ctx, |code_view, ctx| code_view.cleanup_all_tabs(ctx));
        }
    }

    fn snapshot(&self, app: &AppContext) -> LeafContents {
        let code_view_ref = self.file_view.as_ref(app);
        let tabs = (0..code_view_ref.tab_count())
            .filter_map(|i| code_view_ref.tab_at(i))
            .map(|tab| CodePaneTabSnapshot { path: tab.path() })
            .collect();

        LeafContents::Code(CodePaneSnapShot::Local {
            tabs,
            active_tab_index: code_view_ref.active_tab_index(),
            source: Some(code_view_ref.source().clone()),
        })
    }

    fn focus(&self, ctx: &mut ViewContext<PaneGroup>) {
        self.file_view.update(ctx, |view, ctx| view.focus(ctx));
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
