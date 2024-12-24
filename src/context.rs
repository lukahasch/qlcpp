use crate::Command;
use raylib::prelude::*;
use std::{ffi::CStr, ops::Deref};

pub trait Ctx: RaylibDraw + RaylibDrawGui + RaylibScissorModeExt {}

impl<T: RaylibDraw + RaylibDrawGui + RaylibScissorModeExt> Ctx for T {}

pub struct Context<'a, T: Ctx> {
    pub rect: Rectangle,
    pub char: Option<char>,
    pub actions: &'a mut Vec<Box<dyn Command>>,
    pub draw: T,
}

impl<'a> Context<'a, RaylibDrawHandle<'a>> {
    pub fn new(
        rl: &'a mut RaylibHandle,
        thread: &RaylibThread,
        actions: &'a mut Vec<Box<dyn Command>>,
    ) -> Self {
        let rect = Rectangle::new(
            0.0,
            0.0,
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        );
        let char = rl.get_char_pressed();
        let mut draw = rl.begin_drawing(thread);
        draw.clear_background(crate::ui::BACKGROUND_COLOR);
        Self {
            rect,
            draw,
            char,
            actions,
        }
    }
}

impl<'a, T: Ctx> Context<'a, T> {
    /// use unsafe ffi to measure text width
    pub fn measure_text(&self, text: &str, font_size: i32) -> i32 {
        let text = std::ffi::CString::new(text).unwrap();
        unsafe { ffi::MeasureText(text.into_raw(), font_size) }
    }

    pub fn scissor<'b: 'a>(&'b mut self, rect: Rectangle) -> Context<'a, RaylibScissorMode<'b, T>> {
        Context {
            rect,
            char: self.char,
            actions: self.actions,
            draw: self.draw.begin_scissor_mode(
                rect.x as i32,
                rect.y as i32,
                rect.width as i32,
                rect.height as i32,
            ),
        }
    }

