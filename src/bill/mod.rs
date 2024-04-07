//! Iranian Utility Bill tools (`bill` Cargo feature).
//!
//! The structure of Bill/Payment IDs and Barcode is based on [مستندات طرح هماهنگ پرداخت الکترونیکی قبوض - کمیسیون انفورماتیک بانک‌ها](https://core.pod.ir/nzh/file/?fileId=334917&hashCode=16e5a9ee893-0.22579290613133673)
//!
//! #### Example
//!
//! ##### Load from Barcode  
//! A barcode must be 26 characters  
//! if checksum of Bill or Payment ID separately or checksum of them together be invalid Result will return `Err::BillError`
//!  
//! ```rust
//! use rust_persian_tools::bill::{Bill, BillID, PaymentID};
//! use rust_persian_tools::bill::BillType;
//! use rust_persian_tools::bill::CurrencyType;
//! use std::str::FromStr;
//!
//! // This checks both bill/payment IDs and their relation match (checksum2)
//! let bill = Bill::from_str("77483178001420000001770160").unwrap();
//! assert_eq!(bill.get_bill_type(), BillType::Tel);   // Fixed Landline
//! assert_eq!(bill.get_bill_id(), "7748317800142");
//! assert_eq!(bill.get_payment_id(), "1770160");
//! assert_eq!(bill.amount(CurrencyType::Rials), 17000);
//! ```
//!
//! ##### Load from Bill and Payment IDs  
//! Each can be any string with or without leading zeros  
//! if checksum of Bill or Payment ID separately or checksum of them together be invalid Result will return `Err::BillError`
//! ```rust  
//! use rust_persian_tools::bill::{Bill, BillID, PaymentID};
//! use rust_persian_tools::bill::CurrencyType;
//! use rust_persian_tools::bill::BillType;
//! use std::str::FromStr;
//!
//! let bill_id = BillID::from_str("7748317800142").unwrap();
//! let payment_id = PaymentID::from_str("1770160").unwrap();
//! let bill = Bill::new(bill_id, payment_id).unwrap();
//! assert_eq!(bill.get_bill_type(), BillType::Tel);   // Fixed Landline
//! assert_eq!(bill.get_bill_id(), "7748317800142");
//! assert_eq!(bill.get_payment_id(), "1770160");
//! assert_eq!(bill.amount(CurrencyType::Rials), 17000);
//! ```
//!
//! ##### Generate IDs from scratch  
//! This will calculate checksums automatically
//! ```rust
//! use rust_persian_tools::bill::{Bill, BillID, PaymentID};
//! use rust_persian_tools::bill::CurrencyType;
//! use rust_persian_tools::bill::BillType;
//!
//! let bill_id = BillID::new("77483178", "001", BillType::Tel).unwrap();
//! let payment_id = PaymentID::new(17, 7, 1, &bill_id).unwrap();  // Thousands are omitted from amount
//! let bill = Bill::new(bill_id, payment_id).unwrap();
//! assert_eq!(bill.get_bill_type(), BillType::Tel);   // Fixed Landline
//! assert_eq!(bill.get_bill_id(), "7748317800142");
//! assert_eq!(bill.get_payment_id(), "1770160");
//! assert_eq!(bill.amount(CurrencyType::Rials), 17000);
//! assert_eq!(bill.amount(CurrencyType::Tomans), 1700);
//! assert_eq!(bill.to_string(), "77483178001420000001770160");
//! ```

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::From;
use std::str::FromStr;
use std::string::ToString;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, thiserror::Error)]
pub enum BillError {
    #[error("Barcode length must be 26 chars")]
    InvalidBarcodeLength,
    #[error("Payment ID length must be >= 6 and <= 13")]
    InvalidPaymentIDLength,
    #[error("Payment ID is not valid")]
    InvalidPaymentID,
    #[error("Bill ID length must be >= 6 and <= 13")]
    InvalidBillIDLength,
    #[error("Bill Amount is not parsable")]
    InvalidBillIDAmount,
    #[error("Bill Year is not parsable")]
    InvalidBillIDYear,
    #[error("Bill Period is not parable")]
    InvalidBillIDPeriod,
    #[error("Checksum doesn't match")]
    InvalidBillChecksum,
    #[error("Bill Type is not parsable")]
    InvalidBillType,
    #[error("Cannot convert to digit")]
    InvalidDigits,
}

