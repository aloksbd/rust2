use crate::position::Position;

pub struct Object{
    position: Position,
    children: Vec<Object>,
}

impl Object{
    pub fn new_with_x_y(_x: i32, _y: i32) -> Object {
        Object {
            position: Position::new_with_x_y(_x, _y),
            children: Vec::new(),
        }
    }
    pub fn new() -> Object{
        Object{
            position: Position::new(),
            children: Vec::new(),
        }
    }
    pub fn x(&self) -> i32 {
        self.position.x()
    }
    pub fn y(&self) -> i32 {
        self.position.y()
    }
    pub fn set_x(&mut self, _x: i32){
        self.position.set_x(_x);
    }
    pub fn set_y(&mut self, _y: i32){
        self.position.set_y(_y);
    }
}
