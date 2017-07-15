// Use this as a template for implementing sub-states

use std::time::Duration;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;

use substate::states::{StateInfo, StoredValue};
use substate::{Status, SubState};
use utility::ui;

pub struct TemplateState {
    info: StateInfo,
    has_initialized_ui: bool,
    ui_context: ui::UIContext,
}

impl TemplateState {
    pub fn new() -> TemplateState {
        TemplateState {
            info: StateInfo::new(),
            has_initialized_ui: false,
            ui_context: ui::UIContext::new(),
        }
    }

    fn init_ui(&mut self, ctx: &mut Context) {
        graphics::set_background_color(ctx, graphics::BLACK);

        // add ui elements here

        self.has_initialized_ui = true;
    }
}

impl SubState for TemplateState {
    fn get_status(&mut self) -> Option<Status> {
        self.info.get_status()
    }
}

impl event::EventHandler for TemplateState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        //check to see if ui needs to be updated after having been initialized
        if self.info.is_ui_dirty() && self.has_initialized_ui {
            
            // use self.ui_context.modify_element to update control information here

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.has_initialized_ui {
            self.init_ui(ctx);
            self.info.refresh_ui();
        }

        self.ui_context.draw(ctx);

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        state: event::MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        if !state.left() {
            self.ui_context.mouse_moved(x, y);
        }
    }

    fn mouse_button_down_event(&mut self, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left {
            self.ui_context.click(x, y, &mut self.info);
        }
    }
}
