fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  // use 'a 'b as lifetimes of i and j are decoupled
  // &'a binds lifetime variable a' to lifetime of i
  
  // values bound to a lifetime live as long as the
  // last read of ANY other value bound to the lifetime
  
  // lifetime elision rules only apply to references, not to generic parameters, so you must write Deserialze<'a>.
  *i + *j // <1>
}

fn main() {
  let res = add_with_lifetimes(&10, &20); // <2> <3>
  println!("{}", res);
}