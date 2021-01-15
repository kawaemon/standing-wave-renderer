use {
    sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point},
    std::{f64::consts, time::Duration},
};

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() -> Result<(), String> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window("standing_wave", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|x| x.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|x| x.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump()?;
    let mut offset = 0.0;

    'main: loop {
        for ev in event_pump.poll_iter() {
            match ev {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // rad / pixel
        const STEP: f64 = (6.0 * consts::PI) / (WIDTH as f64);
        const Y_MULTIPLIER: f64 = 0.25;
        const PADDING: f64 = (1.0 - Y_MULTIPLIER) / 2.0 * (HEIGHT as f64);

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut wave1 = vec![];
        let mut wave2 = vec![];

        {
            canvas.set_draw_color(Color::RGB(255, 0, 0));

            let mut theta: f64 = offset;
            let mut before_y = None;
            for x in 0..WIDTH {
                let y = (((theta.sin() + 1.0) / 2.0) * (HEIGHT as f64 * Y_MULTIPLIER)) + PADDING;

                wave1.push(theta);

                if let Some(before_y) = before_y {
                    canvas.draw_line(
                        Point::new((x - 1) as _, before_y as _),
                        Point::new(x as _, y as _),
                    );
                }

                before_y = Some(y);

                theta += STEP;
            }
        }

        {
            canvas.set_draw_color(Color::RGB(0, 255, 0));

            let mut theta: f64 = (2.0 * consts::PI) - offset;
            let mut before_y = None;
            for x in 0..WIDTH {
                let y = (((theta.sin() + 1.0) / 2.0) * (HEIGHT as f64 * Y_MULTIPLIER)) + PADDING;

                wave2.push(theta);

                if let Some(before_y) = before_y {
                    canvas.draw_line(
                        Point::new((x - 1) as _, before_y as _),
                        Point::new(x as _, y as _),
                    );
                }

                before_y = Some(y);

                theta += STEP;
            }
        }

        {
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            let mut before_y = None;
            for ((x, y1), y2) in (0..WIDTH).zip(&wave1).zip(&wave2) {
                let y = ((((y1.sin() + y2.sin()) + 1.0) / 2.0) * (HEIGHT as f64 * Y_MULTIPLIER))
                    + PADDING;

                if let Some(by1) = before_y {
                    canvas.draw_line(
                        Point::new((x - 1) as _, by1 as _),
                        Point::new(x as _, y as _),
                    );
                }

                before_y = Some(y);
            }
        }

        canvas.present();

        std::thread::sleep(Duration::from_millis(1000 / 60));
        offset = (offset + 0.1) % (2.0 * consts::PI);
    }

    Ok(())
}
