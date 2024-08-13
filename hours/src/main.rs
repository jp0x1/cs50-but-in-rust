use std::io;

fn calculate_average(arr: &mut Vec<f32>) -> f32 {
    return calculate_total(arr) / arr.len() as f32;
}

fn calculate_total(arr: &mut Vec<f32>) -> f32{
    let mut sum: f32 = 0.0;
    for i in 0.. arr.len() {
        sum += arr[i]
    }
    return sum;
}


fn main() {
    println!("Number of weeks taking CS50");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_of_weeks: usize = input.trim().parse().expect("Input not an  integer");
    //must use vectors because array lengths are constant in rust
    let mut week_hw_hours: Vec<f32> = vec![0.0; num_of_weeks];

    //iterate to collect hw hours for week
    for i in 0..num_of_weeks {
        println!("Enter HW hours for week {i}");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num_of_hours: f32 = input.trim().parse().expect("Input not a float");
        week_hw_hours[i] = num_of_hours;
    }

    //prompt for total or avaerge!
    println!("Enter T for total hours and A for average hours");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input == "T" {
        let total = calculate_total(&mut week_hw_hours);
        println!("The total is {:.1} hours", total);
    } 

    else if input == "A" {
        let average = calculate_average(&mut week_hw_hours);
        println!("The average is {:.1} hours", average);
    } 

    else {
        println!("Invalid input.");
    }
    
}
