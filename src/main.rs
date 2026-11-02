mod algebra {
    pub fn plus(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn minus(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn umnozh(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn deli(a: i32, b: i32) -> i32 {
        a / b
    }
}
fn main() {
    let sum = algebra::plus(10, 5);
    println!("sum is {}", sum);
    let sum = algebra::minus(10, 5);
    println!("raznost is {}", sum);
    let sum = algebra::umnozh(10, 5);
    println!("umn is {}", sum);
    let sum = algebra::deli(10, 5);
    println!("del is {}", sum);
}
