use sdl2::event::Event;
use sdl2::event::EventType;

use sdl2::pixels::Color;
use sdl2::keycode::KeyCode;
use sdl2::surface::Surface;
use sdl2::SdlResult;
use sdl2::render::Renderer;
use sdl2::render::RenderDrawer;
use sdl2::render;

pub trait Entity
{
    fn ProcessEvent(&self, event : &Event);
    fn Render(&self, mut drawer : RenderDrawer);
    fn GetName(&self) -> &str;
}

pub struct LoggerEventEntity;

impl Entity for LoggerEventEntity
{
    fn ProcessEvent(&self, event : &Event)
    {
        println!("{:?}", event);
    }

    fn Render(&self, mut drawer : RenderDrawer)
    {

    }

    fn GetName(&self) -> &str
    {
        &"Logger"
    }
}

pub fn createLoggerEventEntity() -> Box<Entity>
{
    let x = LoggerEventEntity;
    Box::new(x)
}

struct PlayerEntity
{
    isAlive : bool,
    texture : Option<render::Texture>,
}

impl Entity for PlayerEntity
{
    fn ProcessEvent(&self, event : &Event)
    {

    }

    fn Render(&self, mut drawer : RenderDrawer)
    {
        match self.texture
        {
            Some(ref textureValue) =>
            {
                drawer.copy(textureValue, None, None)
            }
            _ => {}
        }

    }

    fn GetName(&self) -> &str
    {
        &"Player"
    }
}

pub fn createPlayerEntity(renderer : &Renderer) -> Box<Entity>
{
    let surface = Surface::load_bmp("../scrollShooter/test.bmp");
    let mut entity = PlayerEntity{isAlive: true, texture: None};
    match surface
    {
        Ok(ref v) =>
        {
            let texResult = renderer.create_texture_from_surface(v);
            match texResult
            {
                Ok(v2) =>
                {
                    entity.texture = Some(v2);
                }
                _ => { }
            }

        }
        Err(e) =>
        {
            println!("Error loading texture {}", e);
        }
    }
    Box::new(entity)
}
