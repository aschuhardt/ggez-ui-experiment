use ggez;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Point, Font, Text};

const DEBUG_FONT_FILE: &'static [u8; 79_584] = include_bytes!("../embedded/LessPerfectDOSVGA.ttf");
const DEBUG_FONT_SIZE: u32 = 16;

lazy_static! {
    static ref DEBUG_FONT: Font = Font::from_bytes("LessPerfectDOSVGA",
                                                    DEBUG_FONT_FILE,
                                                    DEBUG_FONT_SIZE,
                                                    (72.0, 72.0)).unwrap();
}

pub fn draw_debug_information(ctx: &mut Context) {
    //draw current FPS
    let fps_str = format!("{:.1}", ggez::timer::get_fps(ctx));
    let fps_text = Text::new(ctx, fps_str.as_str(), &(*DEBUG_FONT)).unwrap();
    let fps_pos = Point::new(
        fps_text.width() as f32 / 2.0,
        fps_text.height() as f32 / 2.0,
    );
    graphics::set_color(ctx, graphics::WHITE).unwrap();
    graphics::draw(ctx, &fps_text, fps_pos, 0.0).unwrap();

    let screen_rect = graphics::get_screen_coordinates(ctx);

    graphics::set_line_width(ctx, 1.0);

    graphics::set_color(ctx, graphics::Color::from((0, 20, 0))).unwrap();
    graphics::line(
        ctx,
        &[
            Point::new(screen_rect.w / 2.0, 0.0),
            Point::new(screen_rect.w / 2.0, screen_rect.h * -1.0),
        ],
    ).unwrap();

    graphics::set_color(ctx, graphics::Color::from((20, 0, 0))).unwrap();
    graphics::line(
        ctx,
        &[
            Point::new(0.0, screen_rect.h * -1.0 / 2.0),
            Point::new(screen_rect.w, screen_rect.h * -1.0 / 2.0),
        ],
    ).unwrap();
}

pub fn draw_mouse_position(x: i32, y: i32, ctx: &mut Context) {
    let mouse_pos_str = format!("X: {}, Y: {}", x, y);
    let mouse_pos_text = Text::new(ctx, mouse_pos_str.as_str(), &(*DEBUG_FONT)).unwrap();
    let mouse_pos_pos = Point::new(
        mouse_pos_text.width() as f32 / 2.0,
        24.0 + mouse_pos_text.height() as f32 / 2.0,
    );
    graphics::set_color(ctx, graphics::WHITE).unwrap();
    graphics::draw(ctx, &mouse_pos_text, mouse_pos_pos, 0.0).unwrap();

    let screen_rect = graphics::get_screen_coordinates(ctx);

    graphics::set_line_width(ctx, 1.0);

    graphics::set_color(ctx, graphics::Color::from((0, 20, 0))).unwrap();
    graphics::line(
        ctx,
        &[
            Point::new(x as f32, 0.0),
            Point::new(x as f32, screen_rect.h * -1.0),
        ],
    ).unwrap();

    graphics::set_color(ctx, graphics::Color::from((20, 0, 0))).unwrap();
    graphics::line(
        ctx,
        &[
            Point::new(0.0, y as f32),
            Point::new(screen_rect.w, y as f32),
        ],
    ).unwrap();
}
