use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;

#[derive(Debug, Serialize)]
pub struct MonthlyResult {
    month: u32,
    portfolio_value: f64,
    equity: f64,
    maintenance_margin_requirement: f64,
    margin_call: bool,
}

pub fn margin_calculator(
    p_initial: f64,                     // Initial portfolio value
    loaned_amount: f64,                 // Loaned margin amount
    annual_interest_rate: f64,          // Annual interest rate on margin
    maintenance_margin_percentage: f64, // Maintenance margin percentage
    monthly_portfolio_change_rate: f64, // Monthly portfolio change rate
    months: u32,                        // Number of months
) -> Vec<MonthlyResult> {
    let monthly_interest_rate = annual_interest_rate / 12.0; // Convert annual rate to monthly rate

    let mut results = Vec::new(); // Initialize the vector to store results

    // Current portfolio value mutable
    let mut p_current = p_initial;

    for month in 1..=months {
        let monthly_interest = loaned_amount * monthly_interest_rate; // Monthly interest on loaned amount

        let p_new = p_current * (1.0 + monthly_portfolio_change_rate); // Update portfolio value with new portfolio amount

        let maintenance_margin_requirement = maintenance_margin_percentage * p_new; // Calculate maintenance margin requirement

        /*
        Get equity and check for a margin call. Store the results
         */
        let equity = p_new - loaned_amount - monthly_interest;

        let margin_call = equity < maintenance_margin_requirement; // Determine if a margin call is needed

        // Store the results in the vector
        results.push(MonthlyResult {
            month,
            portfolio_value: p_new,
            equity,
            maintenance_margin_requirement,
            margin_call,
        });

        // Update for next iteration
        p_current = p_new;
    }

    results // Return the results vector
}

fn main() {
    // Define the input values
    let initial_portfolio_value: f64 = 4000.0; // Total portfolio value including margin amount
    let loaned_amount: f64 = 2000.0; // Loaned margin amount
    let annual_interest_rate: f64 = 0.0725; // This represents an annual rate of 7.25%
    let maintenance_margin_percentage: f64 = 0.25; // 25% Margin maintenance requirement
    let monthly_change_rate: f64 = 0.02; // Portfolio monthly change rate %. 2% monthly used here.
    let months: u32 = 12; // Number of months

    // Perform margin calculation
    let results = margin_calculator(
        initial_portfolio_value,
        loaned_amount,
        annual_interest_rate,
        maintenance_margin_percentage,
        monthly_change_rate,
        months,
    );

    let path = "margin_calculator_results.csv";
    // Output results to a CSV file
    let csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to create CSV file!!");
    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_writer(csv_file);

    // Write it out to header rows so the CSV looks decent

    wtr.write_record(&[
        "Month",
        "Portfolio Value",
        "Equity",
        "Maintenance Margin Requirement",
        "Margin Call",
    ])
    .expect("Failed to write header rows!!");

    for result in results {
        wtr.serialize(result)
            .expect("Failed to write result to CSV");
    }
    wtr.flush()
        .expect("Flush failed! Could not ensure data integrity");

    println!("Written to margin_calculator_results.csv");
}
