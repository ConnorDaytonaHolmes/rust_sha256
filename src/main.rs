use std::io::stdin;
mod constants;

fn main() {

    //initialize a default test string
    let mut input_to_hash : String = String::from("testingsha256_123123_123123_123123");

    let args: Vec<String> = std::env::args().collect();

    //if the executable is supplied with command line arguments, read the entire line as a continuous string and run the hash function on that    
    if args.len()>1 {
        input_to_hash = args[1..args.len()].join("");
    }
    //otherwise ask the user to input a string
    else {        
        println!("No command line arguments detected, input a string to be hashed.");
        let typed_input = get_input_string();

        //if the input is empty, the default test string is used instead
        if typed_input.len() > 0 {
            input_to_hash = typed_input;
        }
    }
    println!("Input string: {input_to_hash}");
    println!("SHA256 output: {}", hash(input_to_hash));    
}

fn get_input_string() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_owned()
}

fn hash(input: String) -> String{
    let bytes: Vec<u8> = string_to_padded_byte_vector(input);
    let mut words: [u32;64] = u8_vector_to_u32_array_safe(bytes);
    words = create_message_schedule(words);
    let modifiers: [u32;8] = compression_loop(words);
    let mut modified_constants: [u32;8] = constants::SQUARE_ROOTS.clone();
    
    for i in 0..8 {
        modified_constants[i] = modified_constants[i].wrapping_add(modifiers[i]);
        println!("Value h{} : {}", i, modified_constants[i]);
    }

    unsafe {
        let hash_bytes: [u8;32] = *(modified_constants.as_ptr() as *const [u8; 32]);
        let mut hash_string : String = String::with_capacity(64);
        for byte in hash_bytes {
            let s = format!("{:02X?}", byte);
            hash_string.push_str(&s);
            println!("Byte as hex: {s}")
        }
        return hash_string;
    }
}

//Converts a string to binary, appends a 1, then outputs a vector of bytes (u8),
//the vector is padded with zeroes until the size of the vector is divisible by 64,
//ending with the big-endian (u64) representation of the length of the original string
fn string_to_padded_byte_vector(string: String) -> Vec<u8>{
    let mut bytes: Vec<u8> = string.as_bytes().to_vec();

    //if for some reason the length of the string fails to convert to a u64, this length variable defaults to zero (.unwrap_or(0))
    //this will produce an inaccurate hash, but it will stop the program from crashing
    let original_input_length: u64 = bytes.len().try_into().unwrap_or(0);

    //append a 1
    bytes.push(0b_10000000);

    //append zeros to the vector until its size is a multiple of 64, less 8 bytes (leaving room the u64 length on the end)
    while (bytes.len()%64) < 56 {
        bytes.push(0b_00000000);
    }
    
    //append the u64 length
    for byte in original_input_length.to_ne_bytes(){
        bytes.push(byte);
    }
    assert!(bytes.len()%64==0, "Byte array error: number of bits is not divisible by 512.");

    return bytes;
}

//Converts a vector of bytes(u8) into an array of 32-bit "words"
//Performs a bit shifting algorithm on the array, then returns it
fn create_message_schedule(mut words: [u32;64]) -> [u32;64] {
    for i in 16..=63 {
        let s0 : u32 = (words[i-15].rotate_right(7)) ^ (words[i-15].rotate_right(18)) ^ (words[i-15]>>3);
        let s1 : u32 = (words[i-2].rotate_right(17)) ^ (words[i-2].rotate_right(19)) ^ (words[i-2]>>10);        
        words[i] =
            words[i-16]
            .wrapping_add(s0)
            .wrapping_add(words[i-7])
            .wrapping_add(s1);
    }
    return words;
}

fn compression_loop(words: [u32;64]) -> [u32;8]{
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

    return [a, b, c, d, e, f, g, h];
}

fn u8_vector_to_u32_array_safe(bytes: Vec<u8>) -> [u32;64]{
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

fn _u8_vector_to_u32_array_unsafe(bytes: Vec<u8>) -> [u32;64]{
    unsafe{return *(bytes.as_ptr() as *const [u32;64]);}
}
