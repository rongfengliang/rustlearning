pub mod appdemoinfo
{
    pub struct User {
    }
    trait UserLogin {
        fn login(&self,name &str,pwd &str)->bool;
    }
    pub impl UserLogin for User{
        fn login(&self,name &str,pwd &str)->bool{
          if name=="dalong"&&pws=="dalong"{
              return true;
          }
          else{
              return false;
          }
        }
    }
}