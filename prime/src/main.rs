
fn is_prime(n: i32) -> bool {
    if n==1 || n ==0  { 
        return false;
    }
   
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    
    return true;
}

fn main() {
    //declare very inefficient isprime function
    let n = 100;
    for i in 1..n {
        if is_prime(i) {
            println!("{i}");
        }
    }
}
