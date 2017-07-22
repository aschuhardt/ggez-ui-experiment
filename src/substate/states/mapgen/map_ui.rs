use ggez::Context;

use utility::ui;
use substate::states::StateInfo;

pub struct MapUI {
    
}

impl MapUI {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> MapUI {
        MapUI {

        }
    }
}

impl ui::UIElement for MapUI {    
    fn draw(&mut self, ctx: &mut Context) {

    }
    fn hover(&mut self, mouse_x: i32, mouse_y: i32) {

    }
    fn click(&mut self, mouse_x: i32, mouse_y: i32, info: &mut StateInfo) {

    }
}