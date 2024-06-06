# Margin Calculator

A simple Margin Calculator that outputs a CSV/Excel format when ran. This is meant to be used in a later, larger codebase of finance + market tools written using Rust. Perhaps this may give someone else some inspo to do something finance or market related in Rust.

To learn more about what Margin is and why it's used, go and visit our friends at [Investopedia](https://www.investopedia.com/terms/m/margin.asp) 


Outputs as an Excel/Google Sheets friendly format.

## Inputs

- initial_portfolio-value  // The starting value of your portfolio; with margin.
- loaned_amount           // Loaned margin amount
- annual_interest_rate  // Annual interest rate of margin
- maintenance_margin_percentage // Margin maint. %. Differs on each broker, I used 25% in my examplea
- monthly change rate // Rate of monthly change %. Have to account for this. Because you know what they say about working stocks - it has its ups and downs!
- months // Number of months


## Used Example Input Values

- initial_portfolio_value = 4000.0 // Total portfolio value including margin amount
- loaned_amount = 2000.0 // Loaned margin amount
- annual_interest_rate  = 0.0725 // This represents an annual rate of 7.25%
- maintenance_margin_percentage = 0.25 // 25% Margin maintenance requirement
- monthly_change_rate = 0.02 // Portfolio monthly change rate %. 2% monthly used here.
- months = 12 // Number of months


## Spreadsheet 
An example of the generated spreadsheet can be found in this repo *margin_calculator_results.csv*


## Raw output
````
MonthlyResult { month: 1, portfolio_value: 3200.0, equity: 1187.9166666666667, maintenance_margin_requirement: 800.0, margin_call: false }
MonthlyResult { month: 2, portfolio_value: 2560.0, equity: 547.9166666666666, maintenance_margin_requirement: 640.0, margin_call: true }
MonthlyResult { month: 3, portfolio_value: 2048.0, equity: 35.916666666666664, maintenance_margin_requirement: 512.0, margin_call: true }
MonthlyResult { month: 4, portfolio_value: 1638.4, equity: -373.6833333333332, maintenance_margin_requirement: 409.6, margin_call: true }
MonthlyResult { month: 5, portfolio_value: 1310.7200000000003, equity: -701.3633333333331, maintenance_margin_requirement: 327.68000000000006, margin_call: true }
MonthlyResult { month: 6, portfolio_value: 1048.5760000000002, equity: -963.5073333333331, maintenance_margin_requirement: 262.14400000000006, margin_call: true }
MonthlyResult { month: 7, portfolio_value: 838.8608000000003, equity: -1173.2225333333329, maintenance_margin_requirement: 209.71520000000007, margin_call: true }
MonthlyResult { month: 8, portfolio_value: 671.0886400000003, equity: -1340.9946933333329, maintenance_margin_requirement: 167.77216000000007, margin_call: true }
MonthlyResult { month: 9, portfolio_value: 536.8709120000002, equity: -1475.212421333333, maintenance_margin_requirement: 134.21772800000005, margin_call: true }
MonthlyResult { month: 10, portfolio_value: 429.4967296000002, equity: -1582.586603733333, maintenance_margin_requirement: 107.37418240000005, margin_call: true }
MonthlyResult { month: 11, portfolio_value: 343.5973836800002, equity: -1668.4859496533331, maintenance_margin_requirement: 85.89934592000004, margin_call: true }
MonthlyResult { month: 12, portfolio_value: 274.87790694400013, equity: -1737.205426389333, maintenance_margin_requirement: 68.71947673600003, margin_call: true }
```