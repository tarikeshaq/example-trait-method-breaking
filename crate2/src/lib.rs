use crate1::MyTrait;
trait OtherTrait {
    fn func2(&self) {
        println!("OWN FUNC2")
    }
}
struct S;
impl MyTrait for S {
    fn func1(&self) {
        println!("Hello world!")
    }
}
impl OtherTrait for S {
    fn func2(&self) {
        println!("FUNC 2");
    }
}

pub fn my_func() {
    let s = S{};
    s.func2();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
