fn main() {
    let rand_string = "I am a random string";

    println!("Length: {}", rand_string.len());

    let (first, second) = rand_string.split_at(6);
    println!("First {} & Second {}", first, second);

    let count = rand_string.chars().count();
    let mut chars = rand_string.chars();

    let mut indiv_char = chars.next();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_char = chars.next();
    }

    let mut iter = rand_string.split_whitespace();

    loop {
        match indiv_char {
            Some(x) => println!("{}", x),
            None => break
        }
    }

    println!("Find Best : {}", rand_string.contains("random"));
}