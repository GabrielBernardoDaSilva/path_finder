use ggez::{event::EventHandler, graphics, Context, GameResult};

use crate::components::{
    change_color_and_check_indexes, collision::mouse_collision_tile, map::Map, EngineBasic,
};

#[derive(PartialEq, Clone)]
pub enum MODES {
    START,
    END,
    WALL,
    DEFAULT,
}

pub struct ImportantItensPos {
    pub index_start: i32,
    pub index_end: i32,
}

pub struct Game {
    map: Map,
    mode: MODES,
    importante_itens_pos: ImportantItensPos,
    is_pressed: bool,
}

impl Game {
    pub fn new(_contex: &mut Context) -> Game {
        Game {
            map: Map::new(32, 32, _contex),
            mode: MODES::DEFAULT,
            importante_itens_pos: ImportantItensPos {
                index_start: -1,
                index_end: -1,
            },
            is_pressed: false,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.map.update(_ctx)?;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, graphics::Color::WHITE);
        self.map.draw(_ctx)?;
        graphics::present(_ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == ggez::event::KeyCode::Escape {
            println!("Escape key pressed");
            ggez::event::quit(ctx);
        }

        if keycode == ggez::event::KeyCode::A {
            println!("Start Place Mode");
            self.mode = MODES::START;
        }

        if keycode == ggez::event::KeyCode::D {
            println!("End Place Mode");
            self.mode = MODES::END;
        }

        if keycode == ggez::event::KeyCode::S {
            println!("Wall Mode");
            self.mode = MODES::WALL;
        }

        if keycode == ggez::event::KeyCode::Space {
            println!("Default Mode");
            self.mode = MODES::DEFAULT;
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.is_pressed = false;
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.is_pressed = true;
        if self.is_pressed {
            if let Some(mut item) = mouse_collision_tile(_x as u32, _y as u32, &self.map) {
                change_color_and_check_indexes(
                    &mut self.map.tiles,
                    &mut item,
                    &mut self.importante_itens_pos,
                    &self.mode,
                    _ctx,
                );

                let rect = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
                    item.color,
                )
                .unwrap();
                self.map.tiles[item.index as usize].rect = rect;
            }
        }
    }
}
