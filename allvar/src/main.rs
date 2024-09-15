fn main() {
    // let x = 5;'

    /* Can't go over 4,294,967,295 because unsigned 32 bit */
    const MY_NUMBER : u32 = 2_900_003_213;
    let mut x = 5;
    println!("Certified number {}", x);
    x = 11;
    println!("Certified number {}", x);
    println!("My number for something random is {}", MY_NUMBER);

    let b = 1;
    println!("Currently the number is {}", b);
    let b = b + 1;
    println!("The Overshadow number? This is currently {}", b);

    const NORTH_BOUND : (f32, f32, f32) = (0.0, 0.0, 0.0);
    const NORTH_ASC_BOUND : (f32, f32, f32) = (0.0, 1.0, 0.0);
    const NORTH_DESC_BOUND : (f32, f32, f32) = (0.0, -1.0, 0.0);

    println!("Going North {:?}", NORTH_BOUND);
    println!("Going North Ascending {:?}", NORTH_ASC_BOUND);
    println!("Going North Descending {:?}", NORTH_DESC_BOUND);

    let dialog : [String; 5] = ["Hello", "Bye", "you", "going", "store"];
}

