pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap()
}

fn is_prime(n: &u32) -> bool {
    *n==2 || (2..ceil_sqrt(*n, 0, *n)+1).all(|i| *n%i!=0)
}

fn ceil_sqrt(x: u32, lower: u32, upper: u32) -> u32 {
    if lower+1 == upper {
        return upper
    }
    let middle = (lower + upper) / 2;
    if middle*middle >= x {
        ceil_sqrt(x, lower, middle)
    } else {
        ceil_sqrt(x, middle, upper)
    }
}
