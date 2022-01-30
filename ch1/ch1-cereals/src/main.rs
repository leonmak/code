#[derive(Debug)]    // <1> macro auto extend struct to get Debug behavior
enum Cereal {       // <2> can add data to enum, but it's still 1 type
    S(i32, i32, i32),
    Move { x: i32, y: i32 },
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];   // <3>
    grains.push(Cereal::Rye);               // <4>
    grains.push(Cereal::Rye);               // <4>
    grains.push(Cereal::Move {x:1, y:1});               // <4>
    grains.push(Cereal::Rye);               // <4>
    // drop(grains);                           // <5>

    // type inference, rust is statically typed, need to know data type when compiled
    // Use usize and isize when it’s related to memory size, as it will expand bit per architecture
    // – the size of an object, or indexing a vector, for instance

    println!("{:?}", grains[2_usize]);               // <6>
}
