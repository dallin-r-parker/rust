
fn main() {
    let rand_array = [1,2,4,5,2];

    println!("{}", rand_array[0]);
    println!("{}", rand_array.len());
    println!("Second 2 {:?}", &rand_array[1..2]);

    loop {
        if rand_array[0] == 1 {
            println!("You've matched index:{}", 1);
            break;
        }
    }
}