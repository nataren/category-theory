use std::fmt;

fn id<T>(a: T) -> T {
    return a
}

fn main() {
    println!("{}", id(4));
    println!("{}", id("hola"));

    struct Pair(i32, f32);

    impl fmt::Display for Pair {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    println!("{}", id(Pair(3, 0.14)));
}
