mod english;
mod japanese;
mod match_crate;
mod my_lib;
use my_lib::act_mod as act;
fn main() {
  //mod match_crate与main同级
 same_layer_mod();
 //mod 在单独的文件中
  folder_crate();
  //多个文件箱使用
  many_crates();
}
/**
 * 模块与main同级下
 */
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
/**
 * 将模块单独放在一个文件夹下
 */
fn folder_crate(){
    my_lib::act_mod::run();
    act::run();
}
/**
 * 多个模块使用 * 
 */
fn many_crates(){
  println!("Hello in English: {}", english::greetings::hello());
  println!("Goodbye in English: {}", english::farewells::goodbye());
}
