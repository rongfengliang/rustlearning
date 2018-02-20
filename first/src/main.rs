mod app;
extern crate firstmod;
use firstmod::testsappdemo;
fn main() {
 app::applogin();
 let info=app::a::demo("dalong");
 println!("demo{}",info);
 testsappdemo::mylogin()
}
