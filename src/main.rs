mod position;
mod object;

fn main() {
    let mut parent = object::Object::new("parent");
    let mut child = object::Object::new("child");
    parent.add_child(&mut child);
    child.print_parent();

    let mut parent2 = object::Object::new("parent2");
    parent2.add_child(&mut child);
    child.print_parent();
}
