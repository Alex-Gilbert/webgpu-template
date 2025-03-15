use bevy_ecs::system::Resource;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};
use winit::{event::MouseButton, keyboard::KeyCode};

#[derive(Resource)]
pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

#[derive(Debug)]
pub struct KeyState {
    pub pressed: bool,
    pub released: bool,
    pub held: bool,
}

impl KeyState {
    fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            held: false,
        }
    }

    pub fn was_pressed_this_frame(&self) -> bool {
        self.pressed
    }

    pub fn was_released_this_frame(&self) -> bool {
        self.released
    }

    pub fn is_held(&self) -> bool {
        self.held
    }

    pub fn press(&mut self) {
        self.pressed = true;
        self.held = true;
    }

    pub fn release(&mut self) {
        self.released = true;
        self.held = false;
    }

    pub fn update(&mut self) {
        if self.pressed {
            self.pressed = false;
        }

        if self.released {
            self.released = false;
        }
    }
}

pub struct Keyboard {
    pub keys: HashMap<KeyCode, KeyState>,
}
impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn get_key(&self, key: KeyCode) -> Option<&KeyState> {
        self.keys.get(&key)
    }

    pub fn get_or_insert_key(&mut self, key: KeyCode) -> &mut KeyState {
        self.keys.entry(key).or_insert(KeyState::new())
    }

    pub fn update(&mut self) {
        for (_, key) in self.keys.iter_mut() {
            key.update();
        }
    }
}

pub enum MouseButtonState {
    Up,
    PressedThisFrame { start_x: f64, start_y: f64 },
    Down { start_x: f64, start_y: f64 },
    Dragged { start_x: f64, start_y: f64 },
    ReleasedPressThisFrame,
    ReleasedDragThisFrame { start_x: f64, start_y: f64 },
}

impl MouseButtonState {
    fn new() -> Self {
        Self::Up
    }

    pub fn was_pressed_this_frame(&self) -> bool {
        matches!(self, Self::PressedThisFrame { .. })
    }

    pub fn was_released_this_frame(&self) -> bool {
        matches!(
            self,
            Self::ReleasedPressThisFrame | Self::ReleasedDragThisFrame { .. }
        )
    }

    pub fn down(&self) -> bool {
        matches!(
            self,
            Self::PressedThisFrame { .. } | Self::Down { .. } | Self::Dragged { .. }
        )
    }

    pub fn dragging(&self) -> Option<(f64, f64)> {
        match self {
            Self::Dragged { start_x, start_y } => Some((*start_x, *start_y)),
            _ => None,
        }
    }

    pub fn drag_released(&self) -> Option<(f64, f64)> {
        match self {
            Self::ReleasedDragThisFrame { start_x, start_y } => Some((*start_x, *start_y)),
            _ => None,
        }
    }

    fn press(&mut self, x: f64, y: f64) {
        *self = Self::PressedThisFrame {
            start_x: x,
            start_y: y,
        };
    }

    pub fn release(&mut self) {
        *self = match self {
            Self::Up => Self::Up,
            Self::Dragged { start_x, start_y } => Self::ReleasedDragThisFrame {
                start_x: *start_x,
                start_y: *start_y,
            },
            _ => Self::ReleasedPressThisFrame,
        }
    }

    pub fn clicked(&self) -> bool {
        matches!(self, Self::ReleasedPressThisFrame)
    }

    fn update(&mut self, x: f64, y: f64) {
        const MIN_DISTANCE_2_FOR_DRAG: f64 = 3.0 * 3.0;
        *self = match *self {
            Self::ReleasedPressThisFrame => Self::Up,
            Self::ReleasedDragThisFrame { .. } => Self::Up,
            Self::PressedThisFrame { start_x, start_y } | Self::Down { start_x, start_y } => {
                let dx = x - start_x;
                let dy = y - start_y;
                if dx * dx + dy * dy > MIN_DISTANCE_2_FOR_DRAG {
                    Self::Dragged { start_x, start_y }
                } else {
                    Self::Down { start_x, start_y }
                }
            }
            Self::Dragged { start_x, start_y } => Self::Dragged { start_x, start_y },
            Self::Up => Self::Up,
        }
    }
}

pub struct MouseButtonStateMut<'a> {
    state: &'a mut MouseButtonState,
    x: f64,
    y: f64,
}

impl MouseButtonStateMut<'_> {
    pub fn press(&mut self) {
        self.state.press(self.x, self.y);
    }

    pub fn update(&mut self) {
        self.state.update(self.x, self.y);
    }
}

impl Deref for MouseButtonStateMut<'_> {
    type Target = MouseButtonState;

    fn deref(&self) -> &Self::Target {
        self.state
    }
}

impl DerefMut for MouseButtonStateMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state
    }
}

pub struct Mouse {
    pub x: f64,
    pub y: f64,
    pub delta_x: f64,
    pub delta_y: f64,
    pub delta_scroll_x: f64,
    pub delta_scroll_y: f64,
    pub buttons: HashMap<MouseButton, MouseButtonState>,
}
impl Mouse {
    fn new() -> Mouse {
        Self {
            x: 0.0,
            y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            delta_scroll_x: 0.0,
            delta_scroll_y: 0.0,
            buttons: HashMap::new(),
        }
    }

    pub fn get_button(&self, button: MouseButton) -> Option<&MouseButtonState> {
        self.buttons.get(&button)
    }

    pub fn get_or_insert_button(&mut self, button: MouseButton) -> MouseButtonStateMut {
        let state = self
            .buttons
            .entry(button)
            .or_insert(MouseButtonState::new());
        MouseButtonStateMut {
            state,
            x: self.x,
            y: self.y,
        }
    }

    pub fn set_position(&mut self, x: f64, y: f64) {
        self.delta_x = x - self.x;
        self.delta_y = y - self.y;
        self.x = x;
        self.y = y;
    }

    pub fn set_scroll(&mut self, scroll_x: f64, scroll_y: f64) {
        self.delta_scroll_x = scroll_x;
        self.delta_scroll_y = scroll_y;
    }

    pub fn update(&mut self) {
        self.delta_x = 0.0;
        self.delta_y = 0.0;
        self.delta_scroll_x = 0.0;
        self.delta_scroll_y = 0.0;

        for (_button, state) in self.buttons.iter_mut() {
            state.update(self.x, self.y);
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn update(&mut self) {
        self.mouse.update();
        self.keyboard.update();
    }
}