/// Values Are based on the مستندات طرح هماهنگ پرداخت الکترونیکی قبوض - کمیسیون انفورماتیک بانک‌ها \
/// It is possible to use `BillType` directly or from integer value:  
/// ```rust
/// use num::FromPrimitive;
/// use rust_persian_tools::bill::BillType;
/// assert_eq!(FromPrimitive::from_u8(2), Some(BillType::Electricity));
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BillType {
    /// آب  
    /// Service Type Code: 1
    Water = 1,
    /// برق  
    /// Service Type Code: 2
    Electricity = 2,
    /// گاز  
    /// Service Type Code: 3
    Gas = 3,
    /// تلفن ثابت  
    /// Service Type Code: 4
    Tel = 4,
    /// تلفن همراه  
    /// Service Type Code: 5
    Mobile = 5,
    /// شهرداری  
    /// Service Type Code: 6
    Municipality = 6,
    /// سازمان مالیات  
    /// Service Type Code: 7
    Tax = 7,
    /// جریمه راهنمایی و رانندگی  
    /// Service Type Code: 8
    DrivingOffense = 8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CurrencyType {
    Rials,
    Tomans,
}

/// Bill ID Structure based on مستندات طرح هماهنگ پرداخت الکترونیکی قبوض - کمیسیون انفورماتیک بانک‌ها \
///
/// | File ID<br/> Maximum Length: 8 | Company Code<br/>  Length: 3 | Service Type<br/>  Length: 1 | Checksum<br/>  Length: 1 |  
/// |:-------------------------:|-------------------------|-------------------------|----------|  
///
/// Checksum is calculated via [ISSN Modulo 11 check digit](https://www.activebarcode.com/codes/checkdigit/modulo11)
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BillID {
    /// Maximum 8-digit Company Internal File ID
    pub file_id: String,
    /// 3-digit Company Code
    pub company_code: String,
    /// Service Type
    pub r#type: BillType,
    pub checksum: u8,
}
impl FromStr for BillID {
    type Err = BillError;
    /// Loads a Bill ID from string representation \
    /// String must be less than or equal size of 13 chars and more than or equal size of 6 chars \
    /// Returns `Err(Bill::BillError)` on failure
    fn from_str(s: &str) -> Result<BillID, Self::Err> {
        if s.len() < 6 || s.len() > 13 {
            return Err(BillError::InvalidBillIDLength);
        }
        let checksum = s
            .chars()
            .nth_back(0)
            .ok_or(Self::Err::InvalidBillChecksum)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillChecksum)? as u8;

        let calculated_checksum = base11_checksum(&s[..s.len() - 1])?;
        if calculated_checksum != checksum {
            return Err(BillError::InvalidBillChecksum);
        }
        let r#type = s
            .chars()
            .nth_back(1)
            .ok_or(Self::Err::InvalidBillChecksum)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillChecksum)? as u8;
        let r#type: BillType = FromPrimitive::from_u8(r#type).ok_or(Self::Err::InvalidBillType)?;

        let company_code = String::from(&s[s.len() - 5..s.len() - 2]);
        let file_id = String::from(&s[..s.len() - 5]);
        Ok(BillID {
            file_id,
            company_code,
            r#type,
            checksum,
        })
    }
}

