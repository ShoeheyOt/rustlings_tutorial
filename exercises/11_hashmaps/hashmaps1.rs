// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =
    let mut basket = HashMap::new();
    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("orange"), 4);
    basket.insert(String::from("apple"), 7);
    basket.insert("grape".to_string(), 4);
    // TODO: Put more fruits in your basket.

    basket
}

fn main() {
    // You can optionally experiment here.
    let mut scores:HashMap<String,u32> = HashMap::new();
    scores.insert(String::from("blue"), 40);
    scores.insert(String::from("red"),30);

    let result = scores.get(&String::from("red"));
    match result {
        Some(value) => println!("The score is {}", value),
        None => println!("No score")
    }
    println!("scores {:?}", scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
