
    #[derive(Debug)]
    pub enum UsState {
        Alabama,
        Alaska,
    }

    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    /**
     * 全部列举
     */
    pub fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime=>10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    /**
     * 通配符匹配某一模式时执行代码而忽略所有其他值。
     */
    pub fn value_in_cents2(coin: &Coin) -> u32 {
        let mut count = 0;
        match coin{
        Coin::Quarter(state) => {
            println!("_通配符匹配，State quarter from {:?}!", state);
            25
            },
            _ => {
                count+=1;
                count
            },
        }
    }
    /**
     * if let匹配某一模式
     */
    pub fn value_in_cents3(coin: &Coin) -> u32 {
        let mut count = 0;
        if let Coin::Quarter(state)=coin{
            println!("if let匹配某一模式,State quarter from {:?}!", state);
            25
        }
        else{
            count += 1;
            count
        }
    }
    /**
     * 
     */
    pub fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }


