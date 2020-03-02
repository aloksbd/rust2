mod position;
mod object;

fn main() {
    let mut o = object::Object::new("o");
    println!("{} {} {}",o.x(),o.y(),o.name());

    o = object::Object::new_with_x_y("oo",10, 10);
    println!("{} {} {}",o.x(),o.y(),o.name());

    o.set_y(2);
    o.set_x(3);
    o.rename_to("no");
    println!("{} {} {}",o.x(),o.y(),o.name());

    let c1 = object::Object::new("c1");
    let c2 = object::Object::new("c2");
    let c3 = object::Object::new("c1");
    let c4 = object::Object::new("c4");
    println!("{}",o.add_child(c1.clone()));
    println!("{}",o.add_child(c2.clone()));
    println!("{}",o.add_child(c3.clone()));
    println!("{}",o.add_child(c4.clone()));
    o.print_children();

    println!("{}",o.remove_child(c1));
    o.print_children();
    println!("{}",o.remove_child(c2));
    o.print_children();
    println!("{}",o.remove_child(c4));
    o.print_children();
    println!("{}",o.remove_child(c3));
    o.print_children();
}
