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

fn main() {
    println!("Hello, world!");
}
