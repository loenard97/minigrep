pub fn print(results: Vec<&str>) {
    for line in results {
        println!("{}", line);
    }
}

pub fn pretty_print(results: Vec<&str>) {
    for (i, line) in results.iter().enumerate() {
        println!("{} {}", i, line);
    }
}
