use std::io;

fn half(bill: f64, tax_percent: f64, tip_percent: f64) -> f64 {
    //calculate the sale tax
    //calculate the tip needed
    let tax = bill * tax_percent * 0.01;
    let tip = (bill+tax) * tip_percent * 0.01; 
    //sum the total
    let half_sum = (bill + tax + tip) / 2.0;
    //return the half
    return half_sum;
}

fn main() {
    //initialize variables
    //must clean up input
    let mut input = String::new();
    //start code process
    println!("Bill before tax and tip: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let bill: f64 = input.trim().parse().expect("Input not a float");

    let mut input = String::new();
    println!("Sale Tax Percent: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let tax_percent: f64 = input.trim().parse().expect("Input not a float");
    let mut input = String::new();

    println!("Tip Percent: "); 
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let tip_percent: f64 = input.trim().parse().expect("Input not a float");
    //call function to process
    let half_sum = half(bill, tax_percent, tip_percent);
    println!("You both need to pay {:.2}!", half_sum);
}
