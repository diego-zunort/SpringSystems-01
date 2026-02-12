//Diego Zuniga

const freezep: f64 = 32.0;  // Constant for freezing point of water in fahrenheit

fn fahrenheit_to_celsius(f: f64) -> f64 {   // Function to conver F to C
    0.5556 * (f - 32.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {  // Function to convert C to F
    1.8 * c + 32.0
}

fn main(){          // Main function
    let mut tempf: f64 = freezep;   // Mutable variable with temp in fahrenheit

    let tempc: f64 = fahrenheit_to_celsius(tempf);  // Conversion of the fahrenheit temp to celsius and print of result
    println!("{tempc}");

    for _ in 0..5 {     // For loop repeated 5 times
        tempf += 1.0;   // Temperature incremented every iteration
        let c = fahrenheit_to_celsius(tempf);   // Conversion
        println!("{c}");    // Print of every result
    }
}