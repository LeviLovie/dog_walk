pub mod map;
pub mod player;
pub mod utils;

use clap::Parser;
use raylib::prelude::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "800")]
    pub width: u32,
    #[arg(long, default_value = "600")]
    pub height: u32,
    #[arg(long, default_value = "90")]
    pub fov: u32,
    #[arg(long, default_value = "10")]
    pub ray_step: u32,
    #[arg(long, default_value = "10")]
    pub visibility: f32,

    #[arg(long, default_value = "30")]
    pub fps: u32,

    #[arg(short, long, default_value = "false")]
    pub debug: bool,
}

fn main() {
    let args = Args::parse();

    let (mut rl, thread) = raylib::init()
        .size(args.width as i32, args.height as i32)
        .title("Dog Walk")
        .resizable()
        .build();
    rl.set_target_fps(args.fps);

    let map = map::Map::new();

    let mut player = player::Player::new(2.5, 5.0, 0.0);

    let rays_amount = args.width / args.ray_step;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let frame_time = d.get_frame_time();
        if d.is_key_down(KeyboardKey::KEY_W) {
            player.move_forward(2.0 * frame_time);
        } else if d.is_key_down(KeyboardKey::KEY_S) {
            player.move_backward(2.0 * frame_time);
        }
        if d.is_key_down(KeyboardKey::KEY_A) {
            player.turn(-180.0 * frame_time);
        } else if d.is_key_down(KeyboardKey::KEY_D) {
            player.turn(180.0 * frame_time);
        }

        let player_pos = player.pos();

        let mut rays: Vec<utils::Ray> = Vec::new();
        let ray_step = args.fov as f32 / rays_amount as f32;
        for i in 0..rays_amount {
            let angle = player.view() - args.fov as f32 / 2.0 + ray_step * i as f32;
            let ray_dir = utils::Point::new(angle.to_radians().cos(), angle.to_radians().sin());
            let mut ray = utils::Ray::new(ray_dir, *player_pos);
            ray.calculate_intersections(map.walls());
            rays.push(ray);
        }

        // d.draw_rectangle(0, args.height as i32 / 2, args.width as i32, args.height as i32, Color::from_hex("674636").unwrap());

        // Draw vertical bars for the rays and scale each one's width to the args.ray_step
        for (i, ray) in rays.iter().enumerate() {
            for (collision, wall_type) in ray.collisions.clone() {
                let ray_len =
                    (collision.x - player_pos.x).powi(2) + (collision.y - player_pos.y).powi(2);
                let ray_len = f32::sqrt(ray_len);
                let ray_height = args.height as f32 / ray_len;
                match wall_type {
                    utils::WallType::Full => d.draw_rectangle(
                        (args.ray_step as f32 * i as f32) as i32,
                        (args.height as f32 / 2.0 - ray_height / 2.0) as i32,
                        args.ray_step as i32,
                        ray_height as i32,
                        scale_color(ray_len, Color::from_hex("AAAAAA").unwrap()),
                    ),
                    utils::WallType::Half => d.draw_rectangle(
                        (args.ray_step as f32 * i as f32) as i32,
                        (args.height as f32 / 2.0) as i32,
                        args.ray_step as i32,
                        (ray_height / 2.0) as i32,
                        scale_color(ray_len, Color::from_hex("AA3333").unwrap()),
                    ),
                }
            }
        }

        if args.debug {
            for wall in map.walls() {
                d.draw_rectangle_lines(
                    (wall.x * 10.0) as i32,
                    (wall.y * 10.0) as i32,
                    (wall.width * 10.0) as i32,
                    (wall.height * 10.0) as i32,
                    Color::WHITE,
                );

                d.draw_circle_lines(
                    (player_pos.x * 10.0) as i32,
                    (player_pos.y * 10.0) as i32,
                    5.0,
                    Color::RED,
                );

                // Draw the rays
                for i in 0..rays_amount {
                    let ray_dir = rays[i as usize].dir;
                    let distance = rays[i as usize].len();
                    d.draw_line(
                        (player_pos.x * 10.0) as i32,
                        (player_pos.y * 10.0) as i32,
                        ((player_pos.x + ray_dir.x * distance) * 10.0) as i32,
                        ((player_pos.y + ray_dir.y * distance) * 10.0) as i32,
                        Color::GREEN,
                    );
                }
            }

            d.draw_fps(args.width as i32 - 80, 10);
        }
    }
}

fn scale_color(ray_len: f32, color: Color) -> Color {
    let color_scale = 1.0 / ray_len * 0.75;
    return Color::new(
        (color.r as f32 * color_scale) as u8,
        (color.g as f32 * color_scale) as u8,
        (color.b as f32 * color_scale) as u8,
        255,
    );
}
