mod chars;
mod digit_converter;

use chars::{AR_DIGITS, EN_DIGITS, FA_DIGITS};
use digit_converter::digit_converter;

/// convert persian digits to english digits
/// ۰۱۲۳۴۵ -> 012345
pub fn digits_fa_to_en<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, FA_DIGITS, EN_DIGITS)
}

/// convert english digits to persian digits
/// 012345 -> ۰۱۲۳۴۵
pub fn digits_en_to_fa<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, EN_DIGITS, FA_DIGITS)
}

/// convert english digits to arabic digits
/// 012345 -> ٠١٢٣٤٥
pub fn digits_en_to_ar<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, EN_DIGITS, AR_DIGITS)
}

/// convert arabic digits to english digits
/// ٠١٢٣٤٥ -> 012345
pub fn digits_ar_to_en<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, AR_DIGITS, EN_DIGITS)
}

/// convert persian digits to arabic digits
/// ۰۱۲۳۴۵ -> ٠١٢٣٤٥
pub fn digits_fa_to_ar<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, FA_DIGITS, AR_DIGITS)
}

/// convert arabic digits to persian digits
/// ٠١٢٣٤٥ -> ۰۱۲۳۴۵
pub fn digits_ar_to_fa<S>(inp: S) -> String
where
    S: ToString,
{
    digit_converter(inp, AR_DIGITS, FA_DIGITS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_fa_to_en_test() {
        assert_eq!(digits_fa_to_en("123۴۵۶"), "123456");
        assert_eq!(digits_fa_to_en("۸۹123۴۵"), "8912345");
        assert_eq!(digits_fa_to_en("۰۱۲۳۴۵۶۷۸۹"), "0123456789");
    }

    #[test]
    fn digits_en_to_fa_test() {
        assert_eq!(digits_en_to_fa("123۴۵۶"), "۱۲۳۴۵۶");
        assert_eq!(digits_en_to_fa(123), "۱۲۳");
        assert_eq!(digits_en_to_fa(1234567891), "۱۲۳۴۵۶۷۸۹۱");
        assert_eq!(digits_en_to_fa(0), "۰");
        assert_eq!(digits_en_to_fa("٤٥٦"), "٤٥٦");
        assert_eq!(digits_en_to_fa("123۴۵۶"), "۱۲۳۴۵۶");
    }

    #[test]
    fn digits_en_to_ar_test() {
        assert_eq!(digits_en_to_ar(123456), "١٢٣٤٥٦");
        assert_eq!(digits_en_to_ar(1234567891), "١٢٣٤٥٦٧٨٩١");
        assert_eq!(digits_en_to_ar(0), "٠");
        assert_eq!(digits_en_to_ar("123٤٥٦"), "١٢٣٤٥٦");
    }

    #[test]
    fn digits_ar_to_en_test() {
        assert_eq!(digits_ar_to_en("٠١٢٣٤٥٦٧٨٩"), "0123456789");
        assert_eq!(digits_ar_to_en("89١٢٣4٥"), "8912345");
        assert_eq!(digits_ar_to_en("0123۴۵۶789"), "0123۴۵۶789");
    }

    #[test]
    fn digits_fa_to_ar_test() {
        assert_eq!(digits_fa_to_ar("۰۱۲۳۴۵۶۷۸۹"), "٠١٢٣٤٥٦٧٨٩");
        assert_eq!(digits_fa_to_ar("۱۷۸۲۳۴۰۵۶۹"), "١٧٨٢٣٤٠٥٦٩");
        assert_eq!(digits_fa_to_ar("۷۸٤۲۳٤۴"), "٧٨٤٢٣٤٤");
        assert_eq!(digits_fa_to_ar("٤٤٤444۴۴۴"), "٤٤٤444٤٤٤");
    }

    #[test]
    fn digits_ar_to_fa_test() {
        assert_eq!(digits_fa_to_ar("۰۱۲۳۴۵۶۷۸۹"), "٠١٢٣٤٥٦٧٨٩");
        assert_eq!(digits_fa_to_ar("۱۷۸۲۳۴۰۵۶۹"), "١٧٨٢٣٤٠٥٦٩");
        assert_eq!(digits_fa_to_ar("۷۸٤۲۳٤۴"), "٧٨٤٢٣٤٤");
        assert_eq!(digits_fa_to_ar("٤٤٤444۴۴۴"), "٤٤٤444٤٤٤");
    }
}
