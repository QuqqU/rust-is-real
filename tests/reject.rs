#[cfg(test)]
mod automata_reject {
    use is_real::is_real;

    #[test]
    fn trailing_remains() {
        let real_number = "+12.3456E3E".to_string();
        assert_eq!(is_real(&real_number), false);
    }
}
