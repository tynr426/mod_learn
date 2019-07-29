mod english;
mod japanese;
mod match_crate;
mod my_lib;
fn main() {
  //mod match_crate与main同级
 same_layer_mod();
    my_lib::act_mod::run();
  //多个文件箱使用
  println!("Hello in English: {}", english::greetings::hello());
  println!("Goodbye in English: {}", english::farewells::goodbye());
}
fn same_layer_mod(){
    let coin=match_crate::Coin::Quarter(match_crate::UsState::Alaska);
    match_crate::value_in_cents(&coin);
    match_crate::value_in_cents2(&coin);
    match_crate::value_in_cents3(&coin);
    let five = Some(5);
    let six = match_crate::plus_one(five);
    let none = match_crate::plus_one(None);
    println!("value={:?}",six);
}
