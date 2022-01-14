use std::time::Instant;

use macroquad::prelude::*;

const BLOCK_SIZE: f32 = 30.0;
const SCREEN_WIDTH: f32 = BLOCK_SIZE * 10.0;
const SCREEN_HEIGHT: f32 = BLOCK_SIZE * 20.0;
const WTF_RATIO: f32 = 1.25;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        fullscreen: false,
        window_width: (SCREEN_WIDTH * WTF_RATIO) as i32,
        window_height: (SCREEN_HEIGHT * WTF_RATIO) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut moving_blocks: Vec<Block> = vec![];
    let mut move_time = Instant::now();
    let mut laying_blocks: Vec<Block> = vec![];
    let mut rotatable = true;
    println!("{} {}", screen_width(), screen_height());
    loop {
        clear_background(LIGHTGRAY);
        if move_time.elapsed().as_millis() > 100 {
            move_time = Instant::now();
            for block in &mut moving_blocks {
                block.fall();
            }
            if moving_blocks.iter().any(|mb| mb.if_stopped(&laying_blocks)) {
                laying_blocks.append(&mut moving_blocks);
            }
        }
        if move_time.elapsed().as_millis() > 20 {
            for y in 0..20 {
                if laying_blocks
                    .iter()
                    .filter(|n| n.0.y == y as f32 * BLOCK_SIZE as f32)
                    .count()
                    >= 10
                {
                    laying_blocks.retain(|b| b.0.y != y as f32 * BLOCK_SIZE as f32);
                    laying_blocks
                        .iter_mut()
                        .filter(|b| b.0.y < y as f32 * BLOCK_SIZE as f32)
                        .for_each(|b| b.fall());
                }
            }
        }
        if is_key_pressed(KeyCode::Up) && !moving_blocks.is_empty() && rotatable {
            rotate(&mut moving_blocks);
        }
        if is_key_pressed(KeyCode::Right) {
            for block in &mut moving_blocks {
                block.0.x += BLOCK_SIZE;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            for block in &mut moving_blocks {
                block.0.x -= BLOCK_SIZE;
            }
        }
        if moving_blocks.is_empty() {
            let new_shape = create_shape();
            moving_blocks = new_shape.0;
            rotatable = new_shape.1;
        }
        for block in &moving_blocks {
            block.draw();
        }
        for block in &laying_blocks {
            block.draw();
        }

        next_frame().await
    }
}

#[derive(Debug, Clone)]
struct Block(Rect);

impl Block {
    fn draw(&self) {
        draw_rectangle(self.0.x, self.0.y, self.0.w, self.0.h, BLUE);
    }

    fn new(x: f32, y: f32) -> Self {
        Block(Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE))
    }

    fn fall(&mut self) {
        self.0.y += BLOCK_SIZE as f32;
    }

    fn if_stopped(&self, laying_blocks: &[Block]) -> bool {
        let mut next_move = self.clone();
        next_move.fall();

        next_move.0.y >= SCREEN_HEIGHT as f32 || laying_blocks.iter().any(|lb| next_move.0 == lb.0)
    }
}

fn rotate(moving_blocks: &mut [Block]) {
    let x_center = moving_blocks[0].0.x;
    let y_center = moving_blocks[0].0.y;
    for block in &mut moving_blocks[1..] {
        let x_diff = block.0.x - x_center;
        let y_diff = block.0.y - y_center;
        block.0.x = x_center - y_diff;
        block.0.y = y_center + x_diff;
    }
}

fn create_shape() -> (Vec<Block>, bool) {
    let shape: i32 = rand::gen_range(0, 6);
    let x_start = rand::gen_range(1_i32, 9) as f32;

    let blocks_tuple = match shape {
        0 => [(0, -1), (0, 0), (1, 0), (1, -1)],
        1 => [(0, 0), (1, 0), (-1, 0), (-1, -1)],
        2 => [(0, 0), (1, 0), (-1, 0), (1, -1)],
        3 => [(0, 0), (-1, 0), (1, 0), (2, 0)],
        4 => [(0, 0), (0, -1), (1, 0), (-1, 0)],
        5 => [(0, 0), (0, -1), (1, -1), (-1, 0)],
        6 => [(0, 0), (0, -1), (1, 0), (-1, -1)],
        _ => panic!(),
    };
    (
        blocks_tuple
            .into_iter()
            .map(|(x, y)| Block::new((x_start + x as f32) * BLOCK_SIZE, y as f32 * BLOCK_SIZE))
            .collect(),
        shape != 0,
    )
}
