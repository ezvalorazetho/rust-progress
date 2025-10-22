fn main() {

    // For Loop Statement
    let range = 10;
    for i in 0..range {
        println!("(For Loop) Iteration: {}", i);
    }

    // While Loop Statement
    let mut i = 0;
    while i <= 10 {
        println!("(While Loop) Iteration: {}", i);
        i += 1;
    }

    // Infinity Loop
    let mut i = 0;
    loop {
        println!("(Infinity Loop) Iteration: {}", i);

        if i == 20 { break; }
        i += 1;
    }
}
