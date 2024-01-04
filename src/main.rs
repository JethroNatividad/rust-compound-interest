// Write a program to calculate the compound interest.
// Inputs: starting amount, the number of years to invest, the interest rate, and the number of periods per year to compound.
// Process: principal_amount * pow(1 + (interest_rate/times_compounded_per_year), times_compounded_per_year * invested_years)
// Output: $1500 invested at 4.3% for 6 years compounded 4 times per year is $1938.84.

fn calculate_compound_interest(principal_amount: f64, invested_years: f64, interest_rate_percentage: f64, times_compounded_per_year: f64) -> f64 {
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calculate_compound_interest(){
        assert_eq!(calculate_compound_interest(1500.0, 4.3, 6.0, 4.0), 1938.84)
        assert_eq!(calculate_compound_interest(2000.0, 5.0, 3.0, 12.0), 2304.09);
        assert_eq!(calculate_compound_interest(3000.0, 6.0, 2.5, 2.0), 3612.40);
        assert_eq!(calculate_compound_interest(2500.0, 4.0, 5.0, 1.0), 3164.49);
        assert_eq!(calculate_compound_interest(1800.0, 3.5, 4.0, 3.0), 2113.60);
    }
}

fn main() {
    println!("Hello, world!");
}
