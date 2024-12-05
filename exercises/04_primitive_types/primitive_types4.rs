fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
    let b =  nice_slice( &a, 1, 4);

    print!("{:?}", b)
}

fn nice_slice(a : &[i32], start: usize, end: usize) -> &[i32]{

    if start <= end && end <=a.len() {

         return &a[start..end];
    }
    else {
         return &a;
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
