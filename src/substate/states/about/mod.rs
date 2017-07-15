use std::time::Duration;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;

use substate::states::StateInfo;
use substate::{Status, SubState};
use utility::ui;

pub struct AboutState {
    info: StateInfo,
    has_initialized_ui: bool,
    ui_context: ui::UIContext,
}

impl AboutState {
    pub fn new() -> AboutState {
        AboutState {
            info: StateInfo::new(),
            has_initialized_ui: false,
            ui_context: ui::UIContext::new(),
        }
    }

    fn init_ui(&mut self, ctx: &mut Context) {
        graphics::set_background_color(ctx, graphics::BLACK);

        self.ui_context.add_element(
            "lbl_about",
            Box::new(
                ui::Label::new(String::from(
                    "This game was made by Addison Schuhardt."
                ))
            )
        );

        self.ui_context.add_element(
            "lbl_contact",
            Box::new(
                ui::Label::new(String::from(
                    "Contact: a@schuhardt.net"
                ))
            )
        );

        self.ui_context.add_element(
            "btn_menu",
            Box::new(
                ui::Button::new(
                    String::from("Back to Main Menu"),
                    |state: &mut StateInfo| {
                        state.transition("menu");
                    }
                )
            )
        );

        self.has_initialized_ui = true;
    }
}

impl SubState for AboutState {
    fn get_status(&mut self) -> Option<Status> {
        self.info.get_status()
    }
}

impl event::EventHandler for AboutState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        //check to see if ui needs to be updated after having been initialized
        if self.info.is_ui_dirty() && self.has_initialized_ui {            
            let screen = graphics::get_screen_coordinates(ctx);

            self.ui_context.modify_element("lbl_about", |lbl: &mut ui::Label| {
                lbl.set_position(screen.w / 2.0, 100.0);
            });

            self.ui_context.modify_element("lbl_contact", |lbl: &mut ui::Label| {
                lbl.set_position(screen.w / 2.0, 150.0);
            });

            self.ui_context.modify_element("btn_menu", |btn: &mut ui::Button| {
                let height = btn.get_height();
                let width = btn.get_width();
                btn.set_position((width / 2.0) + 2.0, ((screen.h * -1.0) - height / 2.0) - 2.0);
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
