// uses external library (crate) - must have a entry in the cargo.toml file
extern crate chrono;

pub fn run() {
    
    // gets the current time from chrono, formats it down to the year
    // the year is coverted to a string and then parsed as an integer
    let year: i16 = chrono::Utc::now().format("%Y").to_string().parse().unwrap();
    
    // calls the current_leap_year and future_leap_years functions
    current_leap_year(year);
    println!(""); // creates a break
    future_leap_years(year);

}

// is given year a leap year? print it out if it is
fn calculate_leap_year(year: i16) {

    // convert year from i16 to f32
    let fyear = year as f32;

    if (year % 100 == 0 && fyear % 400.0 == 0.0) || (year % 100 != 0 && fyear % 4.0 == 0.0) {
        println!("{} is a leap year.", year);
    }

}

// is the given year a leap year? return bool
fn bool_leap_year(year: i16) -> bool {

    // convert year from i16 to f32
    let fyear = year as f32;

    if (year % 100 == 0 && fyear % 400.0 == 0.0) || (year % 100 != 0 && fyear % 4.0 == 0.0) {
        return true;
    } else {
        return false;
    }
}

// is the current year a leap year?
fn current_leap_year(year: i16) {

    if bool_leap_year(year) == true {
        println!("The CURRENT YEAR ({}) is a leap year.", year);
    } else {
        println!("The CURRENT YEAR ({}) is NOT a leap year.", year);
    }

}

// calculate leap years for the next 25 years
fn future_leap_years(year: i16) {

    let mut x: i16 = year;

    while x < (year + 25) {
        calculate_leap_year(x);
        x += 1;
    }
}