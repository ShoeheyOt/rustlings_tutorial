fn main() {
    // You can optionally experiment here.
    let numbers = (2,3,4,5);
    let third = numbers.2;
    println!("{}", third);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
        let second = numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
