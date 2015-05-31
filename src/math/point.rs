#[allow(dead_code)]
pub struct Point
{
    x : u32,
    y : u32
}

#[allow(dead_code)]
pub fn new() -> Point
{
    Point{x:0, y:0}
}

impl Point
{
    #[allow(dead_code)]
    fn distance(&self, p : &Point ) -> f32
    {
        let x_diff = self.x - p.x;
        let y_diff = self.y - p.y;
        let sum = (x_diff * x_diff + y_diff * y_diff) as f32;
        sum.sqrt()
    }
}
