use std::path::PathBuf;

use warpui::{
    elements::ChildView, ui_components::components::UiComponentStyles, AppContext, Element, Entity,
    TypedActionView, View, ViewContext, ViewHandle,
};

use crate::{
    tab_configs::PickerStyle,
    view_components::{DropdownItem, FilterableDropdown},
};

const DEFAULT_DROPDOWN_WIDTH: f32 = 380.;

pub struct BranchPicker {
    dropdown: ViewHandle<FilterableDropdown<String>>,
    selected: Option<String>,
}

impl BranchPicker {
    pub fn new(
        _cwd: Option<PathBuf>,
        default_value: Option<String>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self::new_with_style(None, default_value, None, ctx)
    }

    pub fn new_with_style(
        _cwd: Option<PathBuf>,
        default_value: Option<String>,
        style: Option<PickerStyle>,
        ctx: &mut ViewContext<Self>,
    ) -> Self {
        let width = style.as_ref().map_or(DEFAULT_DROPDOWN_WIDTH, |s| s.width);
        let bg = style.and_then(|s| s.background);
        let dropdown = ctx.add_typed_action_view(|ctx| {
            let mut dropdown = FilterableDropdown::new(ctx);
            dropdown.set_top_bar_max_width(width);
            dropdown.set_menu_width(width, ctx);
            if let Some(bg) = bg {
                dropdown.set_style(UiComponentStyles {
                    background: Some(bg.into()),
                    ..Default::default()
                });
            }
            dropdown
        });

        if let Some(default) = default_value.as_ref() {
            dropdown.update(ctx, |dropdown, ctx| {
                dropdown.set_items(
                    vec![DropdownItem::new(default.clone(), default.clone())],
                    ctx,
                );
                dropdown.set_selected_by_name(default, ctx);
            });
        }

        Self {
            dropdown,
            selected: default_value,
        }
    }

    pub fn refetch_branches(&mut self, _new_cwd: PathBuf, _ctx: &mut ViewContext<Self>) {}

    pub fn toggle_dropdown(&mut self, ctx: &mut ViewContext<Self>) -> bool {
        self.dropdown.update(ctx, |dropdown, ctx| {
            dropdown.toggle_expanded(ctx);
        });
        self.dropdown.as_ref(ctx).is_expanded()
    }

    pub fn selected_value(&self, app: &AppContext) -> Option<String> {
        self.dropdown
            .as_ref(app)
            .selected_item_label()
            .or_else(|| self.selected.clone())
    }

    pub fn is_loading(&self) -> bool {
        false
    }
}

impl Entity for BranchPicker {
    type Event = String;
}

impl View for BranchPicker {
    fn ui_name() -> &'static str {
        "BranchPicker"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        ChildView::new(&self.dropdown).finish()
    }
}

impl TypedActionView for BranchPicker {
    type Action = String;

    fn handle_action(&mut self, action: &String, ctx: &mut ViewContext<Self>) {
        self.selected = Some(action.clone());
        ctx.emit(action.clone());
    }
}
