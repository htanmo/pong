use raylib::{
    ffi::{CheckCollisionCircleRec, Rectangle, Vector2},
    prelude::*,
};

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
    radius: f32,
}

impl Ball {
    fn new(x: f32, y: f32, speed_x: f32, speed_y: f32, radius: f32) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y,
            radius,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x as i32, self.y as i32, self.radius, Color::WHITE);
    }
}

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Paddle {
    fn new(x: f32, y: f32, speed: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            x,
            y,
            speed,
            width,
            height,
            color,
        }
    }

    fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.x - self.width / 2.0,
            y: self.y - self.height / 2.0,
            width: 10.0,
            height: 100.0,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(self.get_rect(), self.color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Ping Pong")
        .vsync()
        .build();

    let screen_width = rl.get_screen_width();
    let screen_height = rl.get_screen_height();

    let mut ball = Ball::new(
        screen_width as f32 / 2.0,
        screen_height as f32 / 2.0,
        300.0,
        300.0,
        5.0,
    );

    let mut left_paddle = Paddle::new(
        50.0,
        screen_height as f32 / 2.0,
        500.0,
        10.0,
        100.0,
        Color::RED,
    );
    let center_paddle_left = left_paddle.height / 2.0;

    let mut right_paddle = Paddle::new(
        screen_width as f32 - 50.0,
        screen_height as f32 / 2.0,
        500.0,
        10.0,
        100.0,
        Color::BLUE,
    );
    let center_paddle_right = right_paddle.height / 2.0;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        ball.x += ball.speed_x * rl.get_frame_time();
        ball.y += ball.speed_y * rl.get_frame_time();

        if ball.y < 0.0 {
            ball.y = 0.0;
            ball.speed_y *= -1.0;
        }
        if ball.y > screen_height as f32 {
            ball.y = screen_height as f32;
            ball.speed_y *= -1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_W) {
            left_paddle.y -= left_paddle.speed * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            left_paddle.y += left_paddle.speed * rl.get_frame_time();
        }

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            right_paddle.y -= right_paddle.speed * rl.get_frame_time();
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            right_paddle.y += right_paddle.speed * rl.get_frame_time();
        }

        if left_paddle.y < center_paddle_left {
            left_paddle.y = center_paddle_left;
        }
        if left_paddle.y > screen_height as f32 - center_paddle_left {
            left_paddle.y = screen_height as f32 - center_paddle_left;
        }

        if right_paddle.y < center_paddle_right {
            right_paddle.y = center_paddle_right;
        }
        if right_paddle.y > screen_height as f32 - center_paddle_right {
            right_paddle.y = screen_height as f32 - center_paddle_right;
        }

        unsafe {
            if CheckCollisionCircleRec(
                Vector2 {
                    x: ball.x,
                    y: ball.y,
                },
                ball.radius,
                left_paddle.get_rect(),
            ) {
                if ball.speed_x < 0.0 {
                    ball.speed_x *= -1.1;
                    ball.speed_y =
                        (ball.y - left_paddle.y) / (left_paddle.height / 2.0) * ball.speed_x;
                }
            }

            if CheckCollisionCircleRec(
                Vector2 {
                    x: ball.x,
                    y: ball.y,
                },
                ball.radius,
                right_paddle.get_rect(),
            ) {
                if ball.speed_x > 0.0 {
                    ball.speed_x *= -1.1;
                    ball.speed_y =
                        (ball.y - right_paddle.y) / (right_paddle.height / 2.0) * ball.speed_x;
                }
            }
        }

        let mut text = "";
        if ball.x < 0.0 {
            text = "Right Player Wins!";
        }
        if ball.x > rl.get_screen_width() as f32 {
            text = "Left Player Wins!";
        }

        if !text.is_empty() && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            ball.x = screen_width as f32 / 2.0;
            ball.y = screen_height as f32 / 2.0;
            ball.speed_x = 300.0;
            ball.speed_y = 300.0;
            text = "";
        }

        let fps = rl.get_fps().to_string();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        ball.draw(&mut d);
        right_paddle.draw(&mut d);
        left_paddle.draw(&mut d);

        if !text.is_empty() {
            let text_width = measure_text(text, 60);
            d.draw_text(
                text,
                screen_width / 2 - text_width / 2,
                screen_height / 2 - 30,
                60,
                Color::YELLOW,
            );
        }

        d.draw_text(fps.as_str(), 10, 10, 10, Color::WHITE);
    }
}
