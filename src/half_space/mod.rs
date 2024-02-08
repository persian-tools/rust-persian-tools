/// removes half space & soft hyphon from text
pub fn remove_half_space(input: impl AsRef<str>) -> String {
    let input = input.as_ref();

    input
        .replace('\u{00AD}', "")
        .chars()
        .map(|ch| if ch == '\u{200C}' { ' ' } else { ch })
        .collect()
}

// add half space to input based on most useful
pub fn add_half_space(input: &str) -> String {
    let result = remove_half_space(input.trim())
        .replace("\u{0020}می\u{0020}", "\u{0020}می\u{200c}")
        .replace("\u{0020}نمی\u{0020}", "\u{0020}نمی\u{200c}")
        .replace("‌\u{0020}بی\u{0020}", "\u{0020}‌بی‌\u{200c}")
        .replace("\u{0020}ام\u{0020}", "‌ام‌\u{200c}")
        .replace("\u{0020}ات\u{0020}", "‌ات‌\u{200c}")
        .replace("\u{0020}اش\u{0020}", "‌اش‌\u{200c}")
        .replace("\u{0020}ای\u{0020}", "‌ای‌\u{200c}")
        .replace("\u{0020}اید\u{0020}", "‌اید‌\u{200c}")
        .replace("\u{0020}ایم\u{0020}", "‌ایم‌\u{200c}")
        .replace("\u{0020}اند\u{0020}", "‌اند‌\u{200c}")
        .replace("\u{0020}های\u{0020}", "‌های\u{0020}")
        .replace("\u{0020}ها\u{0020}", "‌ها\u{0020}")
        .replace("\u{0020}تر\u{0020}", "‌تر\u{0020}")
        .replace("\u{0020}تری\u{0020}", "‌تری\u{0020}")
        .replace("\u{0020}هایی\u{0020}", "‌هایی‌\u{200c}")
        .replace("\u{0020}هایم\u{0020}", "‌هایم‌\u{200c}")
        .replace("\u{0020}هایت\u{0020}", "‌هایت‌\u{200c}")
        .replace("\u{0020}هایش\u{0020}", "‌هایش‌\u{200c}")
        .replace("\u{0020}هایمان\u{0020}", "‌هایمان‌\u{200c}")
        .replace("\u{0020}هایتان\u{0020}", "‌هایتان‌\u{200c}")
        .replace("\u{0020}هایشان\u{0020}", "‌هایشان‌\u{200c}");

    // these section fixes the words that are started with می | نمی |‌بی
    if result.starts_with("می") {
        if let Some((_, temp)) = result.split_once(' ') {
            return format!("{}{}", "می\u{200c}", temp);
        }
    } else if result.starts_with("نمی") {
        if let Some((_, temp)) = result.split_once(' ') {
            return format!("{}{}", "نمی\u{200c}", temp);
        }
    } else if result.starts_with("بی") {
        if let Some((_, temp)) = result.split_once(' ') {
            return format!("{}{}", "‌بی‌\u{200c}", temp);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_half_space() {
        assert_eq!(
            add_half_space("نمی خواهی درخت ها را ببینیم؟"),
            "نمی‌خواهی درخت‌ها را ببینیم؟".to_string()
        );

        assert_eq!(
            add_half_space("ای دوست سلام من به تو. نمی خواهمت درخت های چنار هاله صمیمی من"),
            "ای دوست سلام من به تو. نمی‌خواهمت درخت‌های چنار هاله صمیمی من".to_string()
        );
        add_half_space("نمیخواهی"); // panic test
    }

    #[test]
    fn test_removing_half_space() {
        assert_eq!(
            remove_half_space("نمی‌خواهی درخت‌ها را ببینیم؟"),
            "نمی خواهی درخت ها را ببینیم؟".to_string()
        );
    }
}
