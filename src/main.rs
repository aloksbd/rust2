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
}
