use ggez::{
    graphics::{self},
    Context, GameResult,
};
use glam::Vec2;

use crate::game::MODES;

use super::EngineBasic;

#[derive(Clone)]
pub struct Tile {
    pub index: u32,
    pub x: u32,
    pub y: u32,
    pub state_tile: MODES,
    pub color: graphics::Color,
    pub rect: graphics::Mesh,
    pub rect_outline: graphics::Mesh,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl Tile {
    pub fn new(
        index: u32,
        x: u32,
        y: u32,
        color: ggez::graphics::Color,
        _ctx: &mut Context,
    ) -> Tile {
        let rect = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
            color,
        )
        .unwrap();

        //outline rect
        let rect_outline = graphics::Mesh::new_rectangle(
            _ctx,
            graphics::DrawMode::stroke(2.0),
            graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
            ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
        Tile {
            index,
            x,
            y,
            color,
            rect,
            rect_outline,
            state_tile: MODES::DEFAULT
        }
    }
}

impl Map {
    pub fn new(width: u32, height: u32, _ctx: &mut Context) -> Map {
        let mut tiles = Vec::new();
        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut index = 0;
        for _ in 0..width {
            for _ in 0..height {
                tiles.push(Tile::new(
                    index,
                    pos_x,
                    pos_y,
                    ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
                    _ctx,
                ));
                index += 1;
                pos_x += 32;
            }
            pos_y += 32;
            pos_x = 0;
        }
        Map {
            width: width,
            height: height,
            tiles: tiles,
        }
    }
}

impl EngineBasic for Tile {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::draw(_ctx, &self.rect, (Vec2::new(self.x as f32, self.y as f32),))?;
        graphics::draw(
            _ctx,
            &self.rect_outline,
            (Vec2::new(self.x as f32, self.y as f32),),
        )?;
        Ok(())
    }
}

impl EngineBasic for Map {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.tiles
            .iter_mut()
            .for_each(|tile| tile.update(_ctx).unwrap());
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for tile in self.tiles.iter_mut() {
            tile.draw(_ctx)?;
        }
        Ok(())
    }
}
