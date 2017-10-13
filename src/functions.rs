fn main() {
    say_hello("Dallin");
    println!("4 + 5 = {}", get_sum(4, 5));

    let sum = get_sum;
    println!("10 + 10 = {}", sum(10, 10));

    let sum_nums = |x: i32, y: i32| x + y;
    println!("7 + 8 = {}", sum_nums(7,8));

    let num_ten = 10;

    let add_10 = |x: i32| x + num_ten;
    println!("5 + 10 = {}", add_10(5));

    let vect = vec![1,2,3,4,5,6,6,7];
    println!("Sum of Vect : {}", sum_vects(&vect));
}

fn  say_hello(name: &str) {
    println!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn sum_vects(v1: &Vec<i32>) -> i32 {
    let sum = v1.iter().fold(0, |mut sum, &x| {
        sum += x; sum
    });
    return sum;
}