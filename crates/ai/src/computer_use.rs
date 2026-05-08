use std::{borrow::Cow, time::Duration};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vector2I {
    x: i32,
    y: i32,
}

impl Vector2I {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Platform {
    Mac,
    Windows,
    LinuxX11,
    LinuxWayland,
}

pub fn is_supported_on_current_platform() -> bool {
    false
}

pub fn create_actor() -> Box<dyn Actor> {
    Box::new(NoopActor)
}

#[async_trait]
pub trait Actor: Send + Sync + 'static {
    fn platform(&self) -> Option<Platform>;

    async fn perform_actions(
        &mut self,
        actions: &[Action],
        options: Options,
    ) -> Result<ActionResult, String>;
}

struct NoopActor;

#[async_trait]
impl Actor for NoopActor {
    fn platform(&self) -> Option<Platform> {
        None
    }

    async fn perform_actions(
        &mut self,
        _actions: &[Action],
        _options: Options,
    ) -> Result<ActionResult, String> {
        Err("computer use is not available in this build".to_owned())
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Key {
    Keycode(i32),
    Char(char),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Wait(Duration),
    MouseDown {
        button: MouseButton,
        at: Vector2I,
    },
    MouseUp {
        button: MouseButton,
    },
    MouseMove {
        to: Vector2I,
    },
    MouseWheel {
        at: Vector2I,
        direction: ScrollDirection,
        distance: ScrollDistance,
    },
    TypeText {
        text: String,
    },
    KeyDown {
        key: Key,
    },
    KeyUp {
        key: Key,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScrollDistance {
    Pixels(i32),
    Clicks(i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotRegion {
    pub top_left: Vector2I,
    pub bottom_right: Vector2I,
}

impl ScreenshotRegion {
    pub fn validate(&self) -> Result<(), String> {
        if self.top_left.x() < 0 || self.top_left.y() < 0 {
            return Err(format!(
                "Screenshot region top_left must be non-negative, got ({}, {})",
                self.top_left.x(),
                self.top_left.y()
            ));
        }
        if self.bottom_right.x() <= self.top_left.x() {
            return Err(format!(
                "Screenshot region must have positive width (bottom_right.x {} must be > top_left.x {})",
                self.bottom_right.x(),
                self.top_left.x()
            ));
        }
        if self.bottom_right.y() <= self.top_left.y() {
            return Err(format!(
                "Screenshot region must have positive height (bottom_right.y {} must be > top_left.y {})",
                self.bottom_right.y(),
                self.top_left.y()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotParams {
    pub max_long_edge_px: Option<usize>,
    pub max_total_px: Option<usize>,
    #[serde(default)]
    pub region: Option<ScreenshotRegion>,
}

pub struct Options {
    pub screenshot_params: Option<ScreenshotParams>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ActionResult {
    pub screenshot: Option<Screenshot>,
    pub cursor_position: Option<Vector2I>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Screenshot {
    pub width: usize,
    pub height: usize,
    pub original_width: usize,
    pub original_height: usize,
    pub data: Vec<u8>,
    pub mime_type: Cow<'static, str>,
}

impl std::fmt::Debug for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Screenshot")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("original_width", &self.original_width)
            .field("original_height", &self.original_height)
            .field("num_data_bytes", &self.data.len())
            .finish()
    }
}
