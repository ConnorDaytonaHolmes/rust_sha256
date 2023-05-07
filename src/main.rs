use std::io::stdin;
mod constants;
mod function_timer;
mod tests;

fn main() {  
    println!("Welcome!");  
    let message = get_message();
    let hash_result = benchmark!(format!("Hashing \"{message}\""), hash(&message));
    println!("Result: {hash_result}");

    while let Some(msg) = msg_from_console() {
        let hash_result = hash(&msg);
        println!("Result: {hash_result}");        
    }
}

//Gets the message to perform the hash on, either through command line arguments,
//the terminal, or a default fallback string
fn get_message() -> String {
    //if the executable is supplied with command line arguments,
    //read the entire line as a continuous string and run the hash function on that   
    if let Some(m) = msg_from_command_line_args() {
        return m;
    } //otherwise ask the user for an input from the console
    else if let Some(m) = msg_from_console() {
        return m;
    }
    //if the user inputs nothing, use a default string
    return String::from("Hello World!");
        
}


fn msg_from_command_line_args() -> Option<String>{
    let args: Vec<String> = std::env::args().collect();
    if args.len()>1 {
        println!("Using command line arguments as input...");
        return Some(args[1..args.len()].join(" "));
    }
    None
}

fn msg_from_console() -> Option<String> {
    println!("Type an input to be hashed, or hit Enter to exit.");
    let input_result = get_input();
        if let Ok(input) = input_result {
            if input.len() > 0 {
                return Some(input);
            }
        }
        None
}

//reads a single line from the terminal
fn get_input() -> std::io::Result<String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            Ok(input.trim().to_owned())
        },
        Err(e) => {
            println!("Line read error");
            Err(e)
        }
    }
}

fn hash(message: &str) -> String {        
    println!("Performing hash on \"{message}\" ...");
    let bytes: Vec<u8> = string_to_padded_byte_vector(message);
    let message_block : Vec<u32> = bytes_to_message_block(&bytes);
    let mut modifiers: [u32;8] = constants::SQUARE_ROOTS.clone();
    
    for i in 0..message_block.len() / 16 {
        let message_schedule = create_message_schedule(&message_block[i*16..(i+1)*16]);
        modifiers = compression_loop(&modifiers, &message_schedule);
    }
    return convert_modifier_array_to_string(&modifiers);    
}

//Converts a string to binary, appends a 1, then outputs a vector of bytes (u8),
//the vector is padded with zeroes until the size of the vector is divisible by 64,
//ending with the big-endian (u64) representation of the length of the original string
fn string_to_padded_byte_vector(string: &str) -> Vec<u8>{
    let mut bytes: Vec<u8> = string.as_bytes().to_vec();

    //if for some reason the length of the string fails to convert to a u64, this length variable defaults to zero (.unwrap_or(0))
    //this will produce an inaccurate hash, but it will stop the program from crashing
    let message_length_in_bits: u64 = (bytes.len()*8) as u64;

    //append a 1
    bytes.push(0b_10000000);

    //append zeros to the vector until its size is a multiple of 64, less 8 bytes (leaving room the u64 length on the end)
    while (bytes.len()%64) != 56 {
        bytes.push(0b_00000000);
    }

    //append the u64 big endian length
    for byte in message_length_in_bits.to_be_bytes(){
        bytes.push(byte);
    }

    assert!(bytes.len()%64==0, "Byte array error: number of bits is not divisible by 512.");

    return bytes;
}

fn bytes_to_message_block(bytes : &Vec<u8>) -> Vec<u32>
{
    assert!(bytes.len()%64==0);
    let mut message_block : Vec<u32> = Vec::new();
    
    for index in 0..bytes.len()/4 {
        let word: u32 = u32::from_be_bytes([
            bytes[index*4],
            bytes[index*4+1],
            bytes[index*4+2],
            bytes[index*4+3]]
        );
        message_block.push(word);
    }    
    message_block
}

//Performs a bit shifting algorithm on the array, then returns it
fn create_message_schedule(message_chunk: &[u32]) -> [u32;64] {
    let mut m_schedule = [0_u32;64];

    for i in 0..16 {
        m_schedule[i]=message_chunk[i];
    }

    for i in 16..64 {
        let s0 : u32 = (m_schedule[i-15].rotate_right(7)) ^ (m_schedule[i-15].rotate_right(18)) ^ (m_schedule[i-15]>>3);
        let s1 : u32 = (m_schedule[i-2].rotate_right(17)) ^ (m_schedule[i-2].rotate_right(19)) ^ (m_schedule[i-2]>>10);        
        m_schedule[i] =
            m_schedule[i-16]
            .wrapping_add(s0)
            .wrapping_add(m_schedule[i-7])
            .wrapping_add(s1);
    }   
    m_schedule 
}

//Uses the previously bitshifted "word" array to create a smaller array of modified constants
//These are appended to each other to build the final hash string
fn compression_loop(working_variables : &[u32;8], words: &[u32;64]) -> [u32;8]{
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = 
    (working_variables[0], working_variables[1], working_variables[2], working_variables[3], 
        working_variables[4], working_variables[5], working_variables[6], working_variables[7]);

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
        modifiers[i] = modifiers[i].wrapping_add(working_variables[i]);
    }
    return modifiers;
}

fn convert_modifier_array_to_string(mods: &[u32;8]) -> String{
    let mut hash_string : String = String::with_capacity(64);
    for i in 0..8 {
        let s = hex::encode(mods[i].to_be_bytes());
        hash_string.push_str(&s);
    }
    return hash_string;
}
