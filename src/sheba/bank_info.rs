pub(super) fn get_bank_info(s: u32) -> Option<ShebaResult> {
    Some(match s {
        10 => ShebaResult::new(
            "Central Bank of Iran",
            "010",
            "central-bank",
            "بانک مرکزی جمهوری اسلامی ایران",
        ),
        11 => ShebaResult::new(
            "Sanat O Madan Bank",
            "011",
            "sanat-o-madan",
            "بانک صنعت و معدن",
        ),
        12 => ShebaResult::new("Mellat Bank", "012", "mellat", "بانک ملت"),
        13 => ShebaResult::new("Refah Bank", "013", "refah", "بانک رفاه کارگران"),
        14 => ShebaResult::new("Maskan Bank", "014", "maskan", "بانک مسکن"),
        15 => ShebaResult::new("Sepah Bank", "015", "sepah", "بانک سپه"),
        16 => ShebaResult::new("Keshavarzi", "016", "keshavarzi", "بانک کشاورزی"),
        17 => ShebaResult::new("Melli", "017", "melli", "بانک ملی ایران"),
        18 => ShebaResult::new("Tejarat Bank", "018", "tejarat", "بانک تجارت"),
        19 => ShebaResult::new("Saderat Bank", "019", "saderat", "بانک صادرات ایران"),
        20 => ShebaResult::new(
            "Tose Saderat Bank",
            "020",
            "tosee-saderat",
            "بانک توسعه صادرات",
        ),
        21 => ShebaResult::new("Post Bank", "021", "post", "پست بانک ایران"),
        22 => ShebaResult::new(
            "Tosee Taavon Bank",
            "022",
            "toose-taavon",
            "بانک توسعه تعاون",
        ),
        51 => ShebaResult::new("Tosee Bank", "051", "tosee", "موسسه اعتباری توسعه"),
        52 => ShebaResult::new("Ghavamin Bank", "052", "ghavamin", "بانک قوامین"),
        53 => ShebaResult::new("Karafarin Bank", "053", "karafarin", "بانک کارآفرین"),
        54 => ShebaResult::new("Parsian Bank", "054", "parsian", "بانک پارسیان"),
        55 => ShebaResult::new(
            "Eghtesad Novin Bank",
            "055",
            "eghtesad-novin",
            "بانک اقتصاد نوین",
        ),
        56 => ShebaResult::new("Saman Bank", "056", "saman", "بانک سامان"),
        57 => ShebaResult::new("Pasargad Bank", "057", "pasargad", "بانک پاسارگاد"),
        58 => ShebaResult::new("Sarmayeh Bank", "058", "sarmayeh", "بانک سرمایه"),
        59 => ShebaResult::new("Sina Bank", "059", "sina", "بانک سینا"),
        60 => ShebaResult::new("Mehr Iran Bank", "060", "mehr-iran", "بانک مهر ایران"),
        61 => ShebaResult::new("City Bank", "061", "shahr", "بانک شهر"),
        62 => ShebaResult::new("Ayandeh Bank", "062", "ayandeh", "بانک آینده"),
        63 => ShebaResult::new("Ansar Bank", "063", "ansar", "بانک انصار"),
        64 => ShebaResult::new("Gardeshgari Bank", "064", "gardeshgari", "بانک گردشگری"),
        65 => ShebaResult::new(
            "Hekmat Iranian Bank",
            "065",
            "hekmat-iranian",
            "بانک حکمت ایرانیان",
        ),
        66 => ShebaResult::new("Dey Bank", "066", "dey", "بانک دی"),
        69 => ShebaResult::new("Iran Zamin Bank", "069", "iran-zamin", "بانک ایران زمین"),
        70 => ShebaResult::new("Resalat Bank", "070", "resalat", "بانک قرض الحسنه رسالت"),
        73 => ShebaResult::new(
            "Kosar Credit Institute",
            "073",
            "kosar",
            "موسسه اعتباری کوثر",
        ),
        75 => ShebaResult::new(
            "Melal Credit Institute",
            "075",
            "melal",
            "موسسه اعتباری ملل",
        ),
        78 => ShebaResult::new(
            "Middle East Bank",
            "078",
            "middle-east-bank",
            "بانک خاورمیانه",
        ),
        80 => ShebaResult::new(
            "Noor Credit Institution",
            "080",
            "noor-bank",
            "موسسه اعتباری نور",
        ),
        79 => ShebaResult::new(
            "Mehr Eqtesad Bank",
            "079",
            "mehr-eqtesad",
            "بانک مهر اقتصاد",
        ),
        90 => ShebaResult::new("Mehr Iran Bank", "090", "mehr-iran", "بانک مهر ایران"),
        95 => ShebaResult::new(
            "Iran and Venezuela Bank",
            "095",
            "iran-venezuela",
            "بانک ایران و ونزوئلا",
        ),

        _ => {
            return None;
        }
    })
}

#[derive(PartialEq, Eq, Debug)]
pub struct ShebaResult {
    pub(super) name: &'static str,
    pub(super) code: &'static str,
    pub(super) nickname: &'static str,
    pub(super) persian_name: &'static str,
}

impl ShebaResult {
    pub(super) fn new(
        name: &'static str,
        code: &'static str,
        nickname: &'static str,
        persian_name: &'static str,
    ) -> Self {
        ShebaResult {
            name,
            code,
            nickname,
            persian_name,
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
    pub fn get_code(&self) -> &'static str {
        self.code
    }
    pub fn get_nickname(&self) -> &'static str {
        self.nickname
    }
    pub fn get_persian_name(&self) -> &'static str {
        self.persian_name
    }
}
