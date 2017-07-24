#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate mopa;
extern crate ggez;
extern crate rand;
extern crate uuid;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod substate;
mod utility;

use std::time::Duration;

use ggez::conf;
use ggez::event;
use ggez::event::{MouseButton, MouseState, Keycode, Mod, Button, Axis};
use ggez::{GameResult, Context};
use ggez::graphics;

use self::substate::mapper;
use self::substate::{Status, SubState};

struct MainState {
    current_substate: Box<SubState>,
    debug: bool,
    paused: bool,
    mouse_position: (i32, i32),
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            current_substate: mapper::from_id("menu").unwrap(),
            debug: cfg!(debug_assertions),
            paused: false,
            mouse_position: (0, 0),
        };
        Ok(s)
    }

    fn transition_state(&mut self, id: &'static str) {
        match mapper::from_id(id) {
            Ok(state) => self.current_substate = state,
            Err(why) => panic!("{}", why),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        // handle sub-state statuses and transitions
        if let Some(status) = self.current_substate.get_status() {
            match status {
                Status::Transition { id } => {
                    self.transition_state(id);
                },
                Status::Quit => {
                    ctx.quit().unwrap();
                }
            }
        }

        if !self.paused {
            self.current_substate.update(ctx, dt)
        } else {
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.paused {
            graphics::clear(ctx);

            self.current_substate.draw(ctx).unwrap();

            if self.debug {
                utility::debug::draw_debug_information(ctx);
                utility::debug::draw_mouse_position(self.mouse_position.0, self.mouse_position.1, ctx)
            }

            graphics::present(ctx);
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.current_substate.mouse_button_down_event(button, x, y);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: i32, y: i32) {
        self.current_substate.mouse_button_up_event(button, x, y);
    }

    fn mouse_motion_event(&mut self, state: MouseState, x: i32, y: i32, xrel: i32, yrel: i32) {
        if self.debug {
            self.mouse_position = (x, y);
        }

        self.current_substate.mouse_motion_event(
            state,
            x,
            y,
            xrel,
            yrel,
        );
    }

    fn mouse_wheel_event(&mut self, x: i32, y: i32) {
        self.current_substate.mouse_wheel_event(x, y);
    }

    fn key_down_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        if keycode == Keycode::F3 {
            self.debug = !self.debug;
        }

        self.current_substate.key_down_event(
            keycode,
            keymod,
            repeat,
        );
    }

    fn key_up_event(&mut self, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.current_substate.key_up_event(keycode, keymod, repeat);
    }

    fn controller_button_down_event(&mut self, button: Button, instance_id: i32) {
        self.current_substate.controller_button_down_event(
            button,
            instance_id,
        );
    }

    fn controller_button_up_event(&mut self, button: Button, instance_id: i32) {
        self.current_substate.controller_button_up_event(
            button,
            instance_id,
        );
    }

    fn controller_axis_event(&mut self, axis: Axis, value: i16, instance_id: i32) {
        self.current_substate.controller_axis_event(
            axis,
            value,
            instance_id,
        );
    }

    fn focus_event(&mut self, gained: bool) {
        self.paused = !gained;
        self.current_substate.focus_event(gained);
    }

    fn quit_event(&mut self) -> bool {
        self.current_substate.quit_event()
    }
}

pub fn main() {
    // create window configuration
    let c = conf::Conf {
        window_title: String::from("Roguelike"),
        window_icon: String::from(""),
        window_height: 768,
        window_width: 1024,
        vsync: false,
        resizable: false,
    };

    // load window context
    let ctx = &mut Context::load_from_conf("aschuhardt", "rl", c).unwrap();

    // instantiate main state
    let state = &mut MainState::new().unwrap();

    // start game loop
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
