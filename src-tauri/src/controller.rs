use tauri::Manager;
use serde::Serialize;

use phonenumber::country;
use phonenumber::Mode;

#[derive(Debug, Serialize, Clone)]
pub struct Number {
    pub id: Option<i32>,
    pub digits: String,
    pub is_valid: bool,
    pub international: Option<String>,
    pub national: Option<String>,
    pub rfc3966: Option<String>,
    pub e164: Option<String>,
    pub email: Option<String>,
    pub carrier: Option<String>,
}

#[tauri::command]
pub fn parse(text: &str, app_handle: tauri::AppHandle) {
    println!("parse {:?}", text);
    let numbers = extract_numbers(text);
    println!("numbers {:?}", numbers);
    app_handle.emit_all("numbers::extracted", numbers).unwrap();
}

use regex::Regex;
fn extract_numbers(text: &str) -> Vec<Number> {
    // Define a regex pattern that captures various phone number formats
    let phone_pattern = r"(?x)
        (\+1[\s.-]?)?                   # Optional country code +1
        \(?(\d{3})\)?[\s.-]?             # Area code (3 digits)
        (\d{3})[\s.-]?                   # First 3 digits
        (\d{4})                          # Last 4 digits
    ";

    // Compile the regex pattern
    let re = Regex::new(phone_pattern).unwrap();

    // Extract matches
    let phone_numbers: Vec<Number> = re
        .captures_iter(text)
        .map(|cap| {
            let mut formatted = String::new();
            // if let Some(country) = cap.get(1) {
            //     formatted.push_str(country.as_str());
            // }
            formatted.push_str(cap.get(2).unwrap().as_str());
            formatted.push('-');
            formatted.push_str(cap.get(3).unwrap().as_str());
            formatted.push('-');
            formatted.push_str(cap.get(4).unwrap().as_str());

            let digits = formatted.clone();
            let number = phonenumber::parse(Some(country::US), formatted).unwrap();
            let is_valid = phonenumber::is_valid(&number);

            Number {
                id: None,
                digits,
                is_valid,
                international: Some(String::from(format!(
                    "{}",
                    number.format().mode(Mode::International)
                ))),
                national: Some(String::from(format!(
                    "{}",
                    number.format().mode(Mode::National)
                ))),
                rfc3966: Some(String::from(format!(
                    "{}",
                    number.format().mode(Mode::Rfc3966)
                ))),
                e164: Some(String::from(format!(
                    "{}",
                    number.format().mode(Mode::E164)
                ))),
                email: None,
                carrier: None,
            }
        })
        .collect();

    phone_numbers
}
