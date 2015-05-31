use super::point::Point;

#[allow(dead_code)]
pub struct Triangle
{
    pub p1 : Point,
    pub p2 : Point,
    pub p3 : Point
}

#[allow(dead_code)]
pub fn new (p1 : Point, p2 : Point, p3 : Point) -> Triangle
{
    Triangle
    {
        p1 : p1,
        p2 : p2,
        p3 : p3
    }
}

impl Triangle
{
    #[allow(dead_code)]
    #[allow(unused_variables)]    
    fn interset(&self, t : &Triangle) -> bool
    {
        true
    }
}
