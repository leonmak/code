use num::complex::Complex; //<1>

// adding objects which implement Add
fn main() {
  let a = Complex { re: 2.1, im: -1.2 }; //<2>
  let b = Complex::new(11.1, 22.2); //<3>
  let result = a + b;

  if a.re < b.re {
    let a = Complex { re: 2, im: 0 }; // variable scope starts when initialized
    println!("a = {}", a);
  }

  println!("({}) + ({}) = {} + {}𝑖", a, b, result.re, result.im) //<4>
}
