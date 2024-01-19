use super::E;

/// this should be 1 to validate sheba
/// ported from typescript implementation
pub fn sheba_iso7064_mod97(iban: impl AsRef<str>) -> Result<usize, E> {
    let mut remainder = iban.as_ref().to_string();

    while remainder.chars().count() > 2 {
        let block: String = remainder.chars().take(9).collect();
        let p1 = (block.parse::<usize>()? % 97).to_string();
        let p2 = &remainder
            .chars()
            .skip(block.chars().count())
            .collect::<String>();
        remainder = p1 + p2;
    }

    Ok(remainder.parse::<usize>()? % 97)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheba_iso7064_mod97_test() {
        assert_eq!(sheba_iso7064_mod97("820540102680020817909002"), Ok(1));
        assert_eq!(sheba_iso7064_mod97("550570022080013447370101"), Ok(6));
        // 550570022080013447370101
        assert_eq!(
            sheba_iso7064_mod97("012345678901234567890123456789"),
            Ok(44)
        );
        assert_eq!(sheba_iso7064_mod97("01234567890123456789"), Ok(10));
        assert_eq!(sheba_iso7064_mod97("012345678901234567890123"), Ok(19));
    }
}
