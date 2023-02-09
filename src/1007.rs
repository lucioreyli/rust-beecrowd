fn format_to_num(string_num: &mut String) -> i32 {
    string_num.trim().to_string().parse::<i32>().unwrap()
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();
    std::io::stdin().read_line(&mut a).expect("Error read line");
    std::io::stdin().read_line(&mut b).expect("Error read line");
    std::io::stdin().read_line(&mut c).expect("Error read line");
    std::io::stdin().read_line(&mut d).expect("Error read line");

    let num_a: i32 = format_to_num(&mut a);
    let num_b: i32 = format_to_num(&mut b);
    let num_c: i32 = format_to_num(&mut c);
    let num_d: i32 = format_to_num(&mut d);

    let result: i32 = (num_a * num_b) - (num_c * num_d);

    println!("DIFERENCA = {}", result);
}

