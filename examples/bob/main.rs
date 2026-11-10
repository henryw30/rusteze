use crate::car::Car;
use crate::cat::Cat;

mod car;
mod cat;

fn main() {
    println!("halksdjfello world");
    println!("lksjdfkl");
    let my_cat = Cat::new();
    let my_car = Car::new();

    println!("{} {}", my_cat.get_name(), my_cat.get_age());
    println!("hello");
    println!("{} ", my_car.miles);
}
