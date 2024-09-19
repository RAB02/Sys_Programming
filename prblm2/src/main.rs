fn main(){
    let array: [i32; 10] = [12, 42, 34, 51, 120, 51, 73, 30, 59, 81];

    let mut sum = 0;
    let mut i = 0;

    while i < array.len() {
        sum += array[i];
        i += 1;
    }
    println!("The Sum is {}", sum);

    for &number in array.iter(){
        if is_even(number){
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }

        if number % 3 == 0 && number % 5 == 0{
            println!("FizzBuzz")
        } else if number % 3 == 0 {
            println!("Fizz")
        } else if number % 5 == 0{
            println!("Buzz")
        }

    }

    let mut large = array[0];
    for &num in array.iter(){
        if num > large {
            large = num;
        }
    }
    
    println!("Largest Number: {}", large);
}

fn is_even(n: i32) -> bool{
    if n % 2 == 0{
        return true;
    }else 
        {return false;}
}