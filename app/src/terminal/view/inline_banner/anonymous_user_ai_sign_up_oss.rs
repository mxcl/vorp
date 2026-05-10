use crate::appearance::Appearance;
use crate::terminal::view::InlineBannerId;
use warpui::prelude::{Empty, MouseStateHandle};
use warpui::Element;

#[derive(Clone, Copy, Debug)]
pub enum AnonymousUserLoginBannerAction {
    SignUp,
    Close,
}

pub struct AnonymousUserAISignUpBannerState {
    pub id: InlineBannerId,
    pub sign_up_button_mouse_state: MouseStateHandle,
    pub close_button_mouse_state: MouseStateHandle,
}

impl AnonymousUserAISignUpBannerState {
    pub fn new(id: InlineBannerId) -> Self {
        Self {
            id,
            sign_up_button_mouse_state: Default::default(),
            close_button_mouse_state: Default::default(),
        }
    }

    pub fn render(&self, _appearance: &Appearance) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
