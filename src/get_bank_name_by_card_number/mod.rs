pub fn get_bank_name_by_card_number() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        get_bank_name_by_card_number();
        assert_eq!(1, 1)
    }
}
