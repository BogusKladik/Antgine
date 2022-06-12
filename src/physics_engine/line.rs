// TODO: Add functions for lines to make a map constraint out of lines

pub struct Line {
    position: HashMap<String, Vec2D>,
    vertex: HashMap<String, [Vec2D; 2]>,
    direction: HashMap<String, Vec2D>,
}