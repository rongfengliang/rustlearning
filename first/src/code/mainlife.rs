trait UserLogin<T, E> {
  fn login(&self, &T, &E) -> &str;
}
struct User;
struct First {
  name: String,
  age: i32,
}
struct Second {
  name: String,
  age: i32,
}
impl UserLogin<First, Second> for User {
  fn login(&self, n: &First, m: &Second) -> &str {
    "dalong"
  }
}
#[derive(Clone, Copy, Debug)]
struct AppDemo {
  name: &'static str,
  myage: i32,
}
impl AppDemo {
  fn a(self) {
    println!("{}", self.myage)
  }
}
impl std::ops::Add for AppDemo {
  type Output = AppDemo;
  fn add(self, rhs: AppDemo) -> AppDemo {
    AppDemo {
      name: self.name,
      myage: self.myage + rhs.myage,
    }
  }
}
fn say_hello(name: &str, func: fn(&str)) {
  func(name)
}
fn appdemo(n: i32) -> &'static str {
  "dalong"
}
type IncFunc = fn(i32) -> i32;
trait Foo {
  fn method(&self) -> String;
}
impl Foo for String {
  fn method(&self) -> String {
    format!("string,{}", *self)
  }
}
fn rong<'a>(x: &str) -> &str {
  x
}
fn rong2<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x > y {
    x
  } else {
    y
  }
}
fn rong3<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
  if x > y {
    x
  } else {
    y
  }
}
fn do_method(x: &Foo) -> String {
  x.method()
}
fn main() {
  let x: i32 = 100;
  let some_closure = move |i: i32| i + x;
  let y = some_closure(2);
  println!("x={},y={}", x, y)
}
