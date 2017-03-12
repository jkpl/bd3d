extern crate sdl2;

use std::cmp;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::rect::Point;

fn main() {
    let width: i32 = 640;
    let height: i32 = 480;

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

    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("BD 3D", width as u32, height as u32)
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
        renderer.set_draw_color(Color::RGB(0,50,0));
        renderer.fill_rect(Rect::new(0, height / 2, width as u32, height as u32 / 2)).unwrap();
        renderer.set_draw_color(Color::RGB(0,0,50));
        renderer.fill_rect(Rect::new(0, 0, width as u32, height as u32 / 2)).unwrap();

        for x in 0i32..width {
            let camera_x = 2. * (x as f32) / (width as f32) - 1.;
            let raypos_x = pos_x;
            let raypos_y = pos_y;
            let raydir_x = dir_x + plane_x * camera_x;
            let raydir_y = dir_y + plane_y * camera_x;

            // which square are we in?
            let mut map_x = raypos_x as i32;
            let mut map_y = raypos_y as i32;

            // length of ray from one x or y-side to next x or y-side
            let deltadist_x = (1. + (raydir_y * raydir_y) / (raydir_x * raydir_x)).sqrt();
            let deltadist_y = (1. + (raydir_x * raydir_x) / (raydir_y * raydir_y)).sqrt();

            // what direction to step in x or y-direction (either +1 or -1)
            let step_x = if raydir_x < 0. { -1 } else { 1 };
            let step_y = if raydir_y < 0. { -1 } else { 1 };

            // length of ray from current position to next x or y-side
            let mut sidedist_x = if raydir_x < 0. {
                (raypos_x - (map_x as f32)) * deltadist_x
            } else {
                ((map_x as f32) + 1. - raypos_x) * deltadist_x
            };
            let mut sidedist_y = if raydir_y < 0. {
                (raypos_y - (map_y as f32)) * deltadist_y
            } else {
                ((map_y as f32) + 1. - raypos_y) * deltadist_y
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
                ((map_x as f32) - raypos_x + ((1 - step_x) as f32) / 2.) / raydir_x
            } else {
                ((map_y as f32) - raypos_y + ((1 - step_y) as f32) / 2.) / raydir_y
            };

            // Calculate height of line to draw on screen
            let line_height = (height as f32 / perpwall_dist) as i32;

            // Calculate lowest and highest pixel to fill in current stripe
            let draw_start = cmp::max(-line_height / 2 + height / 2, 0);
            let draw_end   = cmp::min( line_height / 2 + height / 2, height - 1);

            let bright_mod = if side { 2 } else { 1 };

            let color = match world_map[map_x as usize][map_y as usize] {
                1 => Color::RGB(255 / bright_mod, 0, 0),
                2 => Color::RGB(0, 255 / bright_mod, 0),
                3 => Color::RGB(0, 0, 255 / bright_mod),
                4 => Color::RGB(255 / bright_mod, 255 / bright_mod, 255 / bright_mod),
                _ => Color::RGB(255 / bright_mod, 255 / bright_mod, 0),
            };

            renderer.set_draw_color(color);

            renderer.draw_line(
                Point::new(x as i32, draw_start),
                Point::new(x as i32, draw_end),
            ).unwrap();
        }

        let old_time = time;
        time = timer.ticks();
        let frame_time = (time - old_time) as f32 / 1000.;

        let move_speed = frame_time * 4.;
        let rot_speed = frame_time * 3.;

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
