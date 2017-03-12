extern crate sdl2;

use std::cmp;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::rect::Point;

fn main() {
    let width: i32 = 320;
    let height: i32 = 240;
    let window_width: i32 = 1024;
    let window_height: i32 = 768;

    let world_map = vec![
        vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
        vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let tex_size: i32 = 64;
    let textures: Vec<Vec<u32>> = {
        let mut tx: Vec<Vec<u32>> = Vec::with_capacity(8);
        for _ in 0..8 {
            let size = (tex_size * tex_size) as usize;
            tx.push(vec![0; size]);
        };
        for x in 0..tex_size {
            for y in 0..tex_size {
                let pixel_i = (tex_size * y + x) as usize;
                let xorcolor = ((x * 256 / tex_size) ^ (y * 256 / tex_size)) as u32;
                let ycolor = (y * 256 / tex_size) as u32;
                let xycolor = (y * 128 / tex_size + x * 128 / tex_size) as u32;
                let first_mod = if x != y && x != tex_size - y { 1 } else { 0 };
                tx[0][pixel_i] = 65536 * 254 * first_mod; //flat red texture with black cross
                tx[1][pixel_i] = xycolor + 256 * xycolor + 65536 * xycolor; //sloped greyscale
                tx[2][pixel_i] = 256 * xycolor + 65536 * xycolor; //sloped yellow gradient
                tx[3][pixel_i] = xorcolor + 256 * xorcolor + 65536 * xorcolor; //xor greyscale
                tx[4][pixel_i] = 256 * xorcolor; //xor green
                tx[5][pixel_i] = 65536 * 192 * if x % 16 == 0 && y % 16 == 0 { 0 } else { 1 }; //red bricks
                tx[6][pixel_i] = 65536 * ycolor; //red gradient
                tx[7][pixel_i] = 128 + 256 * 128 + 65536 * 128; //flat grey texture
            }
        };
        tx
    };

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("BD 3D", window_width as u32, window_height as u32)
        .position_centered()
        .resizable()
        .build().unwrap();

    let mut renderer = window
        .renderer()
        .accelerated()
        .present_vsync()
        .build().unwrap();

    renderer.set_logical_size(width as u32, height as u32).unwrap();

    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pos_x: f32 = 22.;     // position
    let mut pos_y: f32 = 12.;
    let mut dir_x: f32 = -1.;     // direction vector
    let mut dir_y: f32 = 0.;
    let mut plane_x: f32 = 0.;    // 2D raycaster version of the camera plane
    let mut plane_y: f32 = 0.66;

    let mut time: u32 = 0;   // time of the current frame
    let mut running = true;

    while running {
        // Draw ceiling and floor
        for y in 0i32..(height as i32) / 2 {
            // Ceiling
            let ceil_color = cmp::max(cmp::min(150 - y, 255), 0);
            renderer.set_draw_color(Color::RGB(0, 0, ceil_color as u8));
            renderer.fill_rect(Rect::new(
                0, y,
                width as u32, 1
            )).unwrap();

            // Floor
            let floor_color = cmp::max(cmp::min(y * 3 / 4, 255), 0);
            renderer.set_draw_color(Color::RGB(0, floor_color as u8, 0));
            renderer.fill_rect(Rect::new(
                0, height / 2 + y,
                width as u32, 1
            )).unwrap();
        };

        for x in 0i32..width {
            let camera_x = 2. * (x as f32) / (width as f32) - 1.;
            let raydir_x = dir_x + plane_x * camera_x;
            let raydir_y = dir_y + plane_y * camera_x;

            // which square are we in?
            let mut map_x = pos_x as i32;
            let mut map_y = pos_y as i32;

            // length of ray from one x or y-side to next x or y-side
            let deltadist_x = (1. + (raydir_y * raydir_y) / (raydir_x * raydir_x)).sqrt();
            let deltadist_y = (1. + (raydir_x * raydir_x) / (raydir_y * raydir_y)).sqrt();

            // what direction to step in x or y-direction (either +1 or -1)
            let step_x = if raydir_x < 0. { -1 } else { 1 };
            let step_y = if raydir_y < 0. { -1 } else { 1 };

            // length of ray from current position to next x or y-side
            let mut sidedist_x = if raydir_x < 0. {
                (pos_x - (map_x as f32)) * deltadist_x
            } else {
                ((map_x as f32) + 1. - pos_x) * deltadist_x
            };
            let mut sidedist_y = if raydir_y < 0. {
                (pos_y - (map_y as f32)) * deltadist_y
            } else {
                ((map_y as f32) + 1. - pos_y) * deltadist_y
            };

            // perform DDA
            let mut hit = false;
            let mut side = false; // was a NS or a EW wall hit?
            while !hit {
                // jump to next map square, OR in x-direction, OR in y-direction
                if sidedist_x < sidedist_y {
                    sidedist_x += deltadist_x;
                    map_x += step_x;
                    side = true;
                } else {
                    sidedist_y += deltadist_y;
                    map_y += step_y;
                    side = false;
                }

                // Check if ray has hit a wall
                if world_map[map_x as usize][map_y as usize] > 0 {
                    hit = true;
                }
            }

            // Calculate distance projected on camera direction
            // (oblique distance will give fisheye effect!)
            let perpwall_dist = if side {
                ((map_x as f32) - pos_x + ((1 - step_x) as f32) / 2.) / raydir_x
            } else {
                ((map_y as f32) - pos_y + ((1 - step_y) as f32) / 2.) / raydir_y
            };

            // Calculate height of line to draw on screen
            let line_height = (height as f32 / perpwall_dist) as i32;

            // Calculate lowest and highest pixel to fill in current stripe
            let draw_start = cmp::max(-line_height / 2 + height / 2, 0);
            let draw_end   = cmp::min( line_height / 2 + height / 2, height - 1);

            let tex_num = world_map[map_x as usize][map_y as usize] - 1;
            let wall_x = {
                let wx = if side {
                    pos_y + perpwall_dist * raydir_y
                } else {
                    pos_x + perpwall_dist * raydir_x
                };
                wx - wx.floor()
            };
            let tex_x: u32 = {
                let tx = (wall_x * (tex_size as f32)) as i32;
                if (side && raydir_x > 0.) || (!side && raydir_y < 0.) {
                    (tex_size - tx - 1) as u32
                } else {
                    tx as u32
                }
            };

            for y in draw_start..draw_end {
                let d = (y * 256 - height * 128 + line_height * 128) as u32;
                let tex_y = d * tex_size as u32 / line_height as u32 / 256;
                let tex_pos = tex_size as u32 * tex_y + tex_x;
                let color_i = textures[tex_num][tex_pos as usize];
                let r = color_i >> 16 & 0xFF;
                let g = color_i >> 8 & 0xFF;
                let b = color_i & 0xFF;
                let dist_mod = (perpwall_dist * 12.) as u32;
                let color = Color::RGB(
                    if r > dist_mod { (r - dist_mod) as u8 } else { 0 },
                    if g > dist_mod { (g - dist_mod) as u8 } else { 0 },
                    if b > dist_mod { (b - dist_mod) as u8 } else { 0 },
                );
                renderer.set_draw_color(color);
                renderer.draw_point(Point::new(x, y)).unwrap();
            }
        }

        let old_time = time;
        time = timer.ticks();
        let frame_time = (time - old_time) as f32 / 1000.;

        let move_speed = frame_time * 4.;
        let rot_speed = frame_time * 2.;

        event_pump.pump_events();

        {
            let kbstate = event_pump.keyboard_state();

            // Up
            if kbstate.is_scancode_pressed(Scancode::W) {
                if world_map[(pos_x + dir_x * move_speed) as usize][pos_y as usize] == 0 {
                    pos_x += dir_x * move_speed;
                }
                if world_map[pos_x as usize][(pos_y + dir_y * move_speed) as usize] == 0 {
                    pos_y += dir_y * move_speed;
                }
            }

            // Down
            if kbstate.is_scancode_pressed(Scancode::S) {
                if world_map[(pos_x - dir_x * move_speed) as usize][pos_y as usize] == 0 {
                    pos_x -= dir_x * move_speed;
                }
                if world_map[pos_x as usize][(pos_y - dir_y * move_speed) as usize] == 0 {
                    pos_y -= dir_y * move_speed;
                }
            }

            // Left
            if kbstate.is_scancode_pressed(Scancode::A) {
                if world_map[(pos_x - plane_x * move_speed) as usize][pos_y as usize] == 0 {
                    pos_x -= plane_x * move_speed;
                }
                if world_map[pos_x as usize][(pos_y - plane_y * move_speed) as usize] == 0 {
                    pos_y -= plane_y * move_speed;
                }
            }

            // Right
            if kbstate.is_scancode_pressed(Scancode::D) {
                if world_map[(pos_x + plane_x * move_speed) as usize][pos_y as usize] == 0 {
                    pos_x += plane_x * move_speed;
                }
                if world_map[pos_x as usize][(pos_y + plane_y * move_speed) as usize] == 0 {
                    pos_y += plane_y * move_speed;
                }
            }

            // Turn left
            if kbstate.is_scancode_pressed(Scancode::J) {
                //both camera direction and camera plane must be rotated
                let old_dir_x = dir_x;
                dir_x = dir_x * rot_speed.cos() - dir_y * rot_speed.sin();
                dir_y = old_dir_x * rot_speed.sin() + dir_y * rot_speed.cos();
                let old_plane_x = plane_x;
                plane_x = plane_x * rot_speed.cos() - plane_y * rot_speed.sin();
                plane_y = old_plane_x * rot_speed.sin() + plane_y * rot_speed.cos();
            }

            // Turn right
            if kbstate.is_scancode_pressed(Scancode::L) {
                //both camera direction and camera plane must be rotated
                let old_dir_x = dir_x;
                dir_x = dir_x * (-rot_speed).cos() - dir_y * (-rot_speed).sin();
                dir_y = old_dir_x * (-rot_speed).sin() + dir_y * (-rot_speed).cos();
                let old_plane_x = plane_x;
                plane_x = plane_x * (-rot_speed).cos() - plane_y * (-rot_speed).sin();
                plane_y = old_plane_x * (-rot_speed).sin() + plane_y * (-rot_speed).cos();
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                }
                _ => {}
            }
        }

        renderer.present();
    }
}
