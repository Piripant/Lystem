// to render: ffmpeg -r 60 -y -i images/out%d.png output.mp4
// to resize: -vf scale=iw*2:ih*2 -sws_flags neighbor

// TODO:
// Invisible forward
// Draw point
// Make the user choose the number of still frames at the end of the video with a cli parameter
// Maybe use macros to autogenerate &str -> Variable -> &mut self.xyz

mod lsystem;
mod scripting;
mod turtle;

use clap::{App, Arg};
use nalgebra::Vector2;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

type Vector2f = Vector2<f32>;

/// Parses the config yml files
#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    axiom: String,
    rules: HashMap<char, String>,
    commands: HashMap<char, Vec<String>>,
    start_state: HashMap<String, f64>,
}

fn main() {
    let matches = App::new("Lystem")
        .version("0.1")
        .author("Piripant <piripant@gmail.com>")
        .about("Simulates and draws L-Systems")
        .arg(
            Arg::with_name("CONFIG")
                .help("Sets the L-system config file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("GENERATIONS")
                .help("The number of generations to simulate")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("last_frame")
                .help("Only renders the last frame")
                .short("l")
                .long("last")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("steps")
                .help("How many steps per frame are rendered")
                .short("s")
                .long("steps")
                .takes_value(true),
        )
        .get_matches();

    let config_name = matches.value_of("CONFIG").unwrap();
    let config_file = File::open(config_name).unwrap();
    let config: Config = serde_yaml::from_reader(config_file).unwrap();

    // Retrive all the L-system rules and axiom
    let mut rules = lsystem::SystemRules::new();
    for (to, from) in config.rules {
        rules.add_rule(to as u8, from.as_bytes().to_vec());
    }
    let axiom = config.axiom.as_bytes().to_vec();
    let generations = matches.value_of("GENERATIONS").unwrap().parse().unwrap();
    let mut system_generations = lsystem::LSystem::new(axiom, generations);

    // Retrive the turtle settings / commands
    let mut pen = turtle::PenState::new();
    pen.load_config(&config.start_state).unwrap();
    let mut turtle = turtle::Turtle::new(pen);
    turtle.load_config(&config.commands).unwrap();

    // Start the Simulation
    let mut strokes = vec![];

    let mut xmin = std::f32::MAX;
    let mut ymin = std::f32::MAX;
    let mut xmax = std::f32::MIN;
    let mut ymax = std::f32::MIN;

    while let Some(symbols) = system_generations.iterate_over(&rules) {
        for symbol in symbols {
            let mut new_strokes = turtle.update(symbol);
            if !new_strokes.is_empty() {
                // Find the global max/min to later generate
                // An image of the right size
                for (from, to, _) in &mut new_strokes {
                    xmin = xmin.min(from.x).min(to.x);
                    ymin = ymin.min(from.y).min(to.y);

                    xmax = xmax.max(from.x).max(to.x);
                    ymax = ymax.max(from.y).max(to.y);
                }

                strokes.extend(new_strokes);
            }
        }
    }

    // Start the drawing
    let last_frame = matches.is_present("last_frame");

    // We will use min to make the coordinates all positive
    // And in this way transform them to image pixel coordinates
    let min = Vector2f::new(xmin, ymin);
    let mut img = image::ImageBuffer::new((xmax - xmin) as u32 + 1, (ymax - ymin) as u32 + 1);

    let step = matches.value_of("steps").map_or(1, |s| s.parse().unwrap());
    for i in (0..strokes.len()).step_by(step) {
        for j in 0..step {
            if i + j >= strokes.len() {
                break;
            }

            let (from, to, color) = strokes[i + j];
            draw_line(&mut img, color, from - min, to - min);
        }

        if !last_frame {
            img.save(format!("images/out{}.png", i / step)).unwrap();
            if i % (step * 4) == 0 {
                println!("Saved {}/{}", i / step + 1, strokes.len() / step);
            }
        }
    }

    // Draw 240 still frames at the end of the video
    // Or just one if the user requested only the last frame
    let still_frames = if last_frame { 1 } else { 240 };
    for i in 0..still_frames {
        img.save(format!("images/out{}.png", strokes.len() / step + i))
            .unwrap();

        if last_frame {
            println!(
                "last image saved on images/out{}.png",
                strokes.len() / step + i
            );
        }
    }
}

type Vector2i = Vector2<i32>;
fn draw_line(img: &mut image::RgbImage, color: [u8; 3], from: Vector2f, to: Vector2f) {
    let mut from = Vector2i::new(from.x as i32, from.y as i32);
    let to = Vector2i::new(to.x as i32, to.y as i32);

    let dx = (to.x - from.x).abs();
    let sx = if from.x < to.x { 1 } else { -1 };
    let dy = -(to.y - from.y).abs();
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        img.put_pixel(from.x as u32, from.y as u32, image::Rgb(color));

        if from.x == to.x && from.y == to.y {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            from.x += sx;
        }
        if e2 <= dx {
            err += dx;
            from.y += sy;
        }
    }
}