fn calc_max(a: i32, b: i32) -> i32 {
    (a + b + (a - b).abs()) / 2
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let nums: Vec<&str> = input.split(' ').collect();
    let a = nums[0].trim().parse::<i32>().unwrap();
    let b = nums[1].trim().parse::<i32>().unwrap();
    let c = nums[2].trim().parse::<i32>().unwrap();
    let bigger: i32 = calc_max(calc_max(a, b), c);
    println!("{} eh o maior", bigger);
}
