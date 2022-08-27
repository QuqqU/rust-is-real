#[cfg(test)]
mod automata_accept {
    use is_real::is_real;

    #[test]
    fn form_full1() {
        let real_number = "+12.3456E+3".to_string();
        assert_eq!(is_real(&real_number), true);
    }

    #[test]
    fn form_full2() {
        let real_number = "-12.3456E-3".to_string();
        assert_eq!(is_real(&real_number), true);
    }

    #[test]
    fn form_full3() {
        let real_number = "-.3456E3".to_string();
        assert_eq!(is_real(&real_number), true);
    }

    #[test]
    fn form_full4() {
        let real_number = "-9909E3".to_string();
        assert_eq!(is_real(&real_number), true);
    }
}
