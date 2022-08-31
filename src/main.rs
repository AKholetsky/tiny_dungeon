use macroquad::{texture::{load_texture, draw_texture}, 
    window::{clear_background, next_frame}, 
    color::colors};

#[macroquad::main("tiny_dungeon")]
async fn main() {
    let texture = load_texture("assets/Tilemap/tilemap.png").await.unwrap();

    loop {
        clear_background(colors::LIGHTGRAY);
        draw_texture(texture, 10., 10., colors::WHITE);
        next_frame().await;
    }
}
