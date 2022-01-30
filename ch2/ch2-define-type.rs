use std::ops::{Add};

struct Counter {
  value: u64, // <1>
}

impl Counter {
  fn new() -> Self { // <2> <3>
    Counter { value: 0 } // <4> <5>
  }

  fn incr(&mut self) { // <6>
    self.value += 1;
  }
}

struct Adder<T> {
  lhs: T,
}

// fn add<T: Add<Output=T>>(i: T, j: T) {
//   i+j
// }

// impl an apply mtd for using adder on any T 
impl<T: Add<Output=T> + Copy> Adder<T> {
  fn new(num: T) -> Self {
    Adder { lhs: num }
  }

  fn apply(&self, rhs: T) -> T {
    self.lhs + rhs
  }
}

// impl Add trait for composing Adders of T (which also impl Add)
// where same as impl<T: Add<Output=T> + Copy> Add for Adder<T> {
impl<T> Add for Adder<T>
where T: Add<Output=T> + Copy {
  type Output = Self;

  fn add(self, adder: Self) -> Self {
    Self { lhs: self.apply(adder.lhs) }
  }
}

fn main() {
   let mut counter = Counter::new();

   counter.incr();
   counter.incr();

   println!("{}", counter.value);

   let a = Adder {lhs: 2};
   let res = a.apply(23);
   println!("2+23={}", res);

   let b = a + Adder::new(3);
   println!("2+3+23={}", b.apply(23))
}
