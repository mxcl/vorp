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

pub struct RepoPicker {
    dropdown: ViewHandle<FilterableDropdown<RepoPickerAction>>,
    selected: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepoPickerAction {
    Select(String),
    AddNewRepo,
}

pub enum RepoPickerEvent {
    Selected(String),
    RequestAddRepo,
}

impl RepoPicker {
    pub fn new(default_value: Option<String>, ctx: &mut ViewContext<Self>) -> Self {
        Self::new_with_style(default_value, None, ctx)
    }

    pub fn new_with_style(
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
                    vec![DropdownItem::new(
                        default.clone(),
                        RepoPickerAction::Select(default.clone()),
                    )],
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

    pub fn refresh_and_select(&mut self, path: PathBuf, ctx: &mut ViewContext<Self>) {
        let path_str = path.to_string_lossy().to_string();
        self.selected = Some(path_str.clone());
        self.dropdown.update(ctx, |dropdown, ctx| {
            dropdown.set_items(
                vec![DropdownItem::new(
                    path_str.clone(),
                    RepoPickerAction::Select(path_str.clone()),
                )],
                ctx,
            );
            dropdown.set_selected_by_name(&path_str, ctx);
        });
    }

    pub fn toggle_dropdown(&mut self, ctx: &mut ViewContext<Self>) -> bool {
        self.dropdown.update(ctx, |dropdown, ctx| {
            dropdown.toggle_expanded(ctx);
        });
        self.dropdown.as_ref(ctx).is_expanded()
    }

    pub fn selected_value(&self, app: &AppContext) -> Option<String> {
        self.selected
            .clone()
            .or_else(|| self.dropdown.as_ref(app).selected_item_label())
    }
}

impl Entity for RepoPicker {
    type Event = RepoPickerEvent;
}

impl View for RepoPicker {
    fn ui_name() -> &'static str {
        "RepoPicker"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        ChildView::new(&self.dropdown).finish()
    }
}

impl TypedActionView for RepoPicker {
    type Action = RepoPickerAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            RepoPickerAction::Select(value) => {
                self.selected = Some(value.clone());
                ctx.emit(RepoPickerEvent::Selected(value.clone()));
            }
            RepoPickerAction::AddNewRepo => {
                ctx.emit(RepoPickerEvent::RequestAddRepo);
            }
        }
    }
}
