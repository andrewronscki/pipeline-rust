fn soma(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(not(feature = "no-main"))]
fn main() {
    let soma = soma(10, 5);
    println!("A soma é: {}", soma);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma() {
        assert_eq!(soma(2, 2), 4);
        assert_eq!(soma(-2, 2), 0);
        assert_eq!(soma(0, 0), 0);
        assert_eq!(soma(10, 5), 15);
    }
}
