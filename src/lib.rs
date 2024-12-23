#![feature(macro_metavar_expr)]

use raylib::prelude::*;

pub mod dmx;
pub mod ui;

use dmx::DMX;
use ui::{EditableText, Pageable};

use crate::ui::Page;

pub trait Command {
    fn execute(&mut self, app: &mut Application);
}

pub struct Context<'a> {
    pub position: Vector2,

    pub width: f32,
    pub height: f32,

    pub actions: &'a mut Vec<Box<dyn Command>>,

    pub draw: RaylibDrawHandle<'a>,
}

impl<'a> Context<'a> {
    pub fn create(actions: &'a mut Vec<Box<dyn Command>>, draw: RaylibDrawHandle<'a>) -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            width: draw.get_screen_width() as f32,
            height: draw.get_screen_height() as f32,
            actions,
            draw,
        }
    }

    pub fn draw_text(&mut self, text: &str, x: f32, y: f32, size: i32, color: Color) {
        self.draw.draw_text(
            text,
            x as i32 + self.position.x as i32,
            y as i32 + self.position.y as i32,
            size,
            color,
        );
    }
}

pub struct Application {
    pub dmx: dmx::DMX,
    pub name: String,
    pub title: EditableText,

    pub actions: Vec<Box<dyn Command>>,
    pub from_top: usize,

    pub pages: Page<(Setup, Scenes)>,
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        Self {
            dmx: DMX::new(),
            name: String::from("Untitled"),
            title: EditableText::new(Vector2::new(10.0, 10.0), 20, Color::WHITE),
            actions: Vec::new(),
            from_top: 0,
            pages: Page::new((Setup::new(), Scenes::new())),
        }
    }

    pub fn run(&mut self) {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("QLCPP")
            .resizable()
            .build();

        rl.set_target_fps(60);
        rl.set_exit_key(Some(KeyboardKey::KEY_F12));

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::DIMGRAY.brightness(-0.2));

            let name = self.name.clone();

            let mut ctx = Context::create(&mut self.actions, d);

            self.title.draw(&mut self.name, &mut ctx);

            self.pages.draw(&mut ctx, &mut self.dmx);
            self.pages.update(&mut ctx, &mut self.dmx);
        }
    }
}

pub struct Setup {}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}

impl Setup {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Scenes {}

impl Default for Scenes {
    fn default() -> Self {
        Self::new()
    }
}

impl Scenes {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pageable for Setup {
    fn draw(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {}
    fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {}
}

impl Pageable for Scenes {
    fn draw(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {}
    fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {}
}
