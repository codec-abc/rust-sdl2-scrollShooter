extern crate sdl2;

use sdl2::pixels::Color;
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
    let game = game::game::buildGame();

    let surface = Surface::load_bmp("../scrollShooter/test.bmp");
    let mut texture : Option<render::Texture> = None;


    {
        let mut drawer = renderer.drawer();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }

    match surface
    {
        Ok(ref v) =>
        {
            let w = renderer.create_texture_from_surface(v);
            match w
            {
                Ok(v2) =>
                {
                    texture = Some(v2);
                }
                _ => { }
            }

        }
        Err(e) =>
        {
            println!("Error loading texture {}", e);
        }
    }

    while running
    {
        {
            let mut drawer = renderer.drawer();
            drawer.clear();
        }
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
                    &game.ProcessEvent(&event);
                }
            }
        }
        {
            let mut drawer = renderer.drawer();
            match texture
            {
                Some(ref v) =>
                {
                    drawer.copy(&v, None, None);
                }
                _ => {}
            }
            drawer.present();
        }
    }
}
