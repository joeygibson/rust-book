use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//fn main() {
//    use List::*;
//
//    let b = Box::new(5);
//
//    println!("b = {}", b);
//
////    let list = Cons(1, Cons(2, Cons(3, Nil)));
//    let list = Cons(1,
//                    Box::new(Cons(2,
//                                  Box::new(Cons(3,
//                                                Box::new(Nil))))));
//
//    println!("List: {:?}", list);
//
//
//}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(word: &str) -> String {
    format!("Hello, {}", word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testBoxDeref() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn testMyBoxDeref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn testDerefCoercion() {
        let s = "World";
        let m = MyBox::new(String::from("World"));

        assert_eq!("Hello, World", hello(s));
        assert_eq!("Hello, World", hello(&m));
    }
}
