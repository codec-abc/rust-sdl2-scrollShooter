extern crate sdl2;

use sdl2::keycode::KeyCode;
use sdl2::surface;

use sdl2::surface::Surface;
use sdl2::SdlResult;
use sdl2::render;

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

    let mut renderer = window.renderer().build().unwrap();
    let mut running = true;
    let mut game = game::game::buildGame(renderer);

    game.Init();
    while running
    {
        game.ProcessFrameBeginEvent();
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
                    game.ProcessEvent(&event);
                }
            }
        }
        game.ProcessFrameEndEvent();
    }
}
