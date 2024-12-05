// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    
    let s = String::from("blue");
    return s;
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
