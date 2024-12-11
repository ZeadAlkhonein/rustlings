fn main() {
    // You can optionally experiment here.
    // mod tests;
    // tests::simple_option()
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        // let mut word: String = String::new();
        // TODO: Make this an if-let statement whose value is `Some`.
       
        if let Some(word) = optional_target {
            println!("heey");
            assert_eq!(word, target);
        }
        // word = optional_target {
        // }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        while let Some(integer) = optional_integers.pop() {
            // println!("{:?}",(integer));
            match integer {
                Some(integer) => {
                    assert_eq!(integer, cursor);
                    cursor -= 1;
                }
                _ => println!("None" ),
                
            }
            
        }

        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        assert_eq!(cursor, 0);
    }
}
