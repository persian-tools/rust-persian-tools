use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::From;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone)]
pub enum BillError {
    InvalidBarcodeLength,
    InvalidPaymentID,
    InvalidBillTypeCode,
    InvalidBillIDAmount,
    InvalidBillIDYear,
    InvalidBillIDPeriod,
    InvalidBillIDChecksum,
    InvalidBillType,
    InvalidDigits,
}

impl std::fmt::Display for BillError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidBarcodeLength => write!(f, "Barcode Length must be 26 characters"),
            Self::InvalidPaymentID => write!(f, "PaymentID is not valid"),
            Self::InvalidBillTypeCode => write!(f, "Bill Type is not valid"),
            Self::InvalidBillIDAmount => write!(f, "Bill Amount is not valid"),
            Self::InvalidBillIDYear => write!(f, "Bill Year is not valid"),
            Self::InvalidBillIDPeriod => write!(f, "Bill Period is not valid"),
            Self::InvalidBillIDChecksum => write!(f, "Bill Checksum is not valid"),
            Self::InvalidBillType => write!(f, "Bill Type is not valid"),
            Self::InvalidDigits => write!(f, "Digits Cannot be parsed"),
        }
    }
}

#[derive(FromPrimitive, Clone, Copy, PartialEq, Debug)]
pub enum BillType {
    Water = 1,
    Electricity = 2,
    Gas = 3,
    Tel = 4,
    Mobile = 5,
    Municipality = 6,
    Tax = 7,
    DrivingOffence = 8,
}

#[derive(Debug, PartialEq)]
pub enum CurrencyType {
    Rials,
    Tomans,
}
#[derive(Debug, PartialEq)]
pub struct BillID {
    pub file_id: String,
    pub company_code: String,
    pub r#type: BillType,
    pub checksum: u8,
}
impl FromStr for BillID {
    type Err = BillError;

    fn from_str(s: &str) -> Result<BillID, Self::Err> {
        let checksum = s
            .chars()
            .nth_back(0)
            .ok_or(Self::Err::InvalidBillIDChecksum)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillIDChecksum)? as u8;

        if base11_checksum(&s[..s.len() - 1])? != checksum {
            return Err(BillError::InvalidBillIDChecksum);
        }
        let r#type = s
            .chars()
            .nth_back(1)
            .ok_or(Self::Err::InvalidBillIDChecksum)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillIDChecksum)? as u8;
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
    fn to_string(&self) -> String {
        format!(
            "{}{:03}{:1}{:1}",
            self.file_id, self.company_code, self.r#type as u8, self.checksum
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct PaymentID {
    amount: u64,
    year: u8,
    period: u8,
    checksum1: u8,
    checksum2: u8,
}
impl std::str::FromStr for PaymentID {
    type Err = BillError;

    fn from_str(s: &str) -> Result<PaymentID, Self::Err> {
        let checksum1 = s
            .chars()
            .nth_back(1)
            .ok_or(Self::Err::InvalidBillIDYear)?
            .to_digit(10)
            .ok_or(Self::Err::InvalidBillIDChecksum)? as u8;
        if base11_checksum(&s[..s.len() - 2])? != checksum1 {
            return Err(BillError::InvalidBillIDChecksum);
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
            .ok_or(Self::Err::InvalidBillIDChecksum)? as u8;

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
    fn to_string(&self) -> String {
        format!(
            "{}{}{:02}{}{}",
            self.amount, self.year, self.period, self.checksum1, self.checksum2
        )
    }
}

impl PaymentID {
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
}

#[derive(Debug, PartialEq)]
pub struct Bill {
    pub bill_id: BillID,
    pub payment_id: PaymentID,
}

impl std::str::FromStr for Bill {
    type Err = BillError;
    // [             Bill ID (6-13 chars)            ][          Payment ID (6-13 chars)             ]
    // [ file ID ][ company code ][ type ][ checksum ][ amount ][ year ][period][checksum1][checksum2]
    //    max 8         3            1         1         max 8     1       2        1          1
    fn from_str(barcode: &str) -> std::result::Result<Bill, BillError> {
        if barcode.len() != 26 {
            return Err(BillError::InvalidBarcodeLength);
        }
        let bill_id = BillID::from_str(&barcode[..13])?;

        let payment_id = PaymentID::from_str(&barcode[16..])?;

        let mut merged = bill_id.to_string();
        merged.push_str(&payment_id.to_string());

        if base11_checksum(&merged[..&merged.len() - 1])? != payment_id.checksum2 {
            return Err(BillError::InvalidBillIDChecksum);
        }

        Ok(Bill {
            bill_id,
            payment_id,
        })
    }
}
impl Bill {
    pub fn amount(&self, currency: CurrencyType) -> u64 {
        match currency {
            CurrencyType::Rials => self.payment_id.amount * 1000,
            CurrencyType::Tomans => self.payment_id.amount * 100,
        }
    }
}

impl ToString for Bill {
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
        let barcode = "33009590043100000385620969";
        let bill = Bill::from_str(barcode);
        assert!(bill.is_ok());
        let bill = bill.unwrap();
        assert_eq!(bill.amount(CurrencyType::Tomans), 385600);
        assert_eq!(bill.amount(CurrencyType::Rials), 3856000);
    }

    #[test]
    fn bill_instantiate_test() {
        let bill_id = BillID::new("33009590", "043", BillType::Water).unwrap();
        let payment_id = PaymentID::new(3856, 2, 9, &bill_id).unwrap();
        let bill = Bill {
            bill_id,
            payment_id,
        };

        assert_eq!(bill.bill_id.to_string(), "3300959004310");
        assert_eq!(bill.payment_id.to_string(), "385620969");
    }

    #[test]
    fn bill_barcode_test() {
        let barcode = "33009590043100000385620969";
        let bill = Bill::from_str(barcode).unwrap();
        assert_eq!(bill.to_string(), barcode);
    }
}
