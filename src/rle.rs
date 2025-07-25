
// takes vector of bytes
// returns vector of tuples: byte, num_times
// becasue z80 register is 8bit, num_times is 8bit, so $FE 500 in a row is $FE, 255 / $FE, 245 
pub fn rle(raw: Vec<u8>) -> Vec<(u8, u8)> {
    println!("Raw input bytes: {}", raw.len());     // len rather than length - already better than Java! :)
    let mut rle_vec: Vec<(u8, u8)> = Vec::new();

    let mut index = 0;

    while index < raw.len() {
        let next_byte: u8 = raw[index];
        let mut count = 1;

        index += 1;

        while index < raw.len() && next_byte == raw[index] {
            count += 1;
            index += 1;
            if count == 255 {
                break;
            }
        }

        rle_vec.push((next_byte, count));        
    }

    println!("RLE bytes: {}", rle_vec.len() * 2);     // len rather than length - already better than Java! :)
    rle_vec       // yeah this is gonna feel weird for a while...
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
            let mut raw: Vec<u8> = Vec::new();
            raw.push(1);

            for _number in 0..500 {
                raw.push(2);
            }
            raw.push(1);

            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 255), (2, 245), (1, 1)];
            let rle_test: Vec<(u8, u8)> = rle(raw);

            assert_eq!(rle_good, rle_test);
        }

}

