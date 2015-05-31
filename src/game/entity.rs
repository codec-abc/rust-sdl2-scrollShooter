use std::fs::File;

use image::ImageFormat;
use image::load;

use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::render::Renderer;
use sdl2::render::RenderDrawer;
use sdl2::render;
use sdl2::pixels::PixelFormatEnum;

pub trait Entity
{
    fn process_event(&self, event : &Event);
    fn render(&self, mut drawer : RenderDrawer);
    fn get_name(&self) -> &str;
}

pub struct LoggerEventEntity
{
    is_logging :bool,
}

impl Entity for LoggerEventEntity
{
    fn process_event(&self, event : &Event)
    {
        if self.is_logging
        {
            println!("{:?}", event);
        }
    }

    #[allow(unused_variables)]
    fn render(&self, drawer : RenderDrawer)
    {

    }

    fn get_name(&self) -> &str
    {
        &"Logger"
    }
}

impl LoggerEventEntity
{
    fn new(is_logging : bool) -> LoggerEventEntity
    {
        LoggerEventEntity{is_logging : is_logging}
    }
}

pub fn create_logger_event_entity() -> Box<Entity>
{
    let x = LoggerEventEntity::new(false);
    Box::new(x)
}

struct PlayerEntity
{
    #[allow(dead_code)]
    is_alive : bool,
    texture : Option<render::Texture>,
}

impl Entity for PlayerEntity
{
    #[allow(unused_variables)]
    fn process_event(&self, event : &Event)
    {

    }

    fn render(&self, mut drawer : RenderDrawer)
    {
        match self.texture
        {
            Some(ref texture_value) =>
            {
                drawer.copy(texture_value, None, None)
            }
            _ => {}
        }

    }

    fn get_name(&self) -> &str
    {
        &"Player"
    }
}

pub fn create_player_entity(renderer : &Renderer) -> Box<Entity>
{
    let mut entity = PlayerEntity{is_alive: true, texture: None};


    let f = File::open("../scrollShooter/res/PNG/playerShip1_blue.png");
    match f
    {
        Ok(value) =>
        {
            let image_result = load(value, ImageFormat::PNG);
            match image_result
            {
                Ok(img_value) =>
                {
                    let im_rgba = img_value.to_rgba();
                    let width = im_rgba.width();
                    let height = im_rgba.height();
                    let mut pixels = im_rgba.into_raw();
                    let format = PixelFormatEnum::RGBA8888;
                    let surface = Surface::from_data(&mut pixels, width, height, width * 4, format);
                    match surface
                    {
                        Ok(ref surf_value) =>
                        {
                            let texture_result = renderer.create_texture_from_surface(surf_value);
                            match texture_result
                            {
                                Ok(v2) =>
                                {
                                    entity.texture = Some(v2);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {println!("fail to open file as an image");}
            }
        }
        _ => {println!("fail to load file")}
    }
    Box::new(entity)
}
