
// Diego Zuniga

const numbers: [i32; 10] = [21, 43, 5, 6, 87, 35, 11, 40, 75, 90]; // 10 integer array

fn is_even(n: i32) -> bool{   // Function is_even returns a bool variable
    n % 2 == 0
}

fn main(){

    for &num in numbers.iter(){    // For loop for Even or Odd check
        if is_even(num){               // If its even
            println!("{}: Even", num);
        }
        else{                                       // If its odd
            println!("{}: Odd", num);
        }
    }


    for &num in numbers.iter() {    // for loop for FizzBuzz check
        if num % 3 == 0 && num % 5 == 0 {   // if both conditions match, prints FizzBuzz
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0 {              //If its only divisible by 3, prints Fizz
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0 {              //If its only divisible by 5, prints Buzz
            println!("{}: Buzz", num);
        }
    }

    let mut i = 0;          // counter variable for printing while loop
    let mut sum = 0;        // varaible for storing the final sum

    while i < numbers.len(){       // While loop for sum
        sum += numbers[i];
        i += 1;
    }

    println!("The sum of all numebers is: {}", sum);    // Prints the result of the loop

    let mut largest = numbers[0];   // Variable to store the largest number
    let mut index = 0;              // counter for the loop

    loop{                           // loop
        if index >= numbers.len() {     // breaks once it has gone through the entire array
            break;
        }
        if numbers[index] > largest{    // if the current number in the array is larger than the current largest value, variable is updated
            largest = numbers[index];
        }
        index += 1;         // counter is updated
    }
    println!("Largest number is: {}", largest);     // message with the largest value is printed
}