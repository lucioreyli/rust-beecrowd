fn read_line(var_to_read: &mut String) -> usize {
    std::io::stdin()
        .read_line(var_to_read)
        .expect("Error read line")
}

fn main() {
    let mut employer_num_str = String::new();
    let mut hours_str = String::new();
    let mut salary_base_str = String::new();
    read_line(&mut employer_num_str);
    read_line(&mut hours_str);
    read_line(&mut salary_base_str);

    let hours: u32 = hours_str.trim().to_string().parse::<u32>().unwrap();
    let salary_base: f32 = salary_base_str.trim().to_string().parse::<f32>().unwrap();

    let final_salary: f32 = (hours as f32) * salary_base;

    print!("NUMBER = {}", employer_num_str);
    println!("SALARY = U$ {:.2}", final_salary);
}
