extern crate sdl2;
extern crate image;

use sdl2::keycode::KeyCode;

mod math;
mod game;

pub fn main()
{
    let mut sdl_context = sdl2::init().video().unwrap();

    let window = sdl_context.window("scrollShooter", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let renderer = window.renderer().build().unwrap();
    let mut running = true;
    let mut game = game::game::build_game(renderer);

    game.init();

    while running
    {
        game.process_frame_begin_event();
        for event in sdl_context.event_pump().poll_iter()
        {
            use sdl2::event::Event;
            match event
            {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } =>
                {
                    running = false
                },
                _ =>
                {
                    &game.process_event(&event);
                }
            }
        }
        game.process_frame_end_event();
    }
}
