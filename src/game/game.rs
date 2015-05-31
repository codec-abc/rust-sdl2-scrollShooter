extern crate sdl2;
use sdl2::event::Event;
use sdl2::event::EventType;
use game::entity::Entity;
use game::entity;

pub trait Game
{
    fn ProcessEvent(&self, event : &Event);
}

struct Scene
{
    entities : Vec<Box<Entity>>
}

impl Game for Scene
{
    fn ProcessEvent(&self, event : &Event)
    {
        for entity in &self.entities
        {
            entity.ProcessEvent(event);
        }
    }
}

pub fn buildGame () -> Box<Game>
{
    let mut vec = Vec::new();
    let log = entity::createLoggerEventEntity();
    let player = entity::createPlayerEntity();
    vec.push(log);
    vec.push(player);
    let w = Scene {entities : vec};
    Box::new(w)
}
