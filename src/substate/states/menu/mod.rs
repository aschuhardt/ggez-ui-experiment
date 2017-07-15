use std::time::Duration;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;

use substate::states::StateInfo;
use substate::{Status, SubState};
use utility::ui;

const TITLE_TOP_PAD: f32 = 48.0;
const BTN_WIDTH: f32 = 120.0;
const BTN_SPACING: f32 = 6.0;
const PLAY_BTN_TOP_PAD: f32 = 200.0;

pub struct MenuState {
    info: StateInfo,
    has_initialized_ui: bool,
    ui_context: ui::UIContext,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {
            info: StateInfo::new(),
            has_initialized_ui: false,
            ui_context: ui::UIContext::new(),
        }
    }

    fn init_ui(&mut self, ctx: &mut Context) {
        graphics::set_background_color(ctx, graphics::BLACK);

        self.ui_context.add_element(
            "lbl_title",
            Box::new(ui::Label::new(String::from("Main Menu")))
        );

        self.ui_context.add_element(
            "btn_play",
            Box::new(
                ui::Button::new(
                    String::from("Play"),
                    |state: &mut StateInfo| {
                        state.transition("mapgen");
                    }
                )
            )
        );

        self.ui_context.add_element(
            "btn_about",
            Box::new(
                ui::Button::new(
                    String::from("About"),
                    |state: &mut StateInfo| {
                        state.transition("about");
                    }
                )
            )
        );

        self.ui_context.add_element(
            "btn_quit",
            Box::new(
                ui::Button::new(
                    String::from("Quit"),
                    |state: &mut StateInfo| {
                        state.quit();
                    }
                )
            )
        );

        self.has_initialized_ui = true;
    }
}

impl SubState for MenuState {
    fn get_status(&mut self) -> Option<Status> {
        self.info.get_status()
    }
}

impl event::EventHandler for MenuState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        //check to see if ui needs to be updated after having been initialized
        if self.info.is_ui_dirty() && self.has_initialized_ui {
            let screen = graphics::get_screen_coordinates(ctx);
            
            self.ui_context.modify_element("lbl_title", |lbl: &mut ui::Label| {
                let height = lbl.get_height();
                lbl.set_position(screen.w / 2.0, TITLE_TOP_PAD + height / 2.0);
            });

            self.ui_context.modify_element("btn_play", |btn: &mut ui::Button| {
                let height = btn.get_height();
                btn.set_position(screen.w / 2.0, PLAY_BTN_TOP_PAD);
                btn.set_width(BTN_WIDTH);
            });

            self.ui_context.modify_element("btn_about", |btn: &mut ui::Button| {
                let height = btn.get_height();
                btn.set_position(screen.w / 2.0, 
                                 height + BTN_SPACING + PLAY_BTN_TOP_PAD);
                btn.set_width(BTN_WIDTH);
            });

            self.ui_context.modify_element("btn_quit", |btn: &mut ui::Button| {
                let height = btn.get_height();
                btn.set_position(screen.w / 2.0, 
                                 2.0 * (height + BTN_SPACING) + PLAY_BTN_TOP_PAD);
                btn.set_width(BTN_WIDTH);
            });
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
