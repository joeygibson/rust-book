fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let n_res = largest(&number_list);
    let c_res = largest(&char_list);

    println!("N: {}, C: {}", n_res, c_res);
}

//struct Point<T> {
//    x: T,
//    y: T,
//}
//
//impl<T> Point<T> {
//    fn x(&self) -> &T {
//        &self.x
//    }
//}
//
//impl Point<f32> {
//    fn distance_from_origin(&self) -> f32 {
//        (self.x.powi(2) + self.y.powi(2)).sqrt()
//    }
//}
//
//fn main() {
//    let integer = Point{x: 5, y: 10};
//    let float = Point{x: 5.0, y: 10.0};
//
//    println!("X: {}", integer.x());
//    println!("D: {}", float.distance_from_origin());
//}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

//fn main() {
//    let p1 = Point{x: 5, y: 10.4};
//    let p2 = Point{x: "Hello", y: 'c'};
//
//    let p3 = p1.mixup(p2);
//
//    println!("p3.x: {}, px.y: {}", p3.x, p3.y);
//}