impl BillID {
    /// Builds Bill ID from scratch and calculates checksum automatically
    pub fn new(file_id: &str, company_code: &str, r#type: BillType) -> Result<Self, BillError> {
        let s = format!("{}{:03}{}", file_id, company_code, r#type as u8);
        let checksum = base11_checksum(&s)?;
        Ok(BillID {
            file_id: String::from(file_id),
            company_code: String::from(company_code),
            r#type,
            checksum,
        })
    }
}
impl ToString for BillID {
    /// Returns String representation of Bill ID
    fn to_string(&self) -> String {
        format!(
            "{}{:03}{:1}{:1}",
            self.file_id, self.company_code, self.r#type as u8, self.checksum
        )
    }
}

/// Payment Structure based on مستندات طرح هماهنگ پرداخت الکترونیکی قبوض - کمیسیون انفورماتیک بانک‌ها \
///
/// | Amount in thousands<br/> Maximum Length: 8 | Year Code<br/>  Length: 1 | Period Code<br/>  Length: 2 | Checksum1<br/>  Length: 1 | Checksum2<br/>  Length: 1 |    
/// |:-------------------------:|-------------------------|-------------------------|----------|----------|  
///
/// Checksums are calculated via [ISSN Modulo 11 check digit](https://www.activebarcode.com/codes/checkdigit/modulo11)  \
/// Checksum1 is the checksum for Payment ID itself and only checks digits in Payment ID  \
/// Checksum2 is the checksum for Bill ID and Payment ID concatenated together and checks validity of relation between two IDs
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct PaymentID {
    /// Amount in scale 1000:1 (1000 will be 1)
    amount: u64,
    /// Company Internal code that represents bill year (Normally last digit of the year)
    year: u8,
    /// Company Internal payment period (Normally sequential increasing number)
    period: u8,
    checksum1: u8,
    checksum2: u8,
}
impl std::str::FromStr for PaymentID {
    type Err = BillError;

    /// Loads a Payment ID from string representation \
    /// String must be less than or equal size of 13 chars and more than or equal size of 6 chars \
    /// Returns `Err(Bill::BillError)` on failure
    /// Note: Only checksum1 is validated here
    fn from_str(s: &str) -> Result<PaymentID, Self::Err> {
        if s.len() < 6 || s.len() > 13 {
            return Err(BillError::InvalidPaymentIDLength);
        }
        let checksum1 = s
            .chars()
            .nth_back(1)
            .ok_or(Self::Err::InvalidBillIDYear)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillChecksum)? as u8;
        let calculated_checksum = base11_checksum(&s[..s.len() - 2])?;
        if calculated_checksum != checksum1 {
            return Err(BillError::InvalidBillChecksum);
        }

        let amount = (s[..s.len() - 5])
            .parse::<u64>()
            .map_err(|_| Self::Err::InvalidBillIDAmount)?;
        let year = s
            .chars()
            .nth_back(4)
            .ok_or(Self::Err::InvalidBillIDYear)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillIDYear)? as u8;

        let period = (s[s.len() - 4..s.len() - 2])
            .parse::<u8>()
            .map_err(|_| Self::Err::InvalidBillIDPeriod)?;
        let checksum2 = s
            .chars()
            .nth_back(0)
            .ok_or(Self::Err::InvalidBillIDYear)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillChecksum)? as u8;

        Ok(PaymentID {
            amount,
            year,
            period,
            checksum1,
            checksum2,
        })
    }
}

impl ToString for PaymentID {
    /// Returns String representation of Bill ID
    fn to_string(&self) -> String {
        format!(
            "{}{}{:02}{}{}",
            self.amount, self.year, self.period, self.checksum1, self.checksum2
        )
    }
}

impl PaymentID {
    /// Builds Bill ID from scratch and calculates checksum automatically
    pub fn new(amount: u64, year: u8, period: u8, bill_id: &BillID) -> Result<Self, BillError> {
        let s = format!("{}{}{:02}", amount, year, period);
        let checksum1 = base11_checksum(&s)?;
        let s = format!("{}{}{}", bill_id.to_string(), s, checksum1);
        let checksum2 = base11_checksum(&s)?;
        Ok(PaymentID {
            amount,
            year,
            period,
            checksum1,
            checksum2,
        })
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }
    pub fn get_year(&self) -> u8 {
        self.year
    }
    pub fn get_period(&self) -> u8 {
        self.period
    }
    pub fn get_checksum1(&self) -> u8 {
        self.checksum1
    }
    pub fn get_checksum2(&self) -> u8 {
        self.checksum2
    }
}

