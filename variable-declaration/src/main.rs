fn main() {
    println!("Learning Variable Definition");
    println!("==================================================");
    println!("let name = \"John Doe\"; // definition without mutability");
    println!("let mut name = \"John Doe\"; // definition with mutaibility");

    let name1 = "John Doe";         // Don't change value using assignment definition because an error will
                                    // occur
    
    //name1 = "John Wick"             // This will throw an error, because all the compiler knows at this
                                    // point is that this variable should not change 
    
    println!("Name1: {}", name1);

    let mut name2 = "John Doe";     // unless we add mut after let, this tells the compiler that the
                                    // value of this variable can be changed

    println!("Nmae2 before changed: {}", name2);

    name2 = "John Wick";            // I can change it because it uses mutability
    
    println!("Name2 after changed: {}", name2);
}

