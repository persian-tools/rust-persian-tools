use arg_parser::*;
use clap::Parser;

mod arg_parser;
mod constants;
mod wrapper;

fn main() {
    let args = Args::parse();

    match args.function {
        Function::AddCommas => handle(&args, wrapper::add_commas),
        Function::RemoveCommas => handle(&args, wrapper::remove_commas),
        Function::AddOrdinalSuffix => handle(&args, wrapper::add_ordinal_suffix),
        Function::RemoveOrdinalSuffix => handle(&args, wrapper::remove_ordinal_suffix),
        Function::HasArabic => handle(&args, wrapper::has_arabic),
        Function::IsArabic => handle(&args, wrapper::is_arabic),
        Function::ToArabic => handle(&args, wrapper::to_arabic),
        Function::GetBillType => handle(&args, wrapper::get_bill_type),
        Function::GetBillAmount => handle(&args, wrapper::get_bill_amount),
        Function::DigitsFaToEn => handle(&args, wrapper::fa_to_en),
        Function::DigitsEnToFa => handle(&args, wrapper::en_to_fa),
        Function::DigitsEnToAr => handle(&args, wrapper::en_to_ar),
        Function::DigitsArToEn => handle(&args, wrapper::ar_to_en),
        Function::DigitsFaToAr => handle(&args, wrapper::fa_to_ar),
        Function::DigitsArToFa => handle(&args, wrapper::ar_to_fa),
        Function::ExtractCardNumber => handle(&args, wrapper::extract_card_number),
        Function::FindCapitalByProvince => handle(&args, wrapper::find_capital_by_province),
        Function::GetBankNameByCardNumber => handle(&args, wrapper::get_bank_name_by_card_number),
        Function::GetCityByIranNationalId => handle(&args, wrapper::get_city_by_iran_national_id),
        Function::GetProvinceByIranNationalId => {
            handle(&args, wrapper::get_province_by_iran_national_id)
        }
        Function::RemoveHalfSpace => handle(&args, wrapper::remove_half_space),
        Function::AddHalfSpace => handle(&args, wrapper::add_half_space),
        Function::VerifyIranianLegalId => handle(&args, wrapper::verify_iranian_legal_id),
        Function::VerifyIranianNationalId => handle(&args, wrapper::verify_iranian_national_id),
        Function::GetPlateType => handle(&args, wrapper::get_plate_type),
        Function::GetPlateProvince => handle(&args, wrapper::get_plate_province),
        Function::GetPlateCategory => handle(&args, wrapper::get_plate_category),
        Function::NumberToWords => handle(&args, wrapper::number_to_words),
        Function::HasPersian => handle(&args, wrapper::has_persian),
        Function::IsPersian => handle(&args, wrapper::is_persian),
        Function::ToPersianChars => handle(&args, wrapper::to_persian_chars),
        Function::IsPhoneValid => handle(&args, wrapper::is_phone_valid),
        Function::GetOperatorPrefix => handle(&args, wrapper::get_operator_prefix),
        Function::GetPhoneOperator => handle(&args, wrapper::get_phone_operator),
        Function::GetPhoneProvince => handle(&args, wrapper::get_phone_province),
        Function::IsShebaValid => handle(&args, wrapper::is_sheba_valid),
        Function::ShebaToBankName => handle(&args, wrapper::sheba_to_bank_name),
        Function::ShebaToPersianBankName => handle(&args, wrapper::sheba_to_persian_bank_name),
        Function::TimeDiff => handle(&args, wrapper::time_diff),
        Function::UrlFix => handle(&args, wrapper::url_fix),
        Function::VerifyCardNumber => handle(&args, wrapper::verify_card_number),
        Function::WordsToNumber => handle(&args, wrapper::words_to_number),
    }
}

fn handle(args: &Args, exe: fn(&str) -> String) {
    if let Some(x) = &args.input {
        println!("{}", exe(x));
    } else {
        let mut buffer = String::new();
        loop {
            buffer.clear();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("can't read from std");
            buffer = buffer.trim().to_string();
            if buffer.is_empty() {
                break;
            }
            println!("{}", exe(&buffer));
        }
    }
}
