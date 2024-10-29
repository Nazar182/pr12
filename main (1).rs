use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, (usize, usize)) {
    data.windows(2)
        .enumerate()
        .map(|(i, w)| (w[0] + w[1], (i, i + 1)))
        .min_by_key(|&(sum, _)| sum)
        .unwrap()
}

fn print_result(data: &[i32]) {
    println!("Generated Vector: {:?}", data);
    let (min_sum, (i, j)) = min_adjacent_sum(data);
    println!(
        "Minimum adjacent sum: {} (elements: {} + {} at indices {} and {})",
        min_sum, data[i], data[j], i, j
    );
}

fn main() {
    let vec = gen_random_vector(20);
    print_result(&vec);
}
