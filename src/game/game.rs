extern crate sdl2;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use game::entity::Entity;
use game::entity;

pub trait Game
{
    fn process_event(& mut self, event : &Event);
    fn process_frame_begin_event(& mut self);
    fn process_frame_end_event(& mut self);
    fn init(& mut self);
    fn shutdown(& mut self);
}

struct ShooterGame<'a>
{
    entities : Vec<Box<Entity>>,
    renderer : Renderer<'a>,
}

impl<'b> Game for ShooterGame<'b>
{
    fn process_event(& mut self, event : &Event)
    {
        for entity in &(self.entities)
        {
            entity.process_event(event);
        }
    }

    fn process_frame_begin_event(& mut self)
    {
        let mut drawer = self.renderer.drawer();
        drawer.clear();
    }

    fn process_frame_end_event(& mut self)
    {

        for entity in &(self.entities)
        {
            let drawer = self.renderer.drawer();
            entity.render(drawer);
        }
        let mut drawer = self.renderer.drawer();
        drawer.present();
    }


    fn init(& mut self)
    {
        let mut drawer = self.renderer.drawer();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }

    fn shutdown(& mut self)
    {

    }
}

pub fn build_game<'c> (renderer : Renderer<'c>) -> Box<Game + 'c>
{
    let mut vec = Vec::new();
    let log = entity::create_logger_event_entity();
    let player = entity::create_player_entity(&renderer);
    vec.push(log);
    vec.push(player);
    Box::new(ShooterGame {entities : vec, renderer : renderer})
}