    /// Detect if a key has been pressed once.
    #[inline]
    pub fn is_key_pressed(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed((key as u32) as i32) }
    }

    /// Detect if a key is being pressed.
    #[inline]
    pub fn is_key_down(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown((key as u32) as i32) }
    }

    /// Detect if a key has been released once.
    #[inline]
    pub fn is_key_released(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased((key as u32) as i32) }
    }

    /// Detect if a key is NOT being pressed.
    #[inline]
    pub fn is_key_up(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp((key as u32) as i32) }
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed(&mut self) -> Option<crate::consts::KeyboardKey> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            return key_from_i32(key);
        }
        None
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed_number(&mut self) -> Option<u32> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            return Some(key as u32);
        }
        None
    }

    /// Gets latest char (unicode) pressed
    #[inline]
    pub fn get_char_pressed(&mut self) -> Option<char> {
        let char_code = unsafe { ffi::GetCharPressed() };
        if char_code > 0 {
            return char::from_u32(char_code as u32);
        }
        None
    }

    /// Sets a custom key to exit program (default is ESC).
    // #[inline]
    pub fn set_exit_key(&mut self, key: Option<crate::consts::KeyboardKey>) {
        unsafe {
            match key {
                Some(k) => ffi::SetExitKey((k as u32) as i32),
                None => ffi::SetExitKey(0),
            }
        }
    }

    /// Detect if a gamepad is available.
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: i32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad) }
    }

    /// Returns gamepad internal name id.
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: i32) -> Option<String> {
        unsafe {
            let name = ffi::GetGamepadName(gamepad);
            match name.is_null() {
                false => match CStr::from_ptr(name).to_str() {
                    Ok(a) => Some(a.to_owned()),
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                        None
                    }
                },
                true => None,
            }
        }
    }

    /// Detect if a gamepad button has been pressed once.
    #[inline]
    pub fn is_gamepad_button_pressed(
        &self,
        gamepad: i32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonPressed(gamepad, button as i32) }
    }

    /// Detect if a gamepad button is being pressed.
    #[inline]
    pub fn is_gamepad_button_down(
        &self,
        gamepad: i32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonDown(gamepad, button as i32) }
    }

    /// Detect if a gamepad button has been released once.
    #[inline]
    pub fn is_gamepad_button_released(
        &self,
        gamepad: i32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonReleased(gamepad, button as i32) }
    }

    /// Detect if a gamepad button is NOT being pressed.
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: i32, button: crate::consts::GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonUp(gamepad, button as i32) }
    }

    /// Gets the last gamepad button pressed.
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<crate::consts::GamepadButton> {
        let button = unsafe { ffi::GetGamepadButtonPressed() };
        if button != GamepadButton::GAMEPAD_BUTTON_UNKNOWN as i32 {
            return Some(unsafe { std::mem::transmute(button as u32) });
        }
        None
    }

    /// Returns gamepad axis count for a gamepad.
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: i32) -> i32 {
        unsafe { ffi::GetGamepadAxisCount(gamepad) }
    }

    /// Returns axis movement value for a gamepad axis.
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: i32, axis: crate::consts::GamepadAxis) -> f32 {
        unsafe { ffi::GetGamepadAxisMovement(gamepad, axis as i32) }
    }

    /// Detect if a mouse button has been pressed once.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Detect if a mouse button is being pressed.
    #[inline]
    pub fn is_mouse_button_down(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Detect if a mouse button has been released once.
    #[inline]
    pub fn is_mouse_button_released(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Detect if a mouse button is NOT being pressed.
    #[inline]
    pub fn is_mouse_button_up(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Returns mouse position X.
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { ffi::GetMouseX() }
    }

    /// Returns mouse position Y.
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { ffi::GetMouseY() }
    }

    /// Returns mouse position.
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Returns mouse delta between frames.
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2 {
        unsafe { ffi::GetMouseDelta().into() }
    }

    /// Sets mouse position.
    #[inline]
    pub fn set_mouse_position(&mut self, position: impl Into<Vector2>) {
        unsafe {
            let Vector2 { x, y } = position.into();
            ffi::SetMousePosition(x as i32, y as i32);
        }
    }

    /// Sets mouse offset.
    #[inline]
    pub fn set_mouse_offset(&mut self, offset: impl Into<Vector2>) {
        unsafe {
            let Vector2 { x, y } = offset.into();
            ffi::SetMouseOffset(x as i32, y as i32);
        }
    }

    /// Sets mouse scaling.
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::SetMouseScale(scale_x, scale_y);
        }
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Get mouse wheel movement for both X and Y
    #[inline]
    pub fn get_mouse_wheel_move_v(&self) -> Vector2 {
        unsafe { ffi::GetMouseWheelMoveV() }.into()
    }

    /// Returns touch position X for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe { ffi::GetTouchX() }
    }

    /// Returns touch position Y for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe { ffi::GetTouchY() }
    }

    /// Returns touch position XY for a touch point index (relative to screen size).
    #[inline]
    pub fn get_touch_position(&self, index: u32) -> Vector2 {
        unsafe { ffi::GetTouchPosition(index as i32).into() }
    }

    /// Enables a set of gestures using flags.
    #[inline]
    pub fn set_gestures_enabled(&self, gesture_flags: u32) {
        unsafe {
            ffi::SetGesturesEnabled(gesture_flags as u32);
        }
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    pub fn set_gamepad_mappings(&self, bind: &[i8]) -> i32 {
        unsafe { ffi::SetGamepadMappings(bind.as_ptr()) }
    }

    /// Checks if a gesture have been detected.
    #[inline]
    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        unsafe { ffi::IsGestureDetected(gesture as u32) }
    }

    /// Gets latest detected gesture.
    #[inline]
    pub fn get_gesture_detected(&self) -> Gesture {
        unsafe { std::mem::transmute(ffi::GetGestureDetected()) }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: u32) -> i32 {
        unsafe { ffi::GetTouchPointId(index as i32) }
    }

    /// Gets touch points count.
    #[inline]
    pub fn get_touch_point_count(&self) -> u32 {
        unsafe { ffi::GetTouchPointCount() as u32 }
    }

    /// Gets gesture hold time in milliseconds.
    #[inline]
    pub fn get_gesture_hold_duration(&self) -> f32 {
        unsafe { ffi::GetGestureHoldDuration() }
    }

    /// Gets gesture drag vector.
    #[inline]
    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        unsafe { ffi::GetGestureDragVector().into() }
    }

    /// Gets gesture drag angle.
    #[inline]
    pub fn get_gesture_drag_angle(&self) -> f32 {
        unsafe { ffi::GetGestureDragAngle() }
    }

    /// Gets gesture pinch delta.
    #[inline]
    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        unsafe { ffi::GetGesturePinchVector().into() }
    }

    /// Gets gesture pinch angle.
    #[inline]
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe { ffi::GetGesturePinchAngle() }
    }
}

