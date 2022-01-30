fn main() {
  let search_term = "picture"; 
  // literals are &str by default as it's most efficient read-only, can be copied / popped off stack easily
  // use String when you want to pay extra cost for mutating string and managing memory on the heap
  // allocating and freeing memory means unknown size at compile allowing for updates

  // Rust compiler manages memory by imposing a set of ownership rules 
  // every variable has ONE owner variable, why one owner: need to allocate and free exactly once
  // variable drop() called to free memory when the variable's owner goes out of variable scope (closing backet)
  
  // other languages use a garbage collector (java/go) or manual free() calls (c++)
  // in c++ 'resource alloc as init', data and variables graph
  // {
  //     let s = String::from("hello"); // s is valid from this point forward
  // }                                  // this scope is now over, and s is no
  //                                    // longer valid
  // design choice won't create deep copies, assigning is allocating
  // passing as a variable function is moving ownership to the function call
  // if never return the param, calling fn no longer has access to memory after call exits and drop is called

  let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";     // <1>

  for line in quote.lines() {                    // <2>
    if line.contains(search_term) {
      println!("{}", line);
    }
  }
}