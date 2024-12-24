pub use raylib::prelude::*;
pub use serde_derive::{Deserialize, Serialize};

use crate::context::Ctx;
pub use crate::{context::Context, dmx::DMX};

pub mod page;
pub mod text;

pub const BACKGROUND_COLOR: Color = Color {
    r: 30,
    g: 30,
    b: 46,
    a: 255,
};
pub const TEXT_COLOR: Color = Color {
    r: 205,
    g: 214,
    b: 244,
    a: 255,
};
pub const ACCENT_COLOR: Color = Color {
    r: 202,
    g: 165,
    b: 245,
    a: 255,
};
pub const TEXT_SIZE: i32 = 20;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum State {
    Normal,
    Hovered,
    Pressed,
}

impl State {
    pub fn pressed(&self) -> bool {
        match self {
            State::Pressed => true,
            _ => false,
        }
    }

    pub fn hovered(&self) -> bool {
        match self {
            State::Hovered => true,
            _ => false,
        }
    }
    pub fn normal(&self) -> bool {
        match self {
            State::Normal => true,
            _ => false,
        }
    }

    pub fn update<'a, D: Ctx>(&mut self, rect: Rectangle, ctx: &mut Context<'a, D>) {
        if rect.check_collision_point_rec(ctx.get_mouse_position()) {
            *self = State::Hovered;
        } else {
            *self = State::Normal;
        }
        if *self == State::Hovered && ctx.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            *self = State::Pressed;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputHandler {
    pub cursor: usize,
    pub timer: u32,
    #[serde(with = "option_and_u32_keyboardkey_wrapper")]
    pub repeat: Option<(KeyboardKey, u32)>,
}

impl InputHandler {
    pub fn new(text: &str) -> Self {
        Self {
            cursor: text.len(),
            timer: 0,
            repeat: None,
        }
    }

    pub fn draw_cursor<'a, D: Ctx>(
        &self,
        text: &str,
        position: Vector2,
        font_size: i32,
        color: Color,
        ctx: &mut Context<'a, D>,
    ) {
        if self.timer % 60 < 30 {
            let cursor_position = ctx.measure_text(&text[..self.cursor], font_size) as f32;
            ctx.draw_text(
                "|",
                (position.x + cursor_position) as i32,
                (position.y) as i32,
                font_size,
                TEXT_COLOR,
            );
        }
    }

    pub fn update<'a, D: Ctx>(&mut self, text: &mut String, ctx: &mut Context<'a, D>) -> bool {
        self.timer += 1;
        if let Some((_, time)) = &mut self.repeat {
            *time += 1;
        }
        if ctx.is_key_pressed(KeyboardKey::KEY_ENTER) {
            return false;
        } else if self.hot_keys(text, ctx) {
            return true;
        } else {
            if let Some(char) = ctx.char {
                text.insert(self.cursor, char);
                self.cursor += 1;
            }
        }
        true
    }

    pub fn hot_keys<'a, D: Ctx>(&mut self, text: &mut String, ctx: &mut Context<'a, D>) -> bool {
        if self.key(KeyboardKey::KEY_DELETE, ctx) || self.key(KeyboardKey::KEY_BACKSPACE, ctx) {
            if self.cursor > 0 {
                text.remove(self.cursor - 1);
                self.cursor -= 1;
            }
            return true;
        } else if self.key(KeyboardKey::KEY_LEFT, ctx) {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
            return true;
        } else if self.key(KeyboardKey::KEY_RIGHT, ctx) {
            if self.cursor < text.len() {
                self.cursor += 1;
            }
            return true;
        }
        false
    }

    /// check if a key should be counted as a hot key
    /// if the key is pressed and repeat high output more keys
    /// if the key is pressed and repeat low output one key
    pub fn key<'a, D: Ctx>(&mut self, key: KeyboardKey, ctx: &mut Context<'a, D>) -> bool {
        match (ctx.is_key_pressed(key), &mut self.repeat) {
            (true, _) => {
                self.repeat = Some((key, 0));
                true
            }
            (false, Some((k, time))) if *k == key => {
                if ctx.is_key_down(key) {
                    if *time % 10 == 0 {
                        true
                    } else {
                        false
                    }
                } else {
                    self.repeat = None;
                    false
                }
            }
            _ => false,
        }
    }
}

mod vector2_wrapper {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(vector: &Vector2, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (vector.x, vector.y).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vector2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y) = <(f32, f32)>::deserialize(deserializer)?;
        Ok(Vector2::new(x, y))
    }
}
mod color_wrapper {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (color.r, color.g, color.b, color.a).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (r, g, b, a) = <(u8, u8, u8, u8)>::deserialize(deserializer)?;
        Ok(Color::new(r, g, b, a))
    }
}

mod rectangle_wrapper {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(rectangle: &Rectangle, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (rectangle.x, rectangle.y, rectangle.width, rectangle.height).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rectangle, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (x, y, width, height) = <(f32, f32, f32, f32)>::deserialize(deserializer)?;
        Ok(Rectangle::new(x, y, width, height))
    }
}

mod option_and_u32_keyboardkey_wrapper {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(key: &Option<(KeyboardKey, u32)>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match key {
            Some((key, time)) => Some((*key as i32, *time)).serialize(serializer),
            None => Option::<(i32, u32)>::serialize(&None, serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<(KeyboardKey, u32)>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key = <Option<(i32, u32)>>::deserialize(deserializer)?;
        Ok(key.map(|(key, time)| (key_from_i32(key).unwrap(), time)))
    }
}

mod keyboardkey_wrapper {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(key: &KeyboardKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*key as i32).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyboardKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key = <i32>::deserialize(deserializer)?;
        Ok(key_from_i32(key).unwrap())
    }
}

pub trait Pad {
    fn pad(self, by: f32) -> Self;
}

impl Pad for Rectangle {
    fn pad(self, by: f32) -> Self {
        Rectangle {
            x: self.x - by,
            y: self.y - by,
            width: self.width + by * 2.0,
            height: self.height + by * 2.0,
        }
    }
}
