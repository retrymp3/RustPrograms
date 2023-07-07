//This program won't compile, because in rust, the boolean value can't be determined by numbers/any other values.
// fn main(){
//      let x=1;
//      if x {
//          println!("True");
//      }
//      else{
//          println!("False");
//      }
// }

//This will compile.
fn conds() -> &str{
    let x=3;
    if x<5 {
        return "less";
    }
    else{
        return "more";
    }
}
