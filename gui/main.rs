use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let mut timer_subsys = sdl_context.timer().unwrap();

    let window = video_subsys.window("bye javafx", WIDTH, HEIGHT)
                             .position_centered()
                             .build()
                             .unwrap();

    let mut canvas = window.into_canvas()
                           .build()
                           .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut color = 0x00_00_00u32;
    let color_mod = 0x100_00_00u32;
    let mut quit = false;
    while !quit {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyUp{ keycode: Some(Keycode::Escape), ..} => {
                    quit = true;
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB((color >> 8 & 0xff) as u8,
                                         (color >> 4 & 0xff) as u8,
                                         (color & 0xff) as u8));
        canvas.clear();
        canvas.present();

        color = (color + 0x10) % color_mod;
        timer_subsys.delay(1);
    }
}
