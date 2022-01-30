fn main() {
  let one             = [1, 2, 3];
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [1; 3];  // init with 3 elements of 1
  // array type [T:n] [u8; 3] (contiguous block of memory of 3 u8-type elements)
  // interactions are normally through slice type [T]
  
  // traits are 1 type for many slices so it's easier to add methods
  // arrays have many for the same element type - [u8; 3] is 1 type
  // most slices use references as compiler wants to know the size

  let arrays = [one, two, blank1, blank2];

  for a in &arrays { // iteration by reference
    print!("{:?}: ", a);
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t(Σ{:?} = {})", a, sum);
  }
}