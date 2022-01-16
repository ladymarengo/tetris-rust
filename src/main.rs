use std::{mem, time::Instant, process::exit};

use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    prelude::*,
    ui::{root_ui, Skin, widgets}, hash,
};

const BLOCK_SIZE: f32 = 30.0;
const SCREEN_WIDTH: f32 = BLOCK_SIZE * 10.0;
const SCREEN_HEIGHT: f32 = BLOCK_SIZE * 20.0;
const WTF_RATIO: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        fullscreen: false,
        high_dpi: false,
        window_width: (SCREEN_WIDTH * WTF_RATIO) as i32,
        window_height: (SCREEN_HEIGHT * WTF_RATIO) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let sound = load_sound_from_bytes(include_bytes!("../music.ogg"))
        .await
        .unwrap();
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 0.2,
        },
    );
    

    loop {
        menu().await;
        game().await;
    }
}

async fn menu() {
    let skin = {
        let button_style = root_ui()
            .style_builder()
            .text_color(BLACK)
            .color(LIGHTGRAY)
            .font_size(30)
            .build();
        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };
    
    loop {
        root_ui().push_skin(&skin);
        clear_background(WHITE);
        if widgets::Button::new("Start")
            // .size(vec2(200., 300.))
            .position(vec2(
                150.0,
                200.0,
            ))
            .ui(&mut *root_ui())
        {
            return;
        }
        next_frame().await;
    }
}

async fn game() {
    let mut moving_blocks: Vec<Block> = vec![];
    let mut move_time = Instant::now();
    let mut laying_blocks: Vec<Block> = vec![];
    let mut rotatable = true;
    let mut points: u32 = 0;
    let mut speed = 400;
    println!("{} {}", screen_width(), screen_height());
    loop {
        clear_background(WHITE);
        if move_time.elapsed().as_millis() > speed {
            move_time = Instant::now();
            if moving_blocks.iter().any(|mb| mb.if_stopped(&laying_blocks)) {
                laying_blocks.append(&mut moving_blocks);
            }
            for block in &mut moving_blocks {
                block.fall();
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
                    points += 10;
                    laying_blocks.retain(|b| b.0.y != y as f32 * BLOCK_SIZE as f32);
                    laying_blocks
                        .iter_mut()
                        .filter(|b| b.0.y < y as f32 * BLOCK_SIZE as f32)
                        .for_each(|b| b.fall());
                }
            }
        }
        if is_key_pressed(KeyCode::Up) && !moving_blocks.is_empty() && rotatable {
            try_action(rotate, &laying_blocks, &mut moving_blocks);
        }
        if is_key_pressed(KeyCode::Right) {
            let move_right = |mb: &mut [Block]| mb.iter_mut().for_each(|b| b.0.x += BLOCK_SIZE);
            try_action(move_right, &laying_blocks, &mut moving_blocks);
        }
        if is_key_pressed(KeyCode::Left) {
            let move_left = |mb: &mut [Block]| mb.iter_mut().for_each(|b| b.0.x -= BLOCK_SIZE);
            try_action(move_left, &laying_blocks, &mut moving_blocks);
        }
        if is_key_pressed(KeyCode::Down) {
            let move_down = |mb: &mut [Block]| mb.iter_mut().for_each(|b| b.0.y += BLOCK_SIZE);
            try_action(move_down, &laying_blocks, &mut moving_blocks);
        }
        if moving_blocks.is_empty() {
            if speed > 100 {
                speed -= 5;
            }
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

        if laying_blocks.iter().any(|lb| lb.0.y <= 0.0) {
            return;
        }
        draw_text(&format!("Points: {points}"), 20.0, 30.0, 20.0, GRAY);

        next_frame().await
    }
}

#[derive(Debug, Clone)]
struct Block(Rect, Color);

impl Block {
    fn draw(&self) {
        draw_rectangle(self.0.x, self.0.y, self.0.w, self.0.h, self.1);
    }

    fn new(x: f32, y: f32, color: Color) -> Self {
        Block(Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE), color)
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

    let (blocks_tuple, color) = match shape {
        0 => ([(0, -1), (0, 0), (1, 0), (1, -1)], MAGENTA),
        1 => ([(0, 0), (1, 0), (-1, 0), (-1, -1)], BLUE),
        2 => ([(0, 0), (1, 0), (-1, 0), (1, -1)], PINK),
        3 => ([(0, 0), (-1, 0), (1, 0), (2, 0)], ORANGE),
        4 => ([(0, 0), (0, -1), (1, 0), (-1, 0)], GOLD),
        5 => ([(0, 0), (0, -1), (1, -1), (-1, 0)], LIME),
        6 => ([(0, 0), (0, -1), (1, 0), (-1, -1)], VIOLET),
        _ => panic!(),
    };
    (
        blocks_tuple
            .into_iter()
            .map(|(x, y)| {
                Block::new(
                    (x_start + x as f32) * BLOCK_SIZE,
                    y as f32 * BLOCK_SIZE,
                    color,
                )
            })
            .collect(),
        shape != 0,
    )
}

fn try_action<F>(func: F, laying_blocks: &[Block], moving_blocks: &mut Vec<Block>)
where
    F: Fn(&mut [Block]),
{
    let mut new_blocks = moving_blocks.to_owned();
    func(&mut new_blocks);

    if !new_blocks.iter().any(|nb| {
        laying_blocks.iter().any(|lb| nb.0 == lb.0)
            || nb.0.x < 0.0
            || nb.0.x >= SCREEN_WIDTH as f32
            || nb.0.y >= SCREEN_HEIGHT as f32
    }) {
        mem::swap(moving_blocks, &mut new_blocks);
    }
}
