use std::cell::Cell;
#[derive(Default, Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}
mod myappdemo {
    #[derive(Default, Debug, Clone, Copy)]
    pub struct User {
        pub x: u32,
        pub y: u32,
    }
    impl User {
        fn first() {
            println!("{}", "myappdemo mod first User")
        }
    }
}
#[derive(Default, Debug)]
struct Point3D {
    point: Cell<Point>,
    center: u32,
}
fn main() {
    let myappuser = myappdemo::User { x: 44, y: 44 };
    println!("{}",myappuser.x );
    let defaultuser =myappdemo::User::default();
    println!("{}",defaultuser.x );

}
