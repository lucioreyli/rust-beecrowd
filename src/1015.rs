fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let point: Vec<&str> = input.trim().split(' ').collect();
    let point_a: Vec<f32> = point
        .iter()
        .map(|v| v.trim().parse::<f32>().unwrap())
        .collect();
    input.clear();

    std::io::stdin().read_line(&mut input).expect("");
    let point: Vec<&str> = input.trim().split(' ').collect();
    let point_b: Vec<f32> = point
        .iter()
        .map(|v| v.trim().parse::<f32>().unwrap())
        .collect();

    let result =
        f32::sqrt((point_b[0] - point_a[0]).powf(2.0) + (point_b[1] - point_a[1]).powf(2.0));

    println!("{:.4}", result);
}
