
// takes vector of bytes
// returns vector of tuples: byte, num_times
// becasue z80 register is 8bit, num_times is 8bit, so $FE 500 in a row is $FE, 255 / $FE, 245 
pub fn rle(raw: Vec<u8>) -> Vec<(u8, u8)> {
    println!("Raw input bytes: {}", raw.len());     // len rather than length - already better than Java! :)
    let v: Vec<(u8, u8)> = Vec::new();


    // get a number
    // how many are of them?
    // write tuple to vec
    // next number


    println!("RLE bytes: {}", v.len());     // len rather than length - already better than Java! :)
    v   // don't have to say 'return' for final expression being the return value
}


#[cfg(test)]              // commented out so 'build' builds the tests too
mod tests {
    use super::*;         

    #[test]
        // tests empty returns empty 
        // (kinda pointless but it's a first test of anything -- objects vs values in them type issues)
        fn does_empty_rle() {                                 
            let raw: Vec<u8> = Vec::new();
            let rle_good: Vec<(u8, u8)> = Vec::new();
            let rle_test: Vec<(u8, u8)> = rle(raw);
            assert_eq!(rle_good, rle_test);
        }

    #[test]
        // tests something doesn't return empty - a negative
        fn does_something_not_empty_rle() {                                 
            let raw: Vec<u8> = Vec::new();
            let rle_good: Vec<(u8, u8)> = vec![(1, 2)];
            let rle_test: Vec<(u8, u8)> = rle(raw);
            assert_ne!(rle_good, rle_test);
        }

    #[test]
        // doesn't worry about breaching u8 for repeated times
        fn does_small_rle() {                                 
            let raw: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 3, 2, 1];
            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 2), (3, 3), (4, 2), (3,1), (2, 1), (1, 1)];
            let rle_test: Vec<(u8, u8)> = rle(raw);
            assert_eq!(rle_good, rle_test);
        }

    #[test]
        // more than 255 in a row, so for $FE 500 times expect back $FE, 255 / $FE, 245
        fn does_large_rle() {
            assert_eq!(true, false);                        
        }

}

