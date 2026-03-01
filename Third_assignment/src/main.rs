// Diego Zuniga
fn most_frequent_word(text: &str) -> (String, usize) {  
    
    let words: Vec<&str> = text.split_whitespace().collect();      // collects the words inside a vector
    let mut max_count = 0;              // creates a max count variable
    let mut max_word = "";              // creates a max word variable

    for word in &words {                // for loop to traverse the vector
        let mut count = 0;              // temp variable to store the count for the current word
        for w in &words{                // second for loop that compares the current word to the following ones
            if w == word{               // updates the counter if it finds a match
                count += 1;
            }
        }
        if count > max_count{           // if the temp count is larger (meaning it found a more recurrent word) it updates max count and max word
            max_count = count;
            max_word = word;
        }
    }
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}