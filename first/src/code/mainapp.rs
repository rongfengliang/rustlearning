fn say_demo(name: &str) {
    println!("{}", name)
}

#[derive(Debug)]
struct User {
    name: &'static str,
    age: u32,
}
trait UserOperator<T> {
    fn appdemo(name:&str)->&str{
        "dalongdemo"
    }
    fn login(&self, name: &str) -> &str;
    fn f(n: T) {
        println!("{:#}","demo")
    }
}
impl User{
    fn first(){
        println!("{}","first method")
    }
    fn second(){
        println!("{}","second" )
    }
}
impl UserOperator<String> for User{
    fn login(&self, name: &str) -> &str {
        self.name
    }
}
fn main() {
    let user= User{
        name: "dalong",
        age: 333,
    };

    println!("{}",User::appdemo("aaaaa"));
    User::first();
    println!("{}",std::mem::size_of_val(user.login("dalong")))
}
