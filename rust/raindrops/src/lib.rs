pub fn raindrops(n: u32) -> String {
    let mut result = Vec::new();
    let rules = [
        (3, "Pling"),
        (5, "Plang"),
        (7, "Plong"),
    ];
    for (factor, sound) in &rules {
        if n % factor == 0 {
            result.push(sound);
        }
    }
    if result.len() == 0 {
        n.to_string()
    } else {
        result.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x)).to_string()
    }
}