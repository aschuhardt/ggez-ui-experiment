use std;
use std::collections::HashMap;

use ggez::graphics;
use ggez::graphics::{Color, Font, Text, Rect, Point};
use ggez::Context;
use mopa;

use substate::states::StateInfo;

pub struct UIContext {
    elements: HashMap<&'static str, Box<UIElement>>,
}

impl UIContext {
    pub fn new() -> UIContext {
        UIContext { elements: HashMap::<&'static str, Box<UIElement>>::new() }
    }

    pub fn add_element<T>(&mut self, name: &'static str, element: Box<T>)
    where
        T: UIElement + 'static,
    {
        self.elements.insert(name, element);
    }

    pub fn mouse_moved(&mut self, x: i32, y: i32) {
        for e in self.elements.values_mut() {
            e.hover(x, y);
        }
    }

    pub fn click(&mut self, x: i32, y: i32, info: &mut StateInfo) {
        for e in self.elements.values_mut() {
            e.click(x, y, info);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for e in self.elements.values_mut() {
            e.draw(ctx);
        }
    }

    pub fn modify_element<F, T>(&mut self, name: &'static str, mut op: F)
    where
        F: FnMut(&mut T),
        T: UIElement,
    {
        if let Some(element) = self.elements.get_mut(name) {
            if let Some(ref mut c) = element.downcast_mut::<T>() {
                op(c);
            } else {
                panic!(
                    "The requested element with name \"{}\" could not be downcast to type {:?}",
                    name,
                    std::any::TypeId::of::<T>()
                )
            }
        } else {
            panic!("No UI element found with name \"{}\"", name);
        }
    }
}

pub trait UIElement: mopa::Any {
    fn draw(&mut self, ctx: &mut Context);
    fn hover(&mut self, mouse_x: i32, mouse_y: i32) {}
    fn click(&mut self, mouse_x: i32, mouse_y: i32, info: &mut StateInfo) {}
}

mopafy!(UIElement);

//defaults
const DEFAULT_FONT_FILE: &'static [u8; 95_068] =
    include_bytes!("../embedded/NunitoSans-SemiBold.ttf");
const DEFAULT_FONT_SIZE: u32 = 16;

lazy_static! {
    static ref DEFAULT_FONT: Font = Font::from_bytes("NunitoSans-SemiBold",
                                                     DEFAULT_FONT_FILE,
                                                     DEFAULT_FONT_SIZE,
                                                     (96.0, 96.0)).unwrap();
    static ref DEFAULT_FONT_COLOR_LIGHT: Color = Color::from((178, 164, 141));
    static ref DEFAULT_FONT_COLOR_DARK: Color = Color::from((1, 4, 8));
}

//label
pub struct Label {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text_contents: String,
    font: &'static Font,
    render_text: Option<Text>,
    font_color: Color,
}

impl Label {
    pub fn new(text: String) -> Label {
        Label {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text_contents: text,
            font: &(*DEFAULT_FONT),
            render_text: None,
            font_color: *DEFAULT_FONT_COLOR_LIGHT,
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_text(&mut self, text: String) {
        self.text_contents = text;
        self.render_text = None;
    }
}

impl UIElement for Label {
    fn draw(&mut self, ctx: &mut Context) {
        if self.render_text.is_none() {
            let txt = Text::new(ctx, &self.text_contents, self.font).unwrap();
            self.width = txt.width() as f32;
            self.height = txt.height() as f32;
            self.render_text = Some(txt);

            if let Some(ref mut txt) = self.render_text {
                txt.set_filter(graphics::FilterMode::Nearest);
            }
        }

        if let Some(ref txt) = self.render_text {
            let text_pos = Point::new(self.x, self.y);
            graphics::set_color(ctx, self.font_color).unwrap();
            graphics::draw(ctx, txt, text_pos, 0.0).unwrap();
        }
    }
}

//button
const BUTTON_DEFAULT_HORIZ_PADDING: f32 = 4.0;
const BUTTON_DEFAULT_VERT_PADDING: f32 = 2.0;
const BUTTON_DEFAULT_BORDER_WIDTH: f32 = 2.0;

lazy_static! {
    static ref BUTTON_DEFAULT_COLOR: Color = Color::from((29, 50, 76));
    static ref BUTTON_DEFAULT_COLOR_HOVER: Color = Color::from((47, 66, 91));
    static ref BUTTON_DEFAULT_COLOR_BORDER: Color = Color::from((1, 4, 8));
}

pub struct Button {
    color: Color,
    color_hover: Color,
    color_border: Color,
    x: f32,
    y: f32,
    text_contents: String,
    font: &'static Font,
    callback: fn(info: &mut StateInfo),
    hovered: bool,
    render_text: Option<Text>,
    font_color: Color,
    width: f32,
    height: f32,
    horiz_padding: f32,
    vert_padding: f32,
    use_default_width: bool,
}

impl Button {
    pub fn new(text: String, callback: fn(info: &mut StateInfo)) -> Button {
        Button {
            color: *BUTTON_DEFAULT_COLOR,
            color_hover: *BUTTON_DEFAULT_COLOR_HOVER,
            color_border: *BUTTON_DEFAULT_COLOR_BORDER,
            x: 0.0,
            y: 0.0,
            text_contents: text,
            font: &(*DEFAULT_FONT),
            callback: callback,
            hovered: false,
            render_text: None,
            font_color: *DEFAULT_FONT_COLOR_DARK,
            width: 0.0,
            height: 0.0,
            horiz_padding: BUTTON_DEFAULT_HORIZ_PADDING,
            vert_padding: BUTTON_DEFAULT_VERT_PADDING,
            use_default_width: true,
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.use_default_width = false;
        self.render_text = None;
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_text(&mut self, text: String) {
        self.text_contents = text;
        self.render_text = None;
    }
}

impl UIElement for Button {
    fn draw(&mut self, ctx: &mut Context) {
        if self.render_text.is_none() {
            self.render_text = Some(Text::new(ctx, &self.text_contents, self.font).unwrap());
            if let Some(ref mut txt) = self.render_text {
                txt.set_filter(graphics::FilterMode::Nearest);
                if self.use_default_width {
                    self.width = txt.width() as f32 + (2.0 * self.horiz_padding);
                }
                self.height = txt.height() as f32 + (2.0 * self.vert_padding);
            }
        }

        let rect = Rect::new(self.x, self.y, self.width, self.height);

        if self.hovered {
            graphics::set_color(ctx, self.color_hover).unwrap();
        } else {
            graphics::set_color(ctx, self.color).unwrap();
        }
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rect).unwrap();

        graphics::set_line_width(ctx, BUTTON_DEFAULT_BORDER_WIDTH);
        graphics::set_color(ctx, self.color_border).unwrap();
        graphics::rectangle(ctx, graphics::DrawMode::Line, rect).unwrap();

        let text_pos = Point::new(self.x, self.y);

        if let Some(ref txt) = self.render_text {
            graphics::set_color(ctx, self.font_color).unwrap();
            graphics::draw(ctx, txt, text_pos, 0.0).unwrap();
        }
    }

    fn hover(&mut self, mouse_x: i32, mouse_y: i32) {
        let mx = mouse_x as f32;
        let my = mouse_y as f32;
        self.hovered = mx >= self.x - (self.width / 2.0) && mx <= self.x + (self.width / 2.0) &&
            my >= self.y - (self.height / 2.0) &&
            my <= self.y + (self.height / 2.0);
    }

    fn click(&mut self, mouse_x: i32, mouse_y: i32, info: &mut StateInfo) {
        if self.hovered {
            (self.callback)(info);
        }
    }
}
