use std::{io::stdin, time::Instant};
mod constants;
mod function_timer;
mod tests;

fn main() {    
    let message : String = get_message();
    let hash_result = hash(&message);
    println!("Output: {}", hash_result);
}

//Gets the message to perform the hash on, either through command line arguments, the terminal, or a default fallback string
fn get_message() -> String {
    //if the executable is supplied with command line arguments, read the entire line as a continuous string and run the hash function on that    
    let args: Vec<String> = std::env::args().collect();
    if args.len()>1 {
        return args[1..args.len()].join(" ");
    }
    //otherwise ask the user to input a string into the terminal
    else {        
        println!("No command line arguments detected, input a string to be hashed.");
        let typed_input = get_input();
        if typed_input.len() > 0 {
            return typed_input;
        }

        //if the input is empty, a default string message is used instead
        return String::from("testingsha256_123123_123123_123123");
    }
}

//reads a single line from the terminal
fn get_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_owned();
}


fn hash(message: &str) -> String {
    
    unsafe{
    //println!("Performing hash on \"{message}\" ...");
    let time = Instant::now();
    let bytes: Vec<u8> = string_to_padded_byte_vector(message);
    let bytes_time = time.elapsed().as_nanos();
    println!("Step 1: {bytes_time} ns");

    let time = Instant::now();
    let _words: [u32;64] = u8_vector_to_u32_array(&bytes);
    let u8_to_u32_v1_time = time.elapsed().as_nanos();
    println!("Step 2 v1: {u8_to_u32_v1_time} ns");

    let time = Instant::now();
    let mut words: [u32;64] = u8_vector_as_u32_array_fast(bytes);
    let u8_to_u32_v2_time = time.elapsed().as_nanos();
    println!("Step 2 v2: {u8_to_u32_v2_time} ns");
    
    let modified_constants = loop {
        let time = Instant::now();
        create_message_schedule(&mut words);
        let schedule_time = time.elapsed().as_nanos();
        println!("Step 3: {schedule_time} ns");

        let time = Instant::now();
        let modifiers: [u32;8] = compression_loop(&words);
        let compression_time = time.elapsed().as_nanos();
        println!("Step 4: {compression_time} ns");
        break modifiers;
    };

    let time = Instant::now();
    let hash_value : String = convert_modifier_array_to_string(&modified_constants);
    let convert_mods_to_string_time = time.elapsed().as_nanos();
    println!("Step 5: {convert_mods_to_string_time} ns");

    
    let time = Instant::now();
    let _hash_value : String = convert_modifier_array_to_string_fast(&modified_constants);
    let convert_mods_to_string_time = time.elapsed().as_nanos();
    println!("Step 5.2: {convert_mods_to_string_time} ns");
    return hash_value;
    }

    //println!("Finished hash...");    
    
}

//Converts a string to binary, appends a 1, then outputs a vector of bytes (u8),
//the vector is padded with zeroes until the size of the vector is divisible by 64,
//ending with the big-endian (u64) representation of the length of the original string
fn string_to_padded_byte_vector(string: &str) -> Vec<u8>{
    let mut bytes: Vec<u8> = string.as_bytes().to_vec();

    //if for some reason the length of the string fails to convert to a u64, this length variable defaults to zero (.unwrap_or(0))
    //this will produce an inaccurate hash, but it will stop the program from crashing
    let message_length: u64 = bytes.len().try_into().unwrap_or(0);

    //append a 1
    bytes.push(0b_10000000);

    //append zeros to the vector until its size is a multiple of 64, less 8 bytes (leaving room the u64 length on the end)
    while (bytes.len()%64) < 56 {
        bytes.push(0b_00000000);
    }
    
    //append the u64 big endian length
    for byte in message_length.to_be_bytes(){
        bytes.push(byte);
    }
    assert!(bytes.len()%64==0, "Byte array error: number of bits is not divisible by 512.");

    return bytes;
}

//Converts a vector of bytes(u8) into an array of 32-bit "words"
//Performs a bit shifting algorithm on the array, then returns it
fn create_message_schedule(words: *mut [u32;64]) {
    unsafe{
        for i in 16..=63 {
            let s0 : u32 = ((*words)[i-15].rotate_right(7)) ^ ((*words)[i-15].rotate_right(18)) ^ ((*words)[i-15]>>3);
            let s1 : u32 = ((*words)[i-2].rotate_right(17)) ^ ((*words)[i-2].rotate_right(19)) ^ ((*words)[i-2]>>10);        
            (*words)[i] =
            (*words)[i-16]
                .wrapping_add(s0)
                .wrapping_add((*words)[i-7])
                .wrapping_add(s1);
        }
    }
}

//Uses the previously bitshifted "word" array to create a smaller array of modified constants
//These are appended to each other to build the final hash string
fn compression_loop(words: &[u32;64]) -> [u32;8]{
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
        constants::SQUARE_ROOTS[0],
        constants::SQUARE_ROOTS[1],
        constants::SQUARE_ROOTS[2],
        constants::SQUARE_ROOTS[3],
        constants::SQUARE_ROOTS[4],
        constants::SQUARE_ROOTS[5],
        constants::SQUARE_ROOTS[6],
        constants::SQUARE_ROOTS[7]
    );

    for i in 0..=63 {
        let s1 : u32 = e.rotate_right(6) ^ e.rotate_right(11) ^ (e.rotate_right(25));
        let ch = (e&f)^((!e)&g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(constants::CUBE_ROOTS[i])
            .wrapping_add(words[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a&b)^(a&c)^(b&c);
        let temp2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    let mut modifiers = [a, b, c, d, e, f, g, h];
    for i in 0..8 {
        modifiers[i] = modifiers[i].wrapping_add(constants::SQUARE_ROOTS[i]);
    }
    return modifiers;
}

fn convert_modifier_array_to_string(mods: &[u32;8]) -> String{
    let mut hash_string : String = String::with_capacity(64);
    for i in 0..8 {
        for b in mods[i].to_ne_bytes(){
            let s = format!("{:02X}", &b);
            hash_string.push_str(&s);
        }
    }
    return hash_string;
}

//About 6 times faster than the regular method, but unsafe
unsafe fn convert_modifier_array_to_string_fast(mods: *const [u32;8]) -> String{
    let ptr: *const [u8;32] = mods as *const [u8;32];
    return hex::encode(*ptr);
}

fn u8_vector_to_u32_array(bytes: &Vec<u8>) -> [u32;64]{
    let mut words: [u32;64] = [0;64];
    for index in 0..16 {
        let word: u32 = u32::from_ne_bytes([
            bytes[index*4],
            bytes[index*4+1],
            bytes[index*4+2],
            bytes[index*4+3]]);

        words[index] = word;
    }
    return words;
}

unsafe fn u8_vector_as_u32_array_fast(vector: Vec<u8>) -> [u32;64]{
    return *(vector.as_ptr() as *const [u32;64]);
}
