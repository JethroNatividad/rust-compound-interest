use std::io;
use std::io::Write;

// Write a program to calculate the compound interest.
// Inputs: starting amount, the number of years to invest, the interest rate, and the number of periods per year to compound.
// Process: principal_amount * pow(1 + (interest_rate/times_compounded_per_year), times_compounded_per_year * invested_years)
// Output: $1500 invested at 4.3% for 6 years compounded 4 times per year is $1938.84.

fn round_decimal(number: f64) -> f64 {
    (number * 100.0).round() / 100.0
}

fn calculate_compound_interest(principal_amount: f64, interest_rate_percentage: f64, invested_years: f64, times_compounded_per_year: f64) -> f64 {
    let interest_rate: f64 = interest_rate_percentage / 100.0;
    let compound_interest: f64 = principal_amount * f64::powf(1.0 + (interest_rate / times_compounded_per_year), times_compounded_per_year * invested_years);
    round_decimal(compound_interest)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calculate_compound_interest(){
        assert_eq!(calculate_compound_interest(1500.0, 4.3, 6.0, 4.0), 1938.84);
        assert_eq!(calculate_compound_interest(1000.0, 5.0, 2.0, 12.0), 1104.94);
        assert_eq!(calculate_compound_interest(2000.0, 6.0, 3.0, 1.0), 2382.03);

        // Test for edge cases: zero interest rate
        assert_eq!(calculate_compound_interest(500.0, 0.0, 10.0, 2.0), 500.0);

        // Test for edge cases: zero invested years
        assert_eq!(calculate_compound_interest(800.0, 8.0, 0.0, 4.0), 800.0);

        // Test for edge cases: zero compound times 
        assert_eq!(calculate_compound_interest(800.0, 8.0, 3.0, 0.0), 800.0);

        // Test for large values: high principal amount and interest rate
        assert_eq!(calculate_compound_interest(50000.0, 10.0, 8.0, 12.0), 110908.78);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    let principal_amount: f64 = get_input("What is the principal amount? ");
    let interest_rate_percentage: f64 = get_input("What is the rate %? ");
    let invested_years: f64 = get_input("What is the number of years? ");
    let times_compounded_per_year: f64 = get_input("What is the number of times the interest is compounded per year? ");

    let compound_interest: f64 = calculate_compound_interest(principal_amount, interest_rate_percentage, invested_years, times_compounded_per_year);
    let plural_invested_years: &str = if invested_years > 1.0 { "years" } else { "year" };
    let plural_times_compounded_per_year: &str = if times_compounded_per_year > 1.0 { "times" } else { "time" };

    println!("${} invested at {}% for {} {} compounded {} {} per year is ${}", principal_amount, interest_rate_percentage, invested_years, plural_invested_years, times_compounded_per_year, plural_times_compounded_per_year, compound_interest);
    
}
