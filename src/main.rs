use std::time::Instant;

use macroquad::prelude::*;

const BLOCKSIZE: i32 = 30;
const WTFRATIO: f32 = 1.25;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        fullscreen: false,
        window_width: (BLOCKSIZE as f32 * 10.0 * WTFRATIO) as i32,
        window_height: (BLOCKSIZE as f32 * 20.0 * WTFRATIO) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut blocks: Vec<Block> = vec![];
    let mut create_time = Instant::now();
    let mut move_time = Instant::now();
    println!("{} {}", screen_width(), screen_height());
    loop {
        clear_background(LIGHTGRAY);
        if create_time.elapsed().as_secs() > 2 {
            blocks.push(Block::new());
            println!("{:?}", blocks.last());
            create_time = Instant::now();
        }
        if move_time.elapsed().as_secs() > 0 {
            for block in &mut blocks {
                block.fall();
            }
            move_time = Instant::now();
        }
        for block in &blocks {
            block.draw();
        }

        next_frame().await
    }
}

#[derive(Debug)]
struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn draw(&self) {
        draw_rectangle(
            self.x as f32,
            self.y as f32,
            BLOCKSIZE as f32,
            BLOCKSIZE as f32,
            BLUE,
        );
    }

    fn new() -> Self {
        Block {
            x: BLOCKSIZE * rand::gen_range(0, 10),
            y: 0,
        }
    }

    fn fall(&mut self) {
        self.y += BLOCKSIZE;
    }
}
