use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Blank 2D Template".to_owned(),
        window_width: 960,
        window_height: 540,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        // Your game code goes here.

        next_frame().await;
    }
}