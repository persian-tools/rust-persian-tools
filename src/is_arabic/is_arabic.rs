pub fn is_arabic<S>(str: S) -> bool where S: Into<String>{ 
    let trim_pattern = regex::Regex::new(r#"[\"'\-\+\(\)\?\s.]"#).unwrap();
    let text: String = trim_pattern.replace_all(&str.into(), "").into_owned();

    let regex = regex::Regex::new(r"^[\u0600-\u06FF\s]+$").unwrap();
    let arabic_contextual_forms_regex = regex::Regex::new(r"[ي|ﻱ|ﻲ|ﻚ|ك|ﻚ|ﺔ|ﺓ|ة]").unwrap();

    regex.is_match(&text) 
    && arabic_contextual_forms_regex.is_match(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_arabic_test() {
		assert_eq!(is_arabic("هل هذا نص فارسي؟"),true);
		assert_eq!(is_arabic(
            "أكد رئيس اللجنة العسكرية الممثلة لحكومة الوفاق الوطني في ليبيا أحمد علي أبو شحمة، أن اللجنة لا تستطيع تنفيذ خطتها لإخراج العناصر الأجنبية من أراضي البلاد."
        ),true);
        
		assert_eq!(is_arabic("این یک متن عربی است؟"),false);
		assert_eq!(is_arabic("Lorem Ipsum Test"),false);
		assert_eq!(is_arabic("これはペルシア語のテキストですか"),false);
		assert_eq!(is_arabic("Это персидский текст?"),false);
		assert_eq!(is_arabic("这是波斯文字吗?"),false);
		assert_eq!(is_arabic(""),false);
    }
}