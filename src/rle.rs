use std::fs;
use std::io;

// takes vector of bytes
// returns vector of tuples: byte, num_times
// becasue z80 register is 8bit, num_times is 8bit, so $FE 500 in a row is $FE, 255 / $FE, 245 
pub fn rle(bitmap: Vec<u8>) -> Vec<(u8, u8)> {
    println!("Bitmap input bytes: {}", bitmap.len());     // len rather than length - already better than Java! :)
    let mut rle_vec: Vec<(u8, u8)> = Vec::new();

    let mut index = 0;

    while index < bitmap.len() {
        let next_byte: u8 = bitmap[index];
        let mut count = 1;

        index += 1;

        while index < bitmap.len() && next_byte == bitmap[index] {
            count += 1;
            index += 1;
            if count == 255 {
                break;
            }
        }

        rle_vec.push((next_byte, count));        
    }

    println!("RLE bytes: {}", rle_vec.len() * 2);     // *2 for tuple of bytes
    rle_vec       // yeah this is gonna feel weird for a while...
}

pub fn rle_print_bitmap_raw(bitmap: Vec<u8>) {
    for byte in bitmap {
        print!("{} ", byte)
    }
}

pub fn rle_print_rle_z80(rle_bytes:  Vec<(u8, u8)>) {
    for tup_bytes in rle_bytes {
        println!("\tdefb\t\t{},\t{}", tup_bytes.0, tup_bytes.1)
    }
}

pub fn rle_write_file_rle_z80(file_name: &str, label_name: &str, rle_bytes:  Vec<(u8, u8)>) {
    let mut z80: String = String::new();

    z80.push_str(format!("{label_name}:\n").as_str());

    for tup_bytes in rle_bytes {
        z80.push_str(format!("\tdefb\t\t{},\t{}\n", tup_bytes.0, tup_bytes.1).as_str());
    }

    fs::write(file_name, z80).ok();         // yeah that .ok() to avoid Result warning... dodgy
                                                           // should at least learn to accept Result
                                                           // and panic on Err 
}


// handles the format downloaded by https://zx.remysharp.com/sprites/#tiles
// that's some zxnext format... 
// but if you skip to the $ sign (0x24) then the rest is list of tileIds
pub fn read_remy_write_file_rle_z80(file_name: &str, label_name: &str) -> io::Result<()> {
    // make out filename (just add .z80)
    let z80_file_name = file_name.to_owned() + ".z80";

    // read file
    let data: Vec<u8> = fs::read(file_name)?;

    // skip headers and stuff
    let mut index: usize = 0;
    while index < data.len() {
        if data[index] == 0x24 {
            index += 1;         // could go out of bounds if it's the last value in the file..
            break;
        }        

        index += 1;
    }

    // get the btimap part
    let bitmap_data = &data[index..data.len()];

    // rle vals
    let rle_data: Vec<(u8, u8)> = rle(bitmap_data.to_vec());

    // write label is passed in
    // passed in plus :    

    // write
    rle_write_file_rle_z80(z80_file_name.as_str(), label_name, rle_data);

    Ok(())
}

// adds .z80 to filename... so yeah ..z80.z80 but meh
// format: \tdefb\tn,\tm
// n gets replaced byt str, so can be %01010011 for attrs
pub fn remap_z80_defb_file(file_name: &str) -> io::Result<()> {
    // read file

    // find the label
    // passed in plus :

    // format \tdefb\tn,\tm
    // we replace n with chars (eg attrs %01010110)

    // and panics (new thing to learn...)
    // check \tdefb\\t - if not panic
    // read num, panic if not, etc etc


    // and hey - return error if format is bad
    // and have a panic test for it...


    Ok(())
}


// tests in same file... also odd

#[cfg(test)]              
mod tests {
    use super::*;         

    #[test]
        // tests empty returns empty 
        // (kinda pointless but it's a first test of anything -- objects vs values in them type issues)
        fn does_empty_rle() {                                 
            let bitmap: Vec<u8> = Vec::new();
            let rle_good: Vec<(u8, u8)> = Vec::new();
            let rle_test: Vec<(u8, u8)> = rle(bitmap);
            assert_eq!(rle_good, rle_test);
        }

    #[test]
        // tests something doesn't return empty - a negative
        fn does_something_not_empty_rle() {                                 
            let bitmap: Vec<u8> = Vec::new();
            let rle_good: Vec<(u8, u8)> = vec![(1, 2)];
            let rle_test: Vec<(u8, u8)> = rle(bitmap);
            assert_ne!(rle_good, rle_test);
        }

    #[test]
        // doesn't worry about breaching u8 for repeated times
        fn does_small_rle() {                                 
            let bitmap: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 3, 2, 1];
            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 2), (3, 3), (4, 2), (3,1), (2, 1), (1, 1)];
            let rle_test: Vec<(u8, u8)> = rle(bitmap);
            assert_eq!(rle_good, rle_test);
        }

    #[test]
        // more than 255 in a row, so for $FE 500 times expect back $FE, 255 / $FE, 245
        fn does_large_rle() {
            let mut bitmap: Vec<u8> = Vec::new();
            bitmap.push(1);

            for _number in 0..500 {
                bitmap.push(2);
            }
            bitmap.push(1);

            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 255), (2, 245), (1, 1)];
            let rle_test: Vec<(u8, u8)> = rle(bitmap);

            assert_eq!(rle_good, rle_test);
        }

    #[test]
        fn test_rle_print_bitmap_raw() {
            let bitmap: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 3, 2, 1];
            rle_print_bitmap_raw(bitmap);
        }

    #[test]
        fn test_rle_print_rle_z80() {
            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 2), (3, 3), (4, 2), (3,1), (2, 1), (1, 1)];
            rle_print_rle_z80(rle_good);
        }

    #[test]
        fn test_rle_write_file_rle_z80() {
            let rle_good: Vec<(u8, u8)> = vec![(1, 1), (2, 2), (3, 3), (4, 2), (3,1), (2, 1), (1, 1)];
            rle_write_file_rle_z80("rle.z80", "TEST_LABEL", rle_good);
        }

    #[test]
        fn test_read_remy_write_file_rle_z80() {
            assert!(!read_remy_write_file_rle_z80("test_remy.map", "TEST_REMY_LABEL").is_err());
        }

}

