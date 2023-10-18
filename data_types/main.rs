fn tuple_destructure() {
    let tup = ("Lemons", 6.5, 29); 
    let (x, y, z) = tup;

    let q = tup.0;

    println!("{q}");

    println!("The value of x, y and are: {x} {y} {z}");
}

fn main() {
    tuple_destructure()
}