/// Container for Both Bill and Payment IDs  \
/// You must use this type to extract all information about the bill  
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Bill {
    pub bill_id: BillID,
    pub payment_id: PaymentID,
}

impl std::str::FromStr for Bill {
    type Err = BillError;
    /// Loads Bill ID and Payment ID form barcode  \
    /// Barcode format is: \[Bill ID\]\[Payment ID\]  \
    /// Barcode Must be exactly 26 chars  \
    /// This also checks validity of the relation between Bill and Payment IDs (checksum2)
    fn from_str(barcode: &str) -> std::result::Result<Bill, BillError> {
        if barcode.len() != 26 {
            return Err(BillError::InvalidBarcodeLength);
        }
        let bill_id = BillID::from_str(&barcode[..13])?;
        let payment_id = PaymentID::from_str(&barcode[16..])?;
        let bill = Bill::new(bill_id, payment_id)?;
        Ok(bill)
    }
}

impl Bill {
    fn validate(&self) -> Result<(), BillError> {
        let mut merged = self.bill_id.to_string();
        merged.push_str(&self.payment_id.to_string());
        let calculated_checksum = base11_checksum(&merged[..&merged.len() - 1])?;
        if calculated_checksum != self.payment_id.checksum2 {
            return Err(BillError::InvalidBillChecksum);
        }
        Ok(())
    }
    pub fn new(bill_id: BillID, payment_id: PaymentID) -> Result<Self, BillError> {
        let bill = Bill {
            bill_id,
            payment_id,
        };
        bill.validate()?;
        Ok(bill)
    }
    pub fn amount(&self, currency: CurrencyType) -> u64 {
        match currency {
            CurrencyType::Rials => self.payment_id.amount * 1000,
            CurrencyType::Tomans => self.payment_id.amount * 100,
        }
    }
    pub fn get_bill_type(&self) -> BillType {
        self.bill_id.r#type
    }

    pub fn get_bill_id(&self) -> String {
        self.bill_id.to_string()
    }

    pub fn get_payment_id(&self) -> String {
        self.payment_id.to_string()
    }
}

