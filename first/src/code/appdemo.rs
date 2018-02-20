#[macro_use]
extern crate rand;
extern crate log;
use std::fmt;
use rand::Rng;
#[derive(Debug)]
enum LoginType {
  MOBILE,
  WEB,
  PC
}
struct UserLogin{
   username:&str,
   userage:u32
}
impl UserLogin {
    fn username2(&mut self,name:String){ 
       self.username=name;
    }
    fn login(&self)->u32{
       self.userage
    }
}
fn main() {
  
  /*
   // if logic 
  let userlogin = if 1==1 {
    "user is include "
  } else{
    "Nothing is include"
  };
  println!("{}",userlogin )
  */
  /*
  //enum
  let logintype:LoginType=LoginType::MOBILE;

  */
  /*
  //struct
  let user:UserLogin=UserLogin{
      userage:44
  };
  let result =user.login();
   println!("{:?}",result ) 

    let t =("dalong",333);
  let dalong ="DAAAlong".to_string();
  println!("{},{}",t.1,t.0);
  let dalong2=String::new();
  let mut myuser =UserLogin{
      userage:33,
      username:String::from("dalongdffffffemo")
  };
  let newusername =String::from("rongfengliang");
  myuser.username2(dalong);
  */
  /*
   // string
  let h =String::from("hello");
  let w =String::from("world");
  let s= h+&w;
  println!("{}",s )
   */
  let words =String::from("dalongdemo");
  let chars =&words[..];
  println!("{}",chars );
  let result=rand::thread_rng().gen_range(10, 100);
  println!("{}",result );
}