impl<'a, T: Ctx> RaylibDraw for Context<'a, T> {
    /// Sets background color (framebuffer clear color.into()).
    #[inline]
    fn clear_background(&mut self, color: impl Into<ffi::Color>) {
        self.draw.clear_background(color)
    }

    /// Define default texture used to draw shapes
    fn set_shapes_texture(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source: impl Into<ffi::Rectangle>,
    ) {
        self.draw.set_shapes_texture(texture, source)
    }

    // SHAPES
    /// Draws a pixel.
    #[inline]
    fn draw_pixel(&mut self, x: i32, y: i32, color: impl Into<ffi::Color>) {
        self.draw
            .draw_pixel((self.rect.x as i32 + x), (self.rect.y as i32 + y), color)
    }

    /// Draws a pixel (Vector version).
    #[inline]
    fn draw_pixel_v(&mut self, position: impl Into<ffi::Vector2>, color: impl Into<ffi::Color>) {
        let pos = position.into();
        self.draw.draw_pixel_v(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            color,
        )
    }

    /// Draws a line.
    #[inline]
    fn draw_line(
        &mut self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_line(
            (self.rect.x as i32 + start_pos_x),
            (self.rect.y as i32 + start_pos_y),
            (self.rect.x as i32 + end_pos_x),
            (self.rect.y as i32 + end_pos_y),
            color,
        )
    }

    /// Draws a line (Vector version).
    #[inline]
    fn draw_line_v(
        &mut self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        let start = start_pos.into();
        let end = end_pos.into();
        self.draw.draw_line_v(
            Vector2::new(self.rect.x + start.x, self.rect.y + start.y),
            Vector2::new(self.rect.x + end.x, self.rect.y + end.y),
            color,
        )
    }

    /// Draws a line with thickness.
    #[inline]
    fn draw_line_ex(
        &mut self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        let start = start_pos.into();
        let end = end_pos.into();
        self.draw.draw_line_ex(
            Vector2::new(self.rect.x + start.x, self.rect.y + start.y),
            Vector2::new(self.rect.x + end.x, self.rect.y + end.y),
            thick,
            color,
        )
    }

    /// Draws a line using cubic-bezier curves in-out.
    #[inline]
    fn draw_line_bezier(
        &mut self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        let start = start_pos.into();
        let end = end_pos.into();
        self.draw.draw_line_bezier(
            Vector2::new(self.rect.x + start.x, self.rect.y + start.y),
            Vector2::new(self.rect.x + end.x, self.rect.y + end.y),
            thick,
            color,
        )
    }

    /// Draw lines sequence
    #[inline]
    fn draw_line_strip(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        let mut offset_points = Vec::with_capacity(points.len());
        for point in points {
            offset_points.push(Vector2::new(self.rect.x + point.x, self.rect.y + point.y));
        }
        self.draw.draw_line_strip(&offset_points, color)
    }

