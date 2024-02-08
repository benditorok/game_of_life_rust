use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use rand::{prelude::*, Error};
use std::ops::{Index, IndexMut};
use std::time::Duration;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rng = rand::thread_rng();

    let mut points: Vec<Vec<bool>> = vec![vec![false; WINDOW_HEIGHT as usize]; WINDOW_WIDTH as usize];
    
    // Init
    for row in &mut points  {
        for point in &mut row.iter_mut() {
            let value = rand::thread_rng().gen_range(0..=8);

            match value {
                1 => *point = true,
                _ => *point = false
            }
        }
    } 

    //canvas.set_scale(2.0, 2.0)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        points = calc_is_alive(&points).unwrap();

        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        canvas.draw_points(cells_to_points(&points).as_slice()).unwrap();

        canvas.present();
    }

    Ok(())
}

fn calc_is_alive(cells: &Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>, Error> {
    let mut edited = cells.to_owned();

    for (x, row) in edited.iter_mut().enumerate()  {
        for (y, point) in row.iter_mut().enumerate() {
            /* If the cell is alive, then it stays alive if it has either 2 or 3 live neighbors
            If the cell is dead, then it springs to life only in the case that it has 3 live neighbors */
            let mut neighbors = 0;

            if x > 0 && cells[x - 1][y] { neighbors += 1 } // Left
            if x > 0 && y > 0 && cells[x - 1][y - 1] { neighbors += 1 } // Top-left 
            if y > 0 && cells[x][y - 1] { neighbors += 1 } // Top
            if y > 0 && x < WINDOW_WIDTH as usize - 1 && cells[x + 1][y - 1] { neighbors += 1 } // Top right
            if x < WINDOW_WIDTH as usize - 1 && cells[x + 1][y] { neighbors += 1 } // Right
            if x < WINDOW_WIDTH as usize - 1 && y < WINDOW_HEIGHT as usize - 1 && cells[x + 1][y + 1] { neighbors += 1 } // Bottom right
            if y < WINDOW_HEIGHT as usize - 1 && cells[x][y + 1] { neighbors += 1 } // Bottom
            if x > 0 && y < WINDOW_HEIGHT as usize - 1 && cells[x - 1][y + 1] { neighbors += 1 } // Bottom left

            if *point && neighbors != 2 && neighbors != 3 {
                *point = false
            } else if  !*point && neighbors == 3 {
                *point = true
            }
        }
    } 

    Ok(edited)
}

fn cells_to_points(cells: &Vec<Vec<bool>>) -> Vec<Point> {
    let mut converted: Vec<Point> = Vec::new();  

    for (x, row) in cells.iter().enumerate()  {
        for (y, point) in row.iter().enumerate() {
            if *point {
                converted.push(Point::new(x as i32, y as i32));
            }
        }
    } 

    converted
}