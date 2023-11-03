pub fn relative_delta(start: usize, dest: usize) -> Option<u8> {
    let delta = isize::try_from(dest).unwrap() - isize::try_from(start).unwrap();
    if delta > 127 || delta < -128 {
        return None;
    }

    Some(i8::try_from(delta).unwrap() as u8)
}

#[cfg(test)]
mod tests {
    use crate::compiler::utilities::relative_delta;

    #[test]
    fn relative_delta_happy_cases() {
        assert_eq!(Some(10), relative_delta(100, 110));
        assert_eq!(Some((-10i8) as u8), relative_delta(110, 100));
        assert_eq!(Some((-128i8) as u8), relative_delta(228, 100));
        assert_eq!(Some(127), relative_delta(100, 227));
    }

    #[test]
    fn relative_delta_error_cases() {
        assert_eq!(None, relative_delta(100, 310));
        assert_eq!(None, relative_delta(310, 100));
        assert_eq!(None, relative_delta(229, 100));
        assert_eq!(None, relative_delta(100, 228));
    }
}
