fn panicdemo() ->!{
    panic!("some wrong")
}
fn appdemo(){
   |x:i32| x+1;
}
fn main() {
   let x =1;
   let mut y =44;
   let mut appdemo =89;
   match x{
       e @ 4...6 =>{
           println!("{}", e)
       },
       ref e =>{
          println!("{} ref demo",e )
       },
       _ =>{
           println!("{}","default" )
       },
   };
   match y {
       ref mut q =>{
           println!("ref mut{}", q)
       },
       _ =>{
        println!("{}", "default info")
       }
   }
   let myfn = |x:i32 | x+2;

   let fndemo = myfn;

   println!("{}", fndemo(33)) ;
  }
