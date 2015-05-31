use sdl2::event::Event;
use sdl2::event::EventType;

use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::surface::Surface;
use sdl2::SdlResult;

pub trait Entity
{
    fn ProcessEvent(&self, event : &Event);
}

pub struct LoggerEventEntity;

impl Entity for LoggerEventEntity
{
    fn ProcessEvent(&self, event : &Event)
    {
        //println!("{:?}", event);
    }
}

pub fn createLoggerEventEntity() -> Box<Entity>
{
    let x = LoggerEventEntity;
    Box::new(x)
}

struct PlayerEntity<'a>
{
    isAlive : bool,
    texture : SdlResult<Surface<'a>>
}

impl<'a> Entity for PlayerEntity<'a>
{
    fn ProcessEvent(&self, event : &Event)
    {

    }
}

pub fn createPlayerEntity() -> Box<Entity>
{
    let x = Surface::load_bmp("test");
    let y = PlayerEntity{isAlive: true, texture: x};
    Box::new(y)
}
