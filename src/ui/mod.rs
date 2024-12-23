use raylib::prelude::*;

use crate::{dmx::DMX, Context};

pub struct Page<T: Pages> {
    pages: T,
    current: usize,
}

pub trait Pages {
    fn draw(&mut self, current: usize, ctx: &mut Context<'_>, dmx: &mut DMX);
    fn get<T: 'static>(&mut self) -> Option<&mut T>;
    fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX);
}

pub trait Pageable {
    fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX);
    fn draw(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {}
}

impl<T: Pages> Page<T> {
    pub fn new(pages: T) -> Self {
        Self { pages, current: 0 }
    }

    pub fn select(&mut self, index: usize) {
        self.current = index;
    }

    pub fn draw(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {
        self.pages.draw(self.current, ctx, dmx);
    }

    pub fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {
        self.pages.update(ctx, dmx);
    }
}

macro_rules! impl_pages {
    (@repeat $ig:ident $($name:ident)*) => {
        impl_pages!($($name)*);
    };
    (@repeat ) => {};

    ($($name:ident)*) => {
        #[allow(unused_variables)]
        impl<$($name),*> Pages for ($($name,)*)
        where
            $($name: Pageable + 'static),*
        {
            fn draw(&mut self, current: usize, ctx: &mut Context<'_>, dmx: &mut DMX) {
                match current {
                    $(
                        ${index()} => self.${index()}.draw(ctx, dmx),
                        ${ignore($name)}
                    )*
                    _ => {}
                }
            }

            fn update(&mut self, ctx: &mut Context<'_>, dmx: &mut DMX) {
                $(
                    self.${index()}.update(ctx, dmx);
                    ${ignore($name)}
                )*
            }

            fn get<TT: 'static>(&mut self) -> Option<&mut TT> {
                $(
                    if std::any::TypeId::of::<TT>() == std::any::TypeId::of::<$name>() {
                        return unsafe { std::mem::transmute(Some(&mut self.${index()})) };
                    }
                )*
                return None;
            }
        }

        impl_pages!(@repeat $($name)*);
    };
}

impl_pages!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);

pub enum State {
    Normal,
    Hovered,
    Selected,
    Forced(Box<State>),
}

impl State {
    pub fn selected(&self) -> bool {
        match self {
            State::Selected => true,
            _ => false,
        }
    }
}

pub struct EditableText {
    position: Vector2,
    font_size: i32,
    color: Color,
    state: State,
}

impl EditableText {
    pub fn new(position: Vector2, font_size: i32, color: Color) -> Self {
        Self {
            position,
            font_size,
            color,
            state: State::Normal,
        }
    }

    pub fn draw(&mut self, text: &mut String, ctx: &mut Context) {
        let text = if self.state.selected() {
            format!("{}|", text)
        } else {
            text.clone()
        };
        ctx.draw_text(
            &text,
            self.position.x,
            self.position.y,
            self.font_size,
            self.color,
        );
    }
}