impl ToString for Bill {
    /// Generates Barcode from Bill struct
    fn to_string(&self) -> String {
        format!(
            "{:0>13}{:0>13}",
            self.bill_id.to_string(),
            self.payment_id.to_string()
        )
    }
}
fn base11_checksum(s: &str) -> Result<u8, BillError> {
    let mut sum: u32 =
        s.chars()
            .rev()
            .enumerate()
            .try_fold(0u32, |acc, (i, v)| -> Result<u32, BillError> {
                let multiplier = i as u32 % 6 + 2;
                let digit = v.to_digit(10).ok_or(BillError::InvalidDigits)?;
                Ok(acc + digit * multiplier)
            })?;
    sum %= 11;
    if sum < 2 {
        Ok(0u8)
    } else {
        Ok(11 - sum as u8)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::bill::{Bill, BillID, BillType, CurrencyType, PaymentID};

    #[test]
    fn bill_load_from_barcode_test() {
        let barcode = "11177532001400000012070160";
        let bill = Bill::from_str(barcode);
        assert!(bill.is_ok());
        let bill = bill.unwrap();
        assert_eq!(bill.amount(CurrencyType::Tomans), 12000);
        assert_eq!(bill.amount(CurrencyType::Rials), 120000);
    }

    #[test]
    fn bill_load_from_ids_test() {
        let bill_id = BillID::from_str("1117753200140");
        let payment_id = PaymentID::from_str("12070160");
        assert!(bill_id.is_ok());
        assert!(payment_id.is_ok());

        let bill_id = bill_id.unwrap();
        let payment_id = payment_id.unwrap();
        let bill = Bill::new(bill_id, payment_id);
        assert!(bill.is_ok());

        let bill = bill.unwrap();
        assert_eq!(bill.amount(CurrencyType::Tomans), 12000);
        assert_eq!(bill.amount(CurrencyType::Rials), 120000);

        assert_eq!(
            Bill::new(
                BillID::from_str("1177809000142").unwrap(),
                PaymentID::from_str("570108").unwrap()
            )
            .unwrap()
            .amount(CurrencyType::Rials),
            5000,
        );
        assert_eq!(
            Bill::new(
                BillID::from_str("1177809000142").unwrap(),
                PaymentID::from_str("570108").unwrap()
            )
            .unwrap()
            .amount(CurrencyType::Tomans),
            500,
        );

        // The Payment ID from typescript version seems to be wrong
        // https://github.com/persian-tools/persian-tools/blob/f47fc0261aa5680cde6e03b87905a1273c91de92/test/bill.spec.ts#L14
        // Tested on AsanPardakht and Sadad to confirm
        // Last digit of payment id is checksum of bill and payment id together
        // with values 1117753200140 for bill id and 177016 for payment id the checksum will be 3
        // so payment id must be 1770163 to be valid
        assert_eq!(
            Bill::new(
                BillID::from_str("1117753200140").unwrap(),
                PaymentID::from_str("1770163").unwrap()
            )
            .unwrap()
            .amount(CurrencyType::Rials),
            17000,
        );
        assert_eq!(
            Bill::new(
                BillID::from_str("1117753200140").unwrap(),
                PaymentID::from_str("1770163").unwrap()
            )
            .unwrap()
            .amount(CurrencyType::Tomans),
            1700,
        );
    }

    #[test]
    fn bill_instantiate_test() {
        let bill_id = BillID::new("11177532", "001", BillType::Tel).unwrap();
        let payment_id = PaymentID::new(120, 7, 1, &bill_id).unwrap();
        let bill = Bill {
            bill_id,
            payment_id,
        };

        assert_eq!(bill.bill_id.to_string(), "1117753200140");
        assert_eq!(bill.payment_id.to_string(), "12070160");
    }

    #[test]
    fn bill_bill_type_test() {
        assert_eq!(
            Bill::new(
                BillID::from_str("7748317800142").unwrap(),
                PaymentID::from_str("1770160").unwrap()
            )
            .unwrap()
            .get_bill_type(),
            BillType::Tel
        );
        // The Payment ID from typescript version seems to be wrong
        // https://github.com/persian-tools/persian-tools/blob/f47fc0261aa5680cde6e03b87905a1273c91de92/test/bill.spec.ts#L14
        // Tested on AsanPardakht and Sadad to confirm
        // Last digit of payment id is checksum of bill and payment id together
        // with values 9174639504124 for bill id and 1290819 for payment id the checksum will be 0
        // so payment id must be 12908190 to be valid
        assert_eq!(
            Bill::new(
                BillID::from_str("9174639504124").unwrap(),
                PaymentID::from_str("12908190").unwrap()
            )
            .unwrap()
            .get_bill_type(),
            BillType::Electricity
        );
        assert_eq!(
            Bill::new(
                BillID::from_str("2050327604613").unwrap(),
                PaymentID::from_str("1070189").unwrap()
            )
            .unwrap()
            .get_bill_type(),
            BillType::Water
        );
        // The Bill ID from typescript version seems to be wrong
        // https://github.com/persian-tools/persian-tools/blob/f47fc0261aa5680cde6e03b87905a1273c91de92/test/bill.spec.ts#L36
        // Bill ID 9100074409151 is invalid! checksum must be 3 so valid Bill ID is 9100074409153
        // So Correct Payment ID should also be changed to 12908199 in turn
        assert_eq!(
            Bill::new(
                BillID::from_str("9100074409153").unwrap(),
                PaymentID::from_str("12908199").unwrap()
            )
            .unwrap()
            .get_bill_type(),
            BillType::Mobile
        );
    }

    #[test]
    fn bill_bill_id_is_valid() {
        assert!(BillID::from_str("7748317800142").is_ok());
        assert!(BillID::from_str("9174639504124").is_ok());
        assert!(BillID::from_str("2050327604613").is_ok());
        assert!(BillID::from_str("2234322344613").is_err());
    }

    #[test]
    fn bill_payment_id_is_valid() {
        // Check first checksum (payment id verification)
        assert!(PaymentID::from_str("1770160").is_ok());
        assert!(PaymentID::from_str("12908197").is_ok());
        assert!(PaymentID::from_str("1070189").is_ok());
        assert!(PaymentID::from_str("1070189").is_ok());

        assert!(PaymentID::from_str("1770150").is_err());
        assert!(PaymentID::from_str("12908117").is_err());
        assert!(PaymentID::from_str("1070179").is_err());
        assert!(PaymentID::from_str("1070169").is_err());

        // Check second checksum (Full bill and payment IDs validation)
        assert!(Bill::new(
            BillID::from_str("7748317800142").unwrap(),
            PaymentID::from_str("1770160").unwrap()
        )
        .is_ok());
        assert!(Bill::new(
            BillID::from_str("9174639504124").unwrap(),
            PaymentID::from_str("12908197").unwrap()
        )
        .is_err());
        assert!(Bill::new(
            BillID::from_str("2050327604613").unwrap(),
            PaymentID::from_str("1070189").unwrap()
        )
        .is_ok());
        // This test is supposed to test Payment ID so the Bill ID should be valid
        // Bill id 2234322344613 from typescript version
        // https://github.com/persian-tools/persian-tools/blob/f47fc0261aa5680cde6e03b87905a1273c91de92/test/bill.spec.ts#L79
        // is not valid, so it is changed to 2234322344617 to be valid so only Payment ID is checked
        assert!(Bill::new(
            BillID::from_str("2234322344617").unwrap(),
            PaymentID::from_str("1070189").unwrap()
        )
        .is_err());
    }

    #[test]
    fn bill_generate_barcode_test() {
        // Based on the https://core.pod.ir/nzh/file/?fileId=334917&hashCode=16e5a9ee893-0.22579290613133673
        // Barcode Musts be exactly 26 chars
        // bill and payment IDs must be left padded by zero to fill 13 chars
        // So the values from https://github.com/persian-tools/persian-tools/blob/f47fc0261aa5680cde6e03b87905a1273c91de92/test/bill.spec.ts#L89
        // Are not correct
        assert_eq!(
            Bill::new(
                BillID::from_str("7748317800142").unwrap(),
                PaymentID::from_str("1770160").unwrap()
            )
            .unwrap()
            .to_string(),
            "77483178001420000001770160"
        );
        // 12908197 from typescript version must be 12908190
        assert_eq!(
            Bill::new(
                BillID::from_str("9174639504124").unwrap(),
                PaymentID::from_str("12908190").unwrap()
            )
            .unwrap()
            .to_string(),
            "91746395041240000012908190"
        );
        assert_eq!(
            Bill::new(
                BillID::from_str("2050327604613").unwrap(),
                PaymentID::from_str("1070189").unwrap()
            )
            .unwrap()
            .to_string(),
            "20503276046130000001070189"
        );
        // Bill ID 2234322344613 from typescript version is not correct
        // Correct Bill ID is 2234322344617
        assert_eq!(
            Bill::new(
                BillID::from_str("2234322344617").unwrap(),
                PaymentID::from_str("1070188").unwrap()
            )
            .unwrap()
            .to_string(),
            "22343223446170000001070188"
        );
    }

    #[test]
    fn bill_load_from_barcode_parts_test() {
        // Bill ID 2234322344613 from typescript version is incorrect, correct Bill ID is 2234322344617
        // So correct payment ID will be 1070188
        let bill = Bill::from_str("22343223446170000001070188").unwrap();

        assert_eq!(bill.get_bill_id(), "2234322344617");
        assert_eq!(bill.get_payment_id(), "1070188");
    }

    #[test]
    fn bill_barcode_regeneration_test() {
        let barcode = "33009590043100000385620969";
        let bill = Bill::from_str(barcode).unwrap();
        assert_eq!(bill.to_string(), barcode);
    }
}
