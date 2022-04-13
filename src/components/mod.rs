use ggez::{graphics, Context, GameResult};

use crate::game::{ImportantItensPos, MODES};

use self::map::Tile;

pub mod collision;
pub mod map;

pub trait EngineBasic {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
}

fn change_tile_color(mode: &MODES) -> graphics::Color {
    match mode {
        MODES::START => graphics::Color::YELLOW,
        MODES::END => graphics::Color::GREEN,
        MODES::WALL => graphics::Color::BLACK,
        MODES::DEFAULT => graphics::Color::WHITE,
    }
}

fn recreate_rect(color: graphics::Color, _ctx: &mut Context) -> graphics::Mesh {
    let rect = graphics::Mesh::new_rectangle(
        _ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, 32.0, 32.0),
        color,
    )
    .unwrap();
    rect
}

pub fn change_color_and_check_indexes(
    items: &mut Vec<Tile>,
    item: &mut Tile,
    importante_itens_pos: &mut ImportantItensPos,
    mode: &MODES,
    _ctx: &mut Context,
) {
    fn change_item_state(val: &mut i32, item: &mut Tile, mode: MODES) {
        *val = item.index as i32;
        item.color = change_tile_color(&mode);
        item.state_tile = mode.clone();
    }

    match mode {
        MODES::START => {
            if importante_itens_pos.index_start == -1 {
                change_item_state(&mut importante_itens_pos.index_start, item, mode.clone());
            } else {
                items[importante_itens_pos.index_start as usize].color =
                    change_tile_color(&MODES::DEFAULT);
                items[importante_itens_pos.index_start as usize].state_tile = MODES::DEFAULT;
                items[importante_itens_pos.index_start as usize].rect =
                    recreate_rect(change_tile_color(&MODES::DEFAULT), _ctx);

                change_item_state(&mut importante_itens_pos.index_start, item, mode.clone());
            }
        }
        MODES::END => {
            if importante_itens_pos.index_end == -1 {
                change_item_state(&mut importante_itens_pos.index_end, item, mode.clone());
            } else {
                items[importante_itens_pos.index_end as usize].color =
                    change_tile_color(&MODES::DEFAULT);
                items[importante_itens_pos.index_end as usize].state_tile = MODES::DEFAULT;
                items[importante_itens_pos.index_end as usize].rect =
                    recreate_rect(change_tile_color(&MODES::DEFAULT), _ctx);

                change_item_state(&mut importante_itens_pos.index_end, item, mode.clone());
            }
        }
        MODES::WALL => {
            item.color = change_tile_color(mode);
        }
        MODES::DEFAULT => {
            return;
        }
    }
}
