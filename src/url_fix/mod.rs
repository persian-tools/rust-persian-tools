use std::string::FromUtf8Error;
use urlencoding::decode;


/// description Used for fix Persian characters in URL<br>
/// separator: space by default
/// # Example:
/// ```
/// use rust_persian_tools::url_fix::url_fix;
/// assert_eq!(
///     url_fix("https://fa.wikipedia.org/wiki/%D9%85%D8%AF%DB%8C%D8%A7%D9%88%DB%8C%DA%A9%DB%8C:Gadget-Extra-Editbuttons-botworks.js",None),
///     Ok("https://fa.wikipedia.org/wiki/مدیاویکی:Gadget-Extra-Editbuttons-botworks.js".to_string())
/// );
/// ```
pub fn url_fix<S>(url: S, separator: Option<S>) -> Result<String, FromUtf8Error>
where
    S: AsRef<str>,
{
    let url: &str = url.as_ref();
    let url = decode(url)?;

    if let Some(separator) = separator {
        let separator: &str = separator.as_ref();
        return Ok(url.replace(' ', &separator));
    }

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn url_fix_test() {
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%D8%AF%DB%8C%D8%A7%D9%88%DB%8C%DA%A9%DB%8C:Gadget-Extra-Editbuttons-botworks.js",None),
            Ok("https://fa.wikipedia.org/wiki/مدیاویکی:Gadget-Extra-Editbuttons-botworks.js".to_string())
        );
        assert_eq!(
            url_fix("https://en.wikipedia.org/wiki/Persian_alphabet", None),
            Ok("https://en.wikipedia.org/wiki/Persian_alphabet".to_string())
        );
        assert_eq!(
            url_fix(
                "https://fa.wikipedia.org/wiki/%D8%AF%DB%8C%D8%A7%DA%A9%D9%88",
                None
            ),
            Ok("https://fa.wikipedia.org/wiki/دیاکو".to_string())
        );
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%DA%A9%D8%A7%D9%86%DB%8C%DA%A9%20%DA%A9%D9%88%D8%A7%D9%86%D8%AA%D9%88%D9%85%DB%8C", Some("_")),
            Ok("https://fa.wikipedia.org/wiki/مکانیک_کوانتومی".to_string())
        );
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%DA%A9%D8%A7%D9%86%DB%8C%DA%A9%20%DA%A9%D9%88%D8%A7%D9%86%D8%AA%D9%88%D9%85%DB%8C", Some("-")),
            Ok("https://fa.wikipedia.org/wiki/مکانیک-کوانتومی".to_string())
        );
        assert_eq!(url_fix("Sample Text", None), Ok("Sample Text".to_string()));
    }
}
