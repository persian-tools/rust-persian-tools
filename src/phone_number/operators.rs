use crate::phone_number::{get_operator_prefix, is_phone_valid, PhoneNumberError};
use std::borrow::Cow;

pub mod constants {
    use super::*;

    pub static MCI: &[(&str, OperatorDetails)] = &[
        (
            "910",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "914",
            OperatorDetails {
                base: "آذربایجان غربی",
                province: Cow::Borrowed(&["آذربایجان شرقی", "اردبیل", "اصفهان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "911",
            OperatorDetails {
                base: "مازندران",
                province: Cow::Borrowed(&["گلستان", "گیلان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "912",
            OperatorDetails {
                base: "تهران",
                province: Cow::Borrowed(&[
                    "البرز",
                    "زنجان",
                    "سمنان",
                    "قزوین",
                    "قم",
                    "برخی از شهرستان های استان مرکزی",
                ]),
                sim_types: Cow::Borrowed(&[SimType::Permanent]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "913",
            OperatorDetails {
                base: "اصفهان",
                province: Cow::Borrowed(&["یزد", "چهارمحال و بختیاری", "کرمان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "915",
            OperatorDetails {
                base: "خراسان رضوی",
                province: Cow::Borrowed(&["خراسان شمالی", "خراسان جنوبی", "سیستان و بلوچستان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "916",
            OperatorDetails {
                base: "خوزستان",
                province: Cow::Borrowed(&["لرستان", "فارس", "اصفهان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "917",
            OperatorDetails {
                base: "فارس",
                province: Cow::Borrowed(&["بوشهر", "کهگیلویه و بویر احمد", "هرمزگان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "918",
            OperatorDetails {
                base: "کرمانشاه",
                province: Cow::Borrowed(&["کردستان", "ایلام", "همدان"]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "919",
            OperatorDetails {
                base: "تهران",
                province: Cow::Borrowed(&["البرز", "سمنان", "قم", "قزوین", "زنجان"]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "990",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "991",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "992",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "993",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "994",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "995",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
        (
            "996",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        ),
    ];

    pub static TALIYA: &[(&str, OperatorDetails)] = &[(
        "932",
        OperatorDetails {
            base: "کشوری",
            province: Cow::Borrowed(&[]),
            sim_types: Cow::Borrowed(&[SimType::Credit]),
            operator: Operator::Taliya,
            model: None,
        },
    )];

    pub static RIGHTTEL: &[(&str, OperatorDetails)] = &[
        (
            "920",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent]),
                operator: Operator::RightTel,
                model: None,
            },
        ),
        (
            "921",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::RightTel,
                model: None,
            },
        ),
        (
            "922",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::RightTel,
                model: None,
            },
        ),
        (
            "923",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::RightTel,
                model: None,
            },
        ),
    ];

    pub static IRANCELL: &[(&str, OperatorDetails)] = &[
        (
            "930",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "933",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "935",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "936",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "937",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "938",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "939",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "901",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "902",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "903",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "905",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "900",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit, SimType::Permanent]),
                operator: Operator::Irancell,
                model: None,
            },
        ),
        (
            "904",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::Irancell,
                model: Some("سیم‌کارت کودک"),
            },
        ),
        (
            "941",
            OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::Irancell,
                model: Some("TD-LTE"),
            },
        ),
    ];

    pub static SHATELMOBILE: &[(&str, OperatorDetails)] = &[(
        "998",
        OperatorDetails {
            base: "کشوری",
            province: Cow::Borrowed(&[]),
            sim_types: Cow::Borrowed(&[SimType::Credit]),
            operator: Operator::ShatelMobile,
            model: None,
        },
    )];
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Operator {
    ShatelMobile,
    MCI,
    Irancell,
    Taliya,
    RightTel,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SimType {
    Permanent,
    Credit,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'a")))]
pub struct OperatorDetails<'a> {
    province: Cow<'a, [&'a str]>,
    base: &'a str,
    model: Option<&'a str>,
    operator: Operator,
    sim_types: Cow<'a, [SimType]>,
}

impl Operator {
    pub fn details(&self) -> &'static [(&'static str, OperatorDetails<'static>)] {
        match self {
            Self::MCI => mci(),
            Self::Taliya => talia(),
            Self::RightTel => talia(),
            Self::Irancell => irancell(),
            Self::ShatelMobile => shatel_mobile(),
        }
    }
}

impl<'a> OperatorDetails<'a> {
    pub fn province_list(&'a self) -> &'a [&'a str] {
        &self.province
    }

    pub fn base(&self) -> &'a str {
        self.base
    }

    pub fn model(&self) -> Option<&'a str> {
        self.model
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn sim_type_list(&'a self) -> &'a [SimType] {
        &self.sim_types
    }
}

/// returns operator details for mci
#[inline(always)]
pub fn mci() -> &'static [(&'static str, OperatorDetails<'static>)] {
    constants::MCI
}

/// returns operator details for talia
#[inline(always)]
pub fn talia() -> &'static [(&'static str, OperatorDetails<'static>)] {
    constants::TALIYA
}

/// returns operator details for RightTel
#[inline(always)]
pub fn right_tel() -> &'static [(&'static str, OperatorDetails<'static>)] {
    constants::RIGHTTEL
}

/// returns operator details for irancell
#[inline(always)]
pub fn irancell() -> &'static [(&'static str, OperatorDetails<'static>)] {
    constants::IRANCELL
}

/// returns operator details for Shatel Mobile
#[inline(always)]
pub fn shatel_mobile() -> &'static [(&'static str, OperatorDetails<'static>)] {
    constants::SHATELMOBILE
}

/// returns all operators details
#[inline(always)]
pub fn all_operators() -> impl Iterator<Item = &'static (&'static str, OperatorDetails<'static>)> {
    mci()
        .iter()
        .chain(talia().iter())
        .chain(right_tel().iter())
        .chain(irancell().iter())
        .chain(shatel_mobile().iter())
}

/// a list of all available Iran operators prefixes
pub fn prefixes() -> Vec<&'static str> {
    all_operators().map(|(key, _)| *key).collect::<Vec<&str>>()
}

/// returns operator details of givin prefix for example (912, 919, 913)
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::operators::{get_prefix_details, SimType, Operator};
///
/// let details = get_prefix_details("910").expect("910 has details");
/// assert_eq!(details.base(), "کشوری");
/// // assert_eq!(details.province_list(), &[]);
/// assert_eq!(details.sim_type_list(), &[SimType::Permanent, SimType::Credit]);
/// assert_eq!(details.operator(), Operator::MCI);
///
/// assert!(get_prefix_details("9100").is_err());
/// ```
pub fn get_prefix_details(prefix: &str) -> Result<&OperatorDetails<'static>, PhoneNumberError> {
    let result = all_operators()
        .find(|(key, _)| key == &prefix)
        .map(|(_, details)| details);

    match result {
        Some(detail) => Ok(detail),
        None => Err(PhoneNumberError::InvalidPrefix(prefix.to_string())),
    }
}

/// returns operator details of givin phone number
///
/// # Examples
///
/// ```
/// use rust_persian_tools::phone_number::operators::{get_phone_details, SimType, Operator};
///
/// let details = get_phone_details("09195431812").expect("The number has details");
/// assert_eq!(details.base(), "تهران");
/// assert_eq!(details.province_list(), &["البرز", "سمنان", "قم", "قزوین", "زنجان"]);
/// assert_eq!(details.sim_type_list(), &[SimType::Credit]);
/// assert_eq!(details.operator(), Operator::MCI);
///
/// assert!(get_phone_details("009195431812").is_err());
/// ```
pub fn get_phone_details(
    phone_number: &str,
) -> Result<&OperatorDetails<'static>, PhoneNumberError> {
    is_phone_valid(phone_number)?;

    get_operator_prefix(phone_number).and_then(get_prefix_details)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_phone_prefix_operator() {
        assert_eq!(
            get_prefix_details("904").unwrap(),
            &OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::Irancell,
                model: Some("سیم‌کارت کودک"),
            },
        );

        assert_eq!(
            get_prefix_details("910").unwrap(),
            &OperatorDetails {
                base: "کشوری",
                province: Cow::Borrowed(&[]),
                sim_types: Cow::Borrowed(&[SimType::Permanent, SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        );

        assert!(get_prefix_details("9100").is_err());
    }

    #[test]
    fn test_get_phone_details() {
        assert_eq!(
            get_phone_details("09195431812").unwrap(),
            &OperatorDetails {
                base: "تهران",
                province: Cow::Borrowed(&["البرز", "سمنان", "قم", "قزوین", "زنجان"]),
                sim_types: Cow::Borrowed(&[SimType::Credit]),
                operator: Operator::MCI,
                model: None,
            },
        );

        assert!(get_phone_details("009195431812").is_err());
    }
}
