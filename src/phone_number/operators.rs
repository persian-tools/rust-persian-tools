use std::collections::HashMap;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    ShatelMobile,
    MCI,
    Irancell,
    Taliya,
    RightTel,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SimType {
    Permanent,
    Credit,
}

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorDetails<'a> {
    pub province: Vec<&'a str>,
    pub base: &'a str,
    pub model: Option<&'a str>,
    pub operator: Operators,
    pub sim_types: Vec<SimType>,
}

/// returns operator details for mci
pub fn mci<'a>() -> HashMap<&'a str, OperatorDetails<'a>> {
    HashMap::from([
        (
            "910",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "914",
            OperatorDetails {
                base: "آذربایجان غربی",
                province: vec!["آذربایجان شرقی", "اردبیل", "اصفهان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "911",
            OperatorDetails {
                base: "مازندران",
                province: vec!["گلستان", "گیلان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "912",
            OperatorDetails {
                base: "تهران",
                province: vec![
                    "البرز",
                    "زنجان",
                    "سمنان",
                    "قزوین",
                    "قم",
                    "برخی از شهرستان های استان مرکزی",
                ],
                sim_types: vec![SimType::Permanent],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "913",
            OperatorDetails {
                base: "اصفهان",
                province: vec!["یزد", "چهارمحال و بختیاری", "کرمان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "915",
            OperatorDetails {
                base: "خراسان رضوی",
                province: vec!["خراسان شمالی", "خراسان جنوبی", "سیستان و بلوچستان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "916",
            OperatorDetails {
                base: "خوزستان",
                province: vec!["لرستان", "فارس", "اصفهان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "917",
            OperatorDetails {
                base: "فارس",
                province: vec!["بوشهر", "کهگیلویه و بویر احمد", "هرمزگان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "918",
            OperatorDetails {
                base: "کرمانشاه",
                province: vec!["کردستان", "ایلام", "همدان"],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "919",
            OperatorDetails {
                base: "تهران",
                province: vec!["البرز", "سمنان", "قم", "قزوین", "زنجان"],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "990",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "991",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "992",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "993",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "994",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "995",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
        (
            "996",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },
        ),
    ])
}

/// returns operator details for talia
pub fn talia() -> HashMap<&'static str, OperatorDetails<'static>> {
    HashMap::from([(
        "932",
        OperatorDetails {
            base: "کشوری",
            province: vec![],
            sim_types: vec![SimType::Credit],
            operator: Operators::Taliya,
            model: None,
        },
    )])
}

/// returns operator details for RightTel
pub fn right_tel() -> HashMap<&'static str, OperatorDetails<'static>> {
    HashMap::from([
        (
            "920",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent],
                operator: Operators::RightTel,
                model: None,
            },
        ),
        (
            "921",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::RightTel,
                model: None,
            },
        ),
        (
            "922",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::RightTel,
                model: None,
            },
        ),
        (
            "923",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::RightTel,
                model: None,
            },
        ),
    ])
}

/// returns operator details for irancell
pub fn irancell() -> HashMap<&'static str, OperatorDetails<'static>> {
    let basic_model = OperatorDetails {
        base: "کشوری",
        province: vec![],
        sim_types: vec![SimType::Credit, SimType::Permanent],
        operator: Operators::Irancell,
        model: None,
    };

    HashMap::from([
        ("930", basic_model.clone()),
        ("933", basic_model.clone()),
        ("935", basic_model.clone()),
        ("936", basic_model.clone()),
        ("937", basic_model.clone()),
        ("938", basic_model.clone()),
        ("939", basic_model.clone()),
        ("901", basic_model.clone()),
        ("902", basic_model.clone()),
        ("903", basic_model.clone()),
        ("905", basic_model.clone()),
        ("900", basic_model.clone()),
        (
            "904",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::Irancell,
                model: Some("سیم‌کارت کودک"),
            },
        ),
        (
            "941",
            OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::Irancell,
                model: Some("TD-LTE"),
            },
        ),
    ])
}

/// returns operator details for Shatel Mobile
pub fn shatel_mobile() -> HashMap<&'static str, OperatorDetails<'static>> {
    HashMap::from([(
        "998",
        OperatorDetails {
            base: "کشوری",
            province: vec![],
            sim_types: vec![SimType::Credit],
            operator: Operators::ShatelMobile,
            model: None,
        },
    )])
}

/// returns all operators details
pub fn all_operators() -> HashMap<&'static str, OperatorDetails<'static>> {
    let mut all_operators = HashMap::new();

    all_operators.extend(mci());
    all_operators.extend(talia());
    all_operators.extend(right_tel());
    all_operators.extend(irancell());
    all_operators.extend(shatel_mobile());

    all_operators
}

/// a list of all available Iran operators prefixes
pub fn prefixs() -> Vec<&'static str> {
    all_operators().into_keys().collect()
}

/// returns operator details of givin prefix for example (912, 919, 913)
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::operators::{*};
///
/// assert_eq!(
///     get_prefix_details("910"),
///     Some(OperatorDetails {
///         base: "کشوری",
///         province: vec![],
///         sim_types: vec![SimType::Permanent, SimType::Credit],
///         operator: Operators::MCI,
///         model: None,
///     },)
/// );
/// assert_eq!(get_prefix_details("9100"), None);
/// ```
pub fn get_prefix_details(prefix: &str) -> Option<OperatorDetails> {
    let mut all_prefixes = all_operators();

    all_prefixes.remove(prefix)
}

/// returns operator details of givin phone number
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::operators::{*};
///
/// assert_eq!(get_phone_details("09195431812") , Some(OperatorDetails {
///     base: "تهران",
///     province: vec!["البرز", "سمنان", "قم", "قزوین", "زنجان"],
///     sim_types: vec![SimType::Credit],
///     operator: Operators::MCI,
///     model: None,
/// },));
///
/// assert_eq!(get_phone_details("009195431812") , None);
/// ```
pub fn get_phone_details(phone_number: &str) -> Option<OperatorDetails> {
    if !super::is_phone_valid(phone_number) {
        return None;
    }

    let prefix = super::get_operator_prefix(phone_number).unwrap();
    get_prefix_details(prefix)
}

#[cfg(test)]
mod test_mobile_operators {
    use super::*;

    #[test]
    fn test_get_phone_prefix_operator() {
        assert_eq!(
            get_prefix_details("904"),
            Some(OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Credit],
                operator: Operators::Irancell,
                model: Some("سیم‌کارت کودک"),
            },)
        );

        assert_eq!(
            get_prefix_details("910"),
            Some(OperatorDetails {
                base: "کشوری",
                province: vec![],
                sim_types: vec![SimType::Permanent, SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },)
        );

        assert_eq!(get_prefix_details("9100"), None);
    }

    #[test]
    fn test_get_phone_details() {
        assert_eq!(
            get_phone_details("09195431812"),
            Some(OperatorDetails {
                base: "تهران",
                province: vec!["البرز", "سمنان", "قم", "قزوین", "زنجان"],
                sim_types: vec![SimType::Credit],
                operator: Operators::MCI,
                model: None,
            },)
        );

        assert_eq!(get_phone_details("009195431812"), None);
    }
}
