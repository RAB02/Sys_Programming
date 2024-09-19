fn main(){
    let secret = 63;

    let mut guess:i32 = 3;
    let mut attempts = 0;

   while true{

        guess = 2 * guess + 1;
        attempts += 1;

        let x = check_guess(guess,secret);

        if x == 0{
            println!("You did it in this amount of tries: {}", attempts);
            break;
        } else if x == 1{
            println!("Guess {} is too high", guess);
        } else if x == -1 {
            println!("Guess {} is too low", guess);
        }
    }
}

fn check_guess(guess:i32, secret:i32) -> i32 {
    if guess > secret{
        return 1;
    } else if guess < secret {
        return -1;
    }else {
        return 0;
    }
}