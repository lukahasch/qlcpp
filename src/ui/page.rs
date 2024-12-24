use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T: Pages> {
    pages: T,
    current: usize,
}

pub trait Pages {
    fn draw<'a, T: Ctx>(&mut self, current: usize, ctx: &mut Context<'a, T>, dmx: &mut DMX);
    fn get<T: 'static>(&mut self) -> Option<&mut T>;
    fn update<'a, T: Ctx>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX);
}

pub trait Pageable {
    fn update<'a, T: Ctx>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX);
    fn draw<'a, T: Ctx>(&mut self, ctx: &mut Context<'a, T>, dmx: &mut DMX) {}
}

impl<T: Pages> Page<T> {
    pub fn new(pages: T) -> Self {
        Self { pages, current: 0 }
    }

    pub fn select(&mut self, index: usize) {
        self.current = index;
    }

    pub fn draw<'a, D: Ctx>(&mut self, ctx: &mut Context<'a, D>, dmx: &mut DMX) {
        self.pages.draw(self.current, ctx, dmx);
    }

    pub fn update<'a, D: Ctx>(&mut self, ctx: &mut Context<'a, D>, dmx: &mut DMX) {
        self.pages.update(ctx, dmx);
    }

    pub fn selector<'a, D: Ctx>(&mut self, ctx: &mut Context<'a, D>) {}
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
            fn draw<'a, TT: Ctx>(&mut self, current: usize, ctx: &mut Context<'a, TT>, dmx: &mut DMX) {
                match current {
                    $(
                        ${index()} => self.${index()}.draw(ctx, dmx),
                        ${ignore($name)}
                    )*
                    _ => {}
                }
            }

            fn update<'a, TT: Ctx>(&mut self, ctx: &mut Context<'a, TT>, dmx: &mut DMX) {
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