    /// Draws a color-filled circle.
    #[inline]
    fn draw_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_circle(
            (self.rect.x as i32 + center_x),
            (self.rect.y as i32 + center_y),
            radius,
            color,
        )
    }

    /// Draw a piece of a circle
    #[inline]
    fn draw_circle_sector(
        &mut self,
        center: impl Into<ffi::Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_circle_sector(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    /// Draw circle sector outline
    #[inline]
    fn draw_circle_sector_lines(
        &mut self,
        center: impl Into<ffi::Vector2>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_circle_sector_lines(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    /// Draws a gradient-filled circle.
    #[inline]
    fn draw_circle_gradient(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        self.draw.draw_circle_gradient(
            (self.rect.x as i32 + center_x),
            (self.rect.y as i32 + center_y),
            radius,
            color1,
            color2,
        )
    }

    /// Draws a color-filled circle (Vector version).
    #[inline]
    fn draw_circle_v(
        &mut self,
        center: impl Into<ffi::Vector2>,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_circle_v(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            radius,
            color,
        )
    }

    /// Draws circle outline.
    #[inline]
    fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_circle_lines(
            (self.rect.x as i32 + center_x),
            (self.rect.y as i32 + center_y),
            radius,
            color,
        )
    }

    /// Draws ellipse.
    #[inline]
    fn draw_ellipse(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_ellipse(
            (self.rect.x as i32 + center_x),
            (self.rect.y as i32 + center_y),
            radius_h,
            radius_v,
            color,
        )
    }

    /// Draws ellipse outline.
    #[inline]
    fn draw_ellipse_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_ellipse_lines(
            (self.rect.x as i32 + center_x),
            (self.rect.y as i32 + center_y),
            radius_h,
            radius_v,
            color,
        )
    }

    /// Draw ring
    #[inline]
    fn draw_ring(
        &mut self,
        center: impl Into<ffi::Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_ring(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    /// Draw ring lines
    #[inline]
    fn draw_ring_lines(
        &mut self,
        center: impl Into<ffi::Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_ring_lines(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    /// Draws a color-filled rectangle.
    #[inline]
    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_rectangle(
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            width,
            height,
            color,
        )
    }

    /// Draws a color-filled rectangle (Vector version).
    #[inline]
    fn draw_rectangle_v(
        &mut self,
        position: impl Into<ffi::Vector2>,
        size: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_rectangle_v(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            size,
            color,
        )
    }

    /// Draws a color-filled rectangle from `rec`.
    #[inline]
    fn draw_rectangle_rec(&mut self, rec: impl Into<ffi::Rectangle>, color: impl Into<ffi::Color>) {
        let rect = rec.into();
        self.draw.draw_rectangle_rec(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            color,
        )
    }

    /// Draws a color-filled rectangle with pro parameters.
    #[inline]
    fn draw_rectangle_pro(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        origin: impl Into<ffi::Vector2>,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        let rect = rec.into();
        self.draw.draw_rectangle_pro(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            origin,
            rotation,
            color,
        )
    }

    /// Draws a vertical-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_v(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        self.draw.draw_rectangle_gradient_v(
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            width,
            height,
            color1,
            color2,
        )
    }

    /// Draws a horizontal-gradient-filled rectangle.
    ///
    /// **NOTE**: Gradient goes from bottom (`color1`) to top (`color2`).
    #[inline]
    fn draw_rectangle_gradient_h(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: impl Into<ffi::Color>,
        color2: impl Into<ffi::Color>,
    ) {
        self.draw.draw_rectangle_gradient_h(
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            width,
            height,
            color1,
            color2,
        )
    }

    /// Draws a gradient-filled rectangle with custom vertex colors.
    ///
    /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
    #[inline]
    fn draw_rectangle_gradient_ex(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        col1: impl Into<ffi::Color>,
        col2: impl Into<ffi::Color>,
        col3: impl Into<ffi::Color>,
        col4: impl Into<ffi::Color>,
    ) {
        let rect = rec.into();
        self.draw.draw_rectangle_gradient_ex(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            col1,
            col2,
            col3,
            col4,
        )
    }

    /// Draws rectangle outline.
    #[inline]
    fn draw_rectangle_lines(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_rectangle_lines(
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            width,
            height,
            color,
        )
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_lines_ex(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        line_thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        let rect = rec.into();
        self.draw.draw_rectangle_lines_ex(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            line_thick,
            color,
        )
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        color: impl Into<ffi::Color>,
    ) {
        let rect = rec.into();
        self.draw.draw_rectangle_rounded(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            roundness,
            segments,
            color,
        )
    }

    /// Draws rectangle outline with extended parameters.
    #[inline]
    fn draw_rectangle_rounded_lines(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        roundness: f32,
        segments: i32,
        line_thickness: f32,
        color: impl Into<ffi::Color>,
    ) {
        let rect = rec.into();
        self.draw.draw_rectangle_rounded_lines(
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            roundness,
            segments,
            line_thickness,
            color,
        )
    }

    /// Draws a triangle.
    #[inline]
    fn draw_triangle(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        let pos1 = v1.into();
        let pos2 = v2.into();
        let pos3 = v3.into();
        self.draw.draw_triangle(
            Vector2::new(self.rect.x + pos1.x, self.rect.y + pos1.y),
            Vector2::new(self.rect.x + pos2.x, self.rect.y + pos2.y),
            Vector2::new(self.rect.x + pos3.x, self.rect.y + pos3.y),
            color,
        )
    }

    /// Draws a triangle using lines.
    #[inline]
    fn draw_triangle_lines(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        let pos1 = v1.into();
        let pos2 = v2.into();
        let pos3 = v3.into();
        self.draw.draw_triangle_lines(
            Vector2::new(self.rect.x + pos1.x, self.rect.y + pos1.y),
            Vector2::new(self.rect.x + pos2.x, self.rect.y + pos2.y),
            Vector2::new(self.rect.x + pos3.x, self.rect.y + pos3.y),
            color,
        )
    }

    /// Draw a triangle fan defined by points.
    #[inline]
    fn draw_triangle_fan(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        let mut offset_points = Vec::with_capacity(points.len());
        for point in points {
            offset_points.push(Vector2::new(self.rect.x + point.x, self.rect.y + point.y));
        }
        self.draw.draw_triangle_fan(&offset_points, color)
    }

    /// Draw a triangle strip defined by points
    #[inline]
    fn draw_triangle_strip(&mut self, points: &[Vector2], color: impl Into<ffi::Color>) {
        let mut offset_points = Vec::with_capacity(points.len());
        for point in points {
            offset_points.push(Vector2::new(self.rect.x + point.x, self.rect.y + point.y));
        }
        self.draw.draw_triangle_strip(&offset_points, color)
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly(
        &mut self,
        center: impl Into<ffi::Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_poly(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            sides,
            radius,
            rotation,
            color,
        )
    }

    /// Draws a regular polygon of n sides (Vector version).
    #[inline]
    fn draw_poly_lines(
        &mut self,
        center: impl Into<ffi::Vector2>,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: impl Into<ffi::Color>,
    ) {
        let pos = center.into();
        self.draw.draw_poly_lines(
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            sides,
            radius,
            rotation,
            color,
        )
    }

    /// Draws a `texture` using specified position and `tint` color.
    #[inline]
    fn draw_texture(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        x: i32,
        y: i32,
        tint: impl Into<ffi::Color>,
    ) {
        self.draw.draw_texture(
            texture,
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            tint,
        )
    }

    /// Draws a `texture` using specified `position` vector and `tint` color.
    #[inline]
    fn draw_texture_v(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<ffi::Vector2>,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_texture_v(
            texture,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            tint,
        )
    }

    /// Draws a `texture` with extended parameters.
    #[inline]
    fn draw_texture_ex(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        position: impl Into<ffi::Vector2>,
        rotation: f32,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_texture_ex(
            texture,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            rotation,
            scale,
            tint,
        )
    }

    /// Draws from a region of `texture` defined by the `source_rec` rectangle.
    #[inline]
    fn draw_texture_rec(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: impl Into<ffi::Rectangle>,
        position: impl Into<ffi::Vector2>,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_texture_rec(
            texture,
            source_rec,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            tint,
        )
    }

    /// Draw from a region of `texture` defined by the `source_rec` rectangle with pro parameters.
    #[inline]
    fn draw_texture_pro(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        source_rec: impl Into<ffi::Rectangle>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<ffi::Vector2>,
        rotation: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let rect = dest_rec.into();
        self.draw.draw_texture_pro(
            texture,
            source_rec,
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            origin,
            rotation,
            tint,
        )
    }

    ///Draws a texture (or part of it) that stretches or shrinks nicely
    #[inline]
    fn draw_texture_n_patch(
        &mut self,
        texture: impl AsRef<ffi::Texture2D>,
        n_patch_info: impl Into<ffi::NPatchInfo>,
        dest_rec: impl Into<ffi::Rectangle>,
        origin: impl Into<ffi::Vector2>,
        rotation: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let rect = dest_rec.into();
        self.draw.draw_texture_n_patch(
            texture,
            n_patch_info,
            Rectangle {
                x: self.rect.x + rect.x,
                y: self.rect.y + rect.y,
                width: rect.width,
                height: rect.height,
            },
            origin,
            rotation,
            tint,
        )
    }

    /// Shows current FPS.
    #[inline]
    fn draw_fps(&mut self, x: i32, y: i32) {
        self.draw
            .draw_fps((self.rect.x as i32 + x), (self.rect.y as i32 + y))
    }

    /// Draws text (using default font).
    /// This does not support UTF-8. Use `[RaylibDrawHandle::draw_text_codepoints]` for that.
    #[inline]
    fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        font_size: i32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_text(
            text,
            (self.rect.x as i32 + x),
            (self.rect.y as i32 + y),
            font_size,
            color,
        )
    }

    /// Draws text (using default font) with support for UTF-8.
    /// If you do not need UTF-8, use `[RaylibDrawHandle::draw_text]`.
    fn draw_text_codepoints(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        self.draw.draw_text_codepoints(
            font,
            text,
            Vector2::new(self.rect.x + position.x, self.rect.y + position.y),
            font_size,
            spacing,
            tint,
        )
    }

    /// Draws text using `font` and additional parameters.
    #[inline]
    fn draw_text_ex(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<ffi::Vector2>,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_text_ex(
            font,
            text,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            font_size,
            spacing,
            tint,
        )
    }

    fn draw_text_pro(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<ffi::Vector2>,
        origin: impl Into<ffi::Vector2>,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_text_pro(
            font,
            text,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            origin,
            rotation,
            font_size,
            spacing,
            tint,
        )
    }

    /// Draw one character (codepoint)
    #[inline]
    fn draw_text_codepoint(
        &mut self,
        font: impl AsRef<ffi::Font>,
        codepoint: i32,
        position: impl Into<ffi::Vector2>,
        scale: f32,
        tint: impl Into<ffi::Color>,
    ) {
        let pos = position.into();
        self.draw.draw_text_codepoint(
            font,
            codepoint,
            Vector2::new(self.rect.x + pos.x, self.rect.y + pos.y),
            scale,
            tint,
        )
    }

    /// Enable waiting for events when the handle is dropped, no automatic event polling
    fn enable_event_waiting(&self) {
        self.draw.enable_event_waiting()
    }

    /// Disable waiting for events when the handle is dropped, no automatic event polling
    fn disable_event_waiting(&self) {
        self.draw.disable_event_waiting()
    }

    /// Draw a polygon outline of n sides with extended parameters
    fn draw_poly_lines_ex(
        &mut self,
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        line_thick: f32,
        color: impl Into<ffi::Color>,
    ) {
        self.draw.draw_poly_lines_ex(
            Vector2::new(self.rect.x + center.x, self.rect.y + center.y),
            sides,
            radius,
            rotation,
            line_thick,
            color,
        )
    }

    /// Draw spline: Linear, minimum 2 points
    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: impl Into<ffi::Color>) {
        let mut offset_points = Vec::with_capacity(points.len());
        for point in points {
            offset_points.push(Vector2::new(self.rect.x + point.x, self.rect.y + point.y));
        }
        self.draw.draw_spline_linear(&offset_points, thick, color)
    }
}

impl<'a, T: Ctx> Deref for Context<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.draw
    }
}
