extern crate ggez;

pub mod components;
pub mod game;
pub mod path_find_algorithms;

use game::Game;
use ggez::{event, ContextBuilder};
use path_find_algorithms::a_start::a_start_num;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 { 
        let maze = [
                    [0, 0, 0, 1, 0 ],
                    [0, 0, 0, 1, 0 ],
                    [0, 0, 0, 0, 0 ],
                    [0, 0, 0, 1, 0 ],
                    [0, 0, 0, 1, 0 ]
        ];

        let _ = a_start_num(maze, 0, 0, 4, 2);

    } else {
        let mut window_mode = ggez::conf::WindowMode::default();
        window_mode.width = 1024.0;
        window_mode.height = 768.0;

        let (mut ctx, event_loop) = ContextBuilder::new("path_finder", "Gabriel")
            .window_setup(ggez::conf::WindowSetup {
                title: "Path Finder".to_string(),
                vsync: true,
                ..Default::default()
            })
            .window_mode(ggez::conf::WindowMode {
                width: 1024.0,
                height: 768.0,
                ..Default::default()
            })
            .build()
            .expect("aieee, could not create ggez context!");

        let game = Game::new(&mut ctx);

        event::run(ctx, event_loop, game);
    }
}
