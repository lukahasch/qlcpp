#![feature(macro_metavar_expr)]

use raylib::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub mod context;
pub mod dmx;
pub mod ui;

use context::Context;
use dmx::DMX;
use ui::{
    page::{Page, Pageable, Pages},
    text::Text,
    TEXT_COLOR, TEXT_SIZE,
};

pub trait Command {
    fn execute(&mut self, app: &mut Application);
}

#[derive(Serialize, Deserialize)]
pub struct Application {
    pub dmx: dmx::DMX,
    pub name: String,
    pub title: Text,

    #[serde(skip, default)]
    pub actions: Vec<Box<dyn Command>>,
    #[serde(skip, default)]
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
            title: Text::new(Vector2::new(10.0, 10.0), TEXT_COLOR, TEXT_SIZE),
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
            let mut ctx = Context::new(&mut rl, &thread, &mut self.actions);
            self.title.draw(&mut self.name, &mut ctx);
            self.pages.draw(&mut ctx, &mut self.dmx);
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        std::fs::write(format!("{}.json", self.name), data).unwrap();
    }

    pub fn load(&mut self) {
        let data = std::fs::read_to_string(format!("{}.json", self.name)).unwrap();
        *self = serde_json::from_str(&data).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Pageable for Setup {
    fn draw<'a, T: RaylibDraw>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX) {}
    fn update<'a, T: RaylibDraw>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX) {}
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Pageable for Scenes {
    fn draw<'a, T: RaylibDraw>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX) {}
    fn update<'a, T: RaylibDraw>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX) {}
}
