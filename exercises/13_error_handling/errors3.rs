// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;
// use std::process;
// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main() -> Result<(), ParseIntError> { //CustomExit {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
        // CustomExit::Error(1)
        Ok(())
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
        // CustomExit::Success
        Ok(())
    }
}

// enum CustomExit {
//     Success,
//     Error(i32),
// }

// impl process::Termination for CustomExit {
//     fn report(self) -> process::ExitCode {
//         match self {
//             CustomExit::Success => process::ExitCode::SUCCESS,
//             CustomExit::Error(code) => process::ExitCode::from(code)
//         }
//     }
// }
