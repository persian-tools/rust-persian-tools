# Rust-Persian-Tools

<img src="./logo.png" width="300">

[![GitHub license](https://badgen.net/github/license/persian-tools/rust-persian-tools)](https://github.com/persian-tools/rust-persian-tools/blob/master/LICENSE)

Rust🦀 implementation of [Persian-Tools](https://github.com/persian-tools/persian-tools)

Note: These tools are totally offline (no api calls)

## Useful links

1. Documentation: [docs.rs](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/)
2. Installation: [crates.io](https://crates.io/crates/rust-persian-tools)
3. Source code: [github.com](https://github.com/persian-tools/rust-persian-tools/tree/master/src)

## Table of features

| name                          | docs     | description  | 
|-------------------------------|----------|--------------|
| add_ordinal_suffix            | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/add_ordinal_suffix/index.html) | پنج رو به پنجم تبدیل میکنه |
| arabic_chars                  | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/arabic_chars/index.html) | عربی بودن یک متن رو چک میکنه و میتونه بعضی حروف فارسی رو به فرم عربی تبدیل کنه|
| bill                          | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/bill/index.html) | اطلاعات مربوط به قبض |
| commas                        | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/commas/index.html) | 3000-> 3,000 |
| digits                        | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/digits/index.html) | 123 -> ۱۲۳ و برعکس |
| extract_card_number           | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/extract_card_number/index.html) | شماره کارت  رو از متن استخراج میکنه چه فارسی چه انگلیسی |
| find_capital_by_province      | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/find_capital_by_province/index.html) | استان رو به مرکز استان تبدیل میکنه |
| get_bank_name_by_card_number  | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/get_bank_name_by_card_number/index.html) |  شماره کارت میدی بهت اسم بانک برمیگردونه|
| get_place_by_iran_national_id | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/get_place_by_iran_national_id/index.html) | کد ملی میدی بهت شهر و استان برمیگردونه |
| half_space                    | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/half_space/index.html) | نیم فاصله هارو اوکی میکنه |
| legal_id                      | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/legal_id/index.html) | شناسه حقوقی رو اعتبار سنجی میکنه |
| national_id                   | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/national_id/index.html) | کد ملی رو اعتبار سنجی میکنه |
| number_plate                  | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/number_plate/index.html) | پلاک ماشین و موتور |
| number_to_words               | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/number_to_words/index.html) | عدد رو به حروف تبدیل میکنه |
| persian_chars                 | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/persian_chars/index.html) | فارسی بودن یک متن رو چک میکنه و میتونه بعضی حروف عربی رو به فارسی تبدیل کنه |
| phone_number                  | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/phone_number/index.html) | شماره تلفن رو اعتبار سنجی میکنه و اپراتور رو شناسایی میکنه |
| remove_ordinal_suffix         | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/remove_ordinal_suffix/index.html) | پنجم رو به پنج تبدیل میکنه |
| sheba                         | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/sheba/index.html) | شماره شبا رو اعتبار سنجی میکنه و اطلاعات بانک مربوط رو برمیگردونه |
| time_diff                     | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/time_diff/index.html) | برای دو لحظه از زمان یک متن فارسی تولید میکنه که اختلاف دو لحظه رو توصیف میکنه |
| url_fix                       | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/url_fix/index.html) | حروف فارسی رو به فرمتی تبدیل میکنه که در url قابل استفاده باشه |
| verity_card_number            | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/verity_card_number/index.html) | شماره کارت بانکی رو اعتبار سنجی میکنه |
| words_to_number               | [link](https://docs.rs/rust-persian-tools/1.0.0/rust_persian_tools/words_to_number/index.html) | حروف فارسی رو به عدد تبدیل میکنه |

## Need help?

- You don't know how to use it?
    1. Read [docs](https://docs.rs/rust-persian-tools/latest/rust_persian_tools/)
    2. [Read tests](https://github.com/persian-tools/rust-persian-tools/tree/master/src)
    3. Create an [issue](https://github.com/persian-tools/rust-persian-tools/issues)

- If you find a bug or you need a new feature
    1. Make a [issue](https://github.com/persian-tools/rust-persian-tools/issues)

- Need more help?
  - Thats my email: <alighahremani1377@gmail.com>

## How to Help

- Star the project ⭐
- Tweet about it
- Refer this project in your project's readme
- Mention the project at local meetups and tell your friends/colleagues
- Add a module see: [Contributing.md](https://github.com/persian-tools/rust-persian-tools/blob/master/Contributing.md).
- Fix a bug see: [Contributing.md](https://github.com/persian-tools/rust-persian-tools/blob/master/Contributing.md).
- Update database see: [Contributing.md](https://github.com/persian-tools/rust-persian-tools/blob/master/Contributing.md).
