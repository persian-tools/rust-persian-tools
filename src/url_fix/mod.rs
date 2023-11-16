use urlencoding::decode;

/// description Used for fix Persian characters in URL <br>
/// separator: space by default <br>
/// 
/// Example:<br>
/// url_fix('https://fa.wikipedia.org/wiki/%D9%85%DA%A9%D8%A7%D9%86%DB%8C%DA%A9%20%DA%A9%D9%88%D8%A7%D9%86%D8%AA%D9%88%D9%85%DB%8C', '_');<br>
/// return 'https://fa.wikipedia.org/wiki/مکانیک_کوانتومی'<br>
/// @return {string} a string of fixed URL
pub fn url_fix<S>(url: S, separator: Option<S>) -> String where S: Into<String>{
    let url:String = url.into();
	let url = decode(&url).unwrap();

    if let Some(separator) = separator{
        let separator: String = separator.into();
        return url.replace(" ", &separator);
    }

	url.to_string()
}


#[cfg(test)]
mod tests{

    use super::*;


    #[test]
    fn url_fix_test() {
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%D8%AF%DB%8C%D8%A7%D9%88%DB%8C%DA%A9%DB%8C:Gadget-Extra-Editbuttons-botworks.js",None),
            "https://fa.wikipedia.org/wiki/مدیاویکی:Gadget-Extra-Editbuttons-botworks.js"
        );
        assert_eq!(
            url_fix("https://en.wikipedia.org/wiki/Persian_alphabet", None),
            "https://en.wikipedia.org/wiki/Persian_alphabet"
        );
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D8%AF%DB%8C%D8%A7%DA%A9%D9%88", None),
            "https://fa.wikipedia.org/wiki/دیاکو"
        );
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%DA%A9%D8%A7%D9%86%DB%8C%DA%A9%20%DA%A9%D9%88%D8%A7%D9%86%D8%AA%D9%88%D9%85%DB%8C", Some("_")),
            "https://fa.wikipedia.org/wiki/مکانیک_کوانتومی"
        );
        assert_eq!(
            url_fix("https://fa.wikipedia.org/wiki/%D9%85%DA%A9%D8%A7%D9%86%DB%8C%DA%A9%20%DA%A9%D9%88%D8%A7%D9%86%D8%AA%D9%88%D9%85%DB%8C", Some("-")),
            "https://fa.wikipedia.org/wiki/مکانیک-کوانتومی"
        );
        assert_eq!(
            url_fix("Sample Text", None),
            "Sample Text"
        );
    }
}