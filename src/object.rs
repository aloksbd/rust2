use crate::position::Position;

pub struct Object{
    name: String,
    position: Position,
    children: Vec<Object>,
}

impl Object{
    pub fn new_with_x_y(_name: &str, _x: i32, _y: i32) -> Object {
        Object {
            name: _name.to_string(),
            position: Position::new_with_x_y(_x, _y),
            children: Vec::new(),
        }
    }
    pub fn new(_name: &str) -> Object{
        Object{
            name: _name.to_string(),
            position: Position::new(),
            children: Vec::new(),
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn rename_to(&mut self, _name: &str){
      while self.name.len() > 0{
          self.name.remove(0);
      }
      self.name.push_str(_name)
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