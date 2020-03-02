use crate::position::Position;

#[derive( Clone)]
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

    // add a child if child with same name doesnot exist, return true if added
    pub fn add_child(&mut self,_child: Object) -> bool{
        let mut exists = false;
        for child in self.children.iter(){
            if child.name() == _child.name(){
                exists = true;
            }
        }
        if !exists{
            self.children.push(_child);
        }
        !exists
    }

    // remove a child with matching name if exist, return true if removed
    pub fn remove_child(&mut self, _child: Object) -> bool{
        let mut removed = false;
        let len = self.children.len();
        for i in 0..len{
            if self.children[i].name() == _child.name(){
                if i < len-1{  // if child is not last in array move it to _child's position and pop
                    self.children[i] = self.children[len-1].clone();
                }
                self.children.pop();
                removed = true;
                break;
            }
        }
        removed
    }

    pub fn print_children(&self){
        for child in self.children.iter(){
            println!("{}",child.name());
        }
    }
}