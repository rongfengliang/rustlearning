use std::rc::Rc;
use std::vec;
type Point = (u32, &'static str);
struct User {
    name: &'static str,
    age: i32,
}

struct Event {
    key: u32,
    count: u32,
    name: InternedString,
}
struct InternedString {
    value: Rc<String>,
}
#[derive(Default)]
struct PointApp{
  name:String,
  age:u32,
  appversion:String,
}
fn h(a:u32)-> Result<u32,String>{
   match a {
       a if a>10 => Ok(a),
       _ => Err(format!("input info is:{} wrong",a))
   }
}

fn main() {
    //   let mut array:[i32;3]=[0;3];
    //   array[1]=3;
    //   for item in &array{
    //       println!("{}",item );
    //   }

    // let v:Vec<i32> =Vec::new();
    // let v:Vec<i32> =vec![];
    // let v = vec![1,2,3,4];
    // let mut v = vec![0;10];
    // v.push(2);
    // let age =44;
    // match age {
    //     1...20 =>{
    //         println!("is to small")
    //     },
    //     21...30 =>{
    //         println!("is ok")
    //     },
    //     _ =>{
    //         println!("too big")
    //     }
    // }

    // let usernames =vec!["dalong","tom","jams"];
    // for item in usernames.iter(){
    //     println!("{}",item )
    // }
    // for item in usernames{
    //     println!("{}",item )
    // }
    let pointdefault =PointApp::default();
    let pointdefault2 =PointApp{
        name:"appdemo".to_string(),
        age:333,
        ..Default::default()
    };
    println!("default info:{},{},{}",pointdefault.name,pointdefault.age,pointdefault.appversion );
    println!("default info:{},{},{}",pointdefault2.name,pointdefault2.age,pointdefault2.appversion );

}
