use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(with = "vector2_wrapper")]
    position: Vector2,
    #[serde(with = "color_wrapper")]
    color: Color,
    font_size: i32,
    state: State,
    input_handler: Option<InputHandler>,
}

impl Text {
    pub fn new(position: Vector2, color: Color, font_size: i32) -> Self {
        Self {
            position,
            color,
            font_size,
            state: State::Normal,
            input_handler: None,
        }
    }

    pub fn rect<'a, D: RaylibDraw>(&self, text: &str, ctx: &mut Context<'a, D>) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            ctx.measure_text(text, self.font_size) as f32,
            self.font_size as f32,
        )
    }

    pub fn draw<'a, D: RaylibDraw>(&mut self, text: &mut String, ctx: &mut Context<'a, D>) {
        self.state.update(self.rect(text, ctx), ctx);
        if let Some(input_handler) = &mut self.input_handler {
            if !input_handler.update(text, ctx) {
                self.input_handler = None;
            }
            self.active(text, ctx);
        } else {
            self.inactive(text, ctx);
        }
    }

    pub fn active<'a, D: RaylibDraw>(&self, text: &str, ctx: &mut Context<'a, D>) {
        ctx.draw_text(
            text,
            self.position.x as i32,
            self.position.y as i32,
            self.font_size,
            ACCENT_COLOR,
        );
        let rect = self.rect(text, ctx).pad(5.0);
        ctx.draw_rectangle_lines_ex(rect, 1.0, ACCENT_COLOR);
        if let Some(input_handler) = &self.input_handler {
            input_handler.draw_cursor(text, self.position, self.font_size, ACCENT_COLOR, ctx);
        }
    }

    pub fn inactive<'a, D: RaylibDraw>(&mut self, text: &str, ctx: &mut Context<'a, D>) {
        match self.state {
            State::Normal => self.normal(text, ctx),
            State::Hovered => self.hover(text, ctx),
            State::Pressed => self.pressed(text, ctx),
        }
    }

    pub fn normal<'a, D: RaylibDraw>(&self, text: &str, ctx: &mut Context<'a, D>) {
        ctx.draw_text(
            text,
            self.position.x as i32,
            self.position.y as i32,
            self.font_size,
            self.color,
        );
    }

    pub fn hover<'a, D: RaylibDraw>(&self, text: &str, ctx: &mut Context<'a, D>) {
        ctx.draw_text(
            text,
            self.position.x as i32,
            self.position.y as i32,
            self.font_size,
            self.color,
        );
        let rect = self.rect(text, ctx).pad(5.0);
        ctx.draw_rectangle_lines_ex(rect, 1.0, self.color);
    }

    pub fn pressed<'a, D: RaylibDraw>(&mut self, text: &str, ctx: &mut Context<'a, D>) {
        self.input_handler = Some(InputHandler::new(text));
    }
}
