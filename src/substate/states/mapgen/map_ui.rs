use ggez::Context;
use ggez::graphics::{self, Color, Rect, DrawMode};

use utility::ui;
use substate::states::StateInfo;
use substate::states::mapgen::Map;
use substate::states::mapgen::map::BiomeType;

const REGION_OUTLINE_WIDTH: f32 = 3.0;

pub struct MapUI {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    biome_data: Vec<Vec<BiomeType>>,
    mouse_pos: (f32, f32),
}

impl MapUI {
    pub fn new() -> MapUI {
        MapUI {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            biome_data: Vec::<Vec<BiomeType>>::new(),
            mouse_pos: (0.0, 0.0),
        }
    }

    pub fn update(&mut self, map: &Map) {
        self.biome_data.clear();
        for x in 0..map.get_width() {
            let mut column = Vec::<BiomeType>::new();
            for y in 0..map.get_height() {
                column.push(map.get_biome_at_offset(x, y));
            }
            self.biome_data.push(column);
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.width = w;
        self.height = h;
    }
}

impl ui::UIElement for MapUI {
    fn draw(&mut self, ctx: &mut Context) {
        if self.biome_data.is_empty() || self.biome_data[0].is_empty() {
            return;
        }

        let data_width = self.biome_data.len();
        let data_height = self.biome_data[0].len();

        let rect_width = self.width / data_width as f32;
        let rect_height = self.height / data_height as f32;

        for x in 0..data_width {
            for y in 0..data_height {
                let biome_color: Color = match self.biome_data[x][y] {
                    BiomeType::Arid => Color::from((128, 98, 69)),
                    BiomeType::Grassland => Color::from((64, 106, 57)),
                    BiomeType::Ocean => Color::from((50, 61, 86)),
                    BiomeType::Rocky => Color::from((26, 27, 31)),
                };

                let rect_x = self.x + (x as f32 * rect_width);
                let rect_y = self.y + (y as f32 * rect_height);

                graphics::set_color(ctx, biome_color);
                graphics::rectangle(
                    ctx,
                    DrawMode::Fill,
                    Rect {
                        x: rect_x,
                        y: rect_y,
                        w: rect_width,
                        h: rect_height,
                    },
                );

                if self.mouse_pos.0 > rect_x - (rect_width / 2.0) 
                    && self.mouse_pos.0 < rect_x + (rect_width / 2.0)
                    && self.mouse_pos.1 > rect_y - (rect_height / 2.0) 
                    && self.mouse_pos.1 < rect_y + (rect_height / 2.0) {
                    graphics::set_color(ctx, graphics::BLACK);
                    graphics::set_line_width(ctx, REGION_OUTLINE_WIDTH);
                    graphics::rectangle(
                        ctx,
                        DrawMode::Line,
                        Rect {
                            x: rect_x,
                            y: rect_y,
                            w: rect_width,
                            h: rect_height,
                        },
                    );
                }
            }
        }
    }

    fn hover(&mut self, mouse_x: i32, mouse_y: i32) {
        self.mouse_pos = (mouse_x as f32, mouse_y as f32);
    }

    fn click(&mut self, mouse_x: i32, mouse_y: i32, info: &mut StateInfo) {

    }
}
