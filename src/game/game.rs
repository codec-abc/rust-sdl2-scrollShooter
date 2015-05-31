extern crate sdl2;
use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::render;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::pixels::Color;
use game::entity::Entity;
use game::entity;

pub trait Game
{
    fn ProcessEvent(& mut self, event : &Event);
    fn ProcessFrameBeginEvent(& mut self);
    fn ProcessFrameEndEvent(& mut self);
    fn Init(& mut self);
    fn Shutdown(& mut self);
}

struct ShooterGame<'a>
{
    entities : Vec<Box<Entity>>,
    renderer : Renderer<'a>,
}

impl<'b> Game for ShooterGame<'b>
{
    fn ProcessEvent(& mut self, event : &Event)
    {
        for entity in &(self.entities)
        {
            entity.ProcessEvent(event);
        }
    }

    fn ProcessFrameBeginEvent(& mut self)
    {
        let mut drawer = self.renderer.drawer();
        drawer.clear();
    }

    fn ProcessFrameEndEvent(& mut self)
    {

        for entity in &(self.entities)
        {
            let mut drawer = self.renderer.drawer();
            entity.Render(drawer);
        }
        /*
        render() ...
        match texture
        {
            Some(ref v) =>
            {
                drawer.copy(&v, None, None);
            }
            _ => {}
        }
        */
        let mut drawer = self.renderer.drawer();
        drawer.present();
    }


    fn Init(& mut self)
    {
        let mut drawer = self.renderer.drawer();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }

    fn Shutdown(& mut self)
    {

    }
}

pub fn buildGame<'c> (renderer : Renderer<'c>) -> Box<Game + 'c>
{
    let mut vec = Vec::new();
    let log = entity::createLoggerEventEntity();
    let player = entity::createPlayerEntity(&renderer);
    vec.push(log);
    vec.push(player);
    Box::new(ShooterGame {entities : vec, renderer : renderer})
}
