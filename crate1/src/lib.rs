pub trait MyTrait {
    fn func1(&self);

    // Library author adds this function
    // with a default implementation is still
    // a breaking change
    // fn func2(&self) {
    //     println!("LIBRARY FUNC 2")
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
