#[cfg(test)]
use crate::*;
const _HW_PADDED_BINARY_ARRAY_U8 : [u8;64] = [
    0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f,
    0x72, 0x6c, 0x64, 0x80, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58
];

const _HW_PADDED_BINARY_ARRAY_U32 : [u32;64] = [
    0x68656c6c, 0x6f20776f, 0x726c6480, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000058,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000
];

#[test]
fn check_byte_padding(){
    let s = String::from("hello world");
    let padded_binary_vector : Vec<u8> = Vec::from(_HW_PADDED_BINARY_ARRAY_U8);
    for char in s.as_bytes(){
        println!("{char}");
    }
    let stp_bv = string_to_padded_byte_vector(&s);

    assert!(padded_binary_vector.len()==stp_bv.len(), "Length mismatch between correct binary vector & generated one.");

    let mut index = 0;
    let b: bool = loop{
        if index >= padded_binary_vector.len() {
            break true;
        }
        //println!("Byte {}\n    bv: {}\nstp_bv: {}", index, bv[index], stp_bv[index]);
        if padded_binary_vector[index]!=stp_bv[index] {
            println!("mismatch at index {index}, const array is {}, generated is {}", padded_binary_vector[index], stp_bv[index]);
            break false;
        }
        index+=1;
    };

    assert!(b, "Padded vector failed");
}

#[test]
fn perform_hash_on_strings(){
    for i in 0..constants::TEST_STRINGS.len(){
        assert_eq!(
            constants::EXPECTED_RESULTS[i],
            hash(constants::TEST_STRINGS[i]),
            "\nAssertion failed on test item {i}: {}", constants::TEST_STRINGS[i])
    }
}

#[test]
fn benchmark_hash_on_strings(){
    for (i, s) in constants::TEST_STRINGS.iter().enumerate() {            
        assert_eq!(
            constants::EXPECTED_RESULTS[i],
            benchmark!(s, hash(s)),
            "\nAssertion failed on test item {i}: {}", s);
    }
}