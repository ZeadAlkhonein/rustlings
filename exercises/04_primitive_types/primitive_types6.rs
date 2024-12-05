// fn main() {
//     // You can optionally experiment here. -> (i32, &string)
//     let number = (1, 2, 3);
//     get_idx(number, 2);
// }

// fn get_idx(number: (i32, i32, i32), elemnt:i32) {

//     let i = 0;
//     for item in number.iter(){
//         if i == elemnt {
//             return item
//         }
//         else {
//             i+=1;
//         }
//     }
//     println!("{:?}", number)

// }

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers: (i32, i32, i32) = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;
        let second = numbers.1;

        println!("second =={}", second);

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
