fn panicdemo() ->!{
    panic!("some wrong")
}
#[derive(Clone,Copy)]
struct Point {
    x:i32,
    y:i32,
}
fn hello<T>(h:T) ->T{
  return h;
}
fn demo<F:Fn(i32)->i32> (f:F,param:i32)->i32{
   f(param)
}

fn box_fn(x:i32)->Box< Fn(i32) -> i32>{
   Box::new(move|y| x+y)
}
fn main() {
   let info:&str= hello("dalong");
   println!("{}", info);
   let result =demo(|x:i32|x+100, 444);
   println!("{}", result);
   let boxfn =box_fn(444);
   println!("{}", boxfn(444))
  }
  