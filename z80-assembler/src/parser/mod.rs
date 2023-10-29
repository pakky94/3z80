pub fn test() {
    println!("parser test module");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert!(true)
    }
}
