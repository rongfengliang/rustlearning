
trait UserLogin {
  fn login(&self,token:&str) ->&str;
}
struct User{
  name:& 'static str,
  age:i32,
}

struct Applogin{
  appname:& 'static str,
  appversion:& 'static str,
}
impl UserLogin for User{
    fn login(&self,token:&str)->&str{
      "user login"
    }
}
impl UserLogin for Applogin{
  fn login(&self,token:&str)->&str{
      "app login"
    }
}
fn calllogin<T:UserLogin>(user:T,){
  let result= user.login("dalong");
  println!("{}",result )
}
fn main() {
  let userlogin=User{
    name:"dalong",
    age:444
  };
  let applogin =Applogin{
    appname:"first app",
    appversion:"v1.0",
  };
  calllogin(userlogin);
  calllogin(applogin);
}
  