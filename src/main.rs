#![feature(int_roundings)]

use raylib::{prelude::RaylibDraw, color::Color};

mod paddle;
mod ball;



fn main() {
    let w = 800;
    let h = 800;





    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Hello, World")
        .build();
    

    let mut left_paddle = paddle::Paddle::new(20.0, (h/2) as f32, 100.0, 20.0, 400.0, h);
    let mut ball = ball::Ball::new(w as f32/2.0, h as f32/2.0, 400.0, 20.0, 20.0, w as f32, h as f32);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.draw_text(&format!("{}", ball.get_score()), w/2, h/10, 30, Color::WHITE);

        left_paddle.tick(d.get_frame_time(), &d);
        d.draw_rectangle_rec(left_paddle.rect(), Color::WHITE);
        ball.tick(d.get_frame_time(), left_paddle);
        d.draw_rectangle_rec(ball.rect(), Color::WHITE);


        println!("frame time: {}", d.get_frame_time());
        println!("width:      {}", d.get_screen_width());
        println!("height:     {}", d.get_screen_height());
        print!("{esc}c", esc = 27 as char);

        d.clear_background(Color::BLACK);
    }
}
