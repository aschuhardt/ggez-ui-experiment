mod map;

use std::time::Duration;

use rand;
use rand::Rng;
use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics;

use substate::states::{StateInfo, StoredValue};
use substate::{Status, SubState};
use utility::ui;
use self::map::Map;

const SEED_ELEMENT_PADDING_TOP: f32 = 8.0;
const SEED_ELEMENT_PADDING_HORIZ: f32 = 10.0;

pub struct MapGenState {
    info: StateInfo,
    has_initialized_ui: bool,
    ui_context: ui::UIContext,
    map: Map,
}

impl MapGenState {
    pub fn new() -> MapGenState {
        MapGenState {
            info: StateInfo::new(),
            has_initialized_ui: false,
            ui_context: ui::UIContext::new(),
            map: Map::new(16, 16, 32),
        }
    }

    fn init_ui(&mut self, ctx: &mut Context) {
        graphics::set_background_color(ctx, graphics::BLACK);

        self.ui_context.add_element(
            "btn_newSeed",
            Box::new(ui::Button::new(
                String::from("Generate New"),
                MapGenState::set_random_seed,
            )),
        );

        self.ui_context.add_element(
            "lbl_mapSeed",
            Box::new(ui::Label::new(String::from("..."))),
        );

        self.has_initialized_ui = true;
        MapGenState::set_random_seed(&mut self.info);
    }

    fn set_random_seed(state: &mut StateInfo) {
        state.set_value(
            "map_seed",
            StoredValue::Integral { value: rand::thread_rng().gen::<i32>() },
        );
        state.set_value("gen_map", StoredValue::Boolean { value: true });
        state.refresh_ui();
    }

    fn should_regenerate_map(&mut self) -> bool {
        if let Ok(&StoredValue::Boolean { value: flag }) = self.info.get_value("gen_map") {
            return flag.clone();
        } else {
            return false;
        }
    }
}

impl SubState for MapGenState {
    fn get_status(&mut self) -> Option<Status> {
        self.info.get_status()
    }
}

impl event::EventHandler for MapGenState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {

        //check to see if ui needs to be updated after having been initialized
        if self.info.is_ui_dirty() && self.has_initialized_ui {
            let screen = graphics::get_screen_coordinates(ctx);

            let mut seed_label_width: f32 = 0.0;
            let mut seed_text = String::new();
            let mut map_seed = 0i32;

            if let Ok(&StoredValue::Integral { value: seed }) = self.info.get_value("map_seed") {
                map_seed = seed.clone();
            }

            if self.should_regenerate_map() {
                self.map.set_seed(map_seed);
                self.map.generate_regions(|_| {}); //todo: setup callback
                self.info.set_value(
                    "gen_map",
                    StoredValue::Boolean { value: false },
                );
            }

            self.ui_context.modify_element(
                "lbl_mapSeed",
                |lbl: &mut ui::Label| {
                    lbl.set_text(format!("Seed: {}", map_seed), ctx);
                    seed_label_width = lbl.get_width();
                    let height = lbl.get_height();
                    lbl.set_position(
                        SEED_ELEMENT_PADDING_HORIZ + seed_label_width / 2.0,
                        SEED_ELEMENT_PADDING_TOP + height / 2.0,
                    );
                },
            );

            self.ui_context.modify_element(
                "btn_newSeed",
                |btn: &mut ui::Button| {
                    let width = btn.get_width();
                    let height = btn.get_height();
                    btn.set_position(
                        2.0 * SEED_ELEMENT_PADDING_HORIZ + seed_label_width + width / 2.0,
                        SEED_ELEMENT_PADDING_TOP + height / 2.0,
                    )
                },
            );
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
