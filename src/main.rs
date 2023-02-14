use std::io;

fn main() {
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("");

        let wagons_length: i32 = input.trim().parse().expect("Isn't a int num");

        if wagons_length == 0 {
            break;
        }

        let i: i32 = -1;
        while i != 0 {
            if i == 0 {
                println!("Yes");
                continue;
            }
            input.clear();
            io::stdin().read_line(&mut input).expect("");

            let wagons: Vec<u32> = input
                .trim()
                .split(' ')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            let can_reorder: bool = wagons.
        }
    }
}
