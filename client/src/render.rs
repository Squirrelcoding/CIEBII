
use std::{path::Path, thread, time::Duration};

use macroquad::{
    shapes::draw_rectangle,
    window::{next_frame, Conf}, miniquad::conf::Icon,
};
use ciebii_lib::io::read_file;

use colored::*;

use crate::icons;

pub fn render(file_name: String) -> anyhow::Result<()> {
    let shf = read_file(Path::new(&file_name));
    let shf = match shf {
        Ok(shf) => shf,
        Err(err) => {
            println!("{}", "An unexpected error has occured!".red().bold());

            panic!("{}", err);
        }
    };

    let (width, height) = shf.dimensions();

    macroquad::Window::from_config(
        Conf {
            window_title: "ciebii file viewer".to_owned(),
            window_width: width as i32,
            window_height: height as i32,
            icon: Some(Icon {
                small: icons::SMALL_ICON,
                medium: icons::MEDIUM_ICON,
                big: icons::LARGE_ICON,
            }),
            ..Default::default()
        },
        async move {
            let px_width = 1.0;
            let px_height = 1.0;

            let mut x = 0.0;
            let mut y = 0.0;

            shf.chunks().iter().for_each(|chunk| {
                let color = chunk.rgb().color();
                let color = macroquad::color::Color::from_rgba(color.0, color.1, color.2, 255);

                // ctx.dr

                draw_rectangle(x, y, px_width, px_height, color);

                x += px_width;

                if x >= width as f32 {
                    x = 0.0;
                    y += px_height;
                }
            });

            next_frame().await;
            loop {
                thread::sleep(Duration::from_secs(5));
            }
        },
    );

    Ok(())
}
// 