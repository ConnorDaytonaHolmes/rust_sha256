//square roots of first 8 primes (2-19)
pub const SQUARE_ROOTS: [u32;8] = [
    0x6a09e667,
    0xbb67ae85,
    0x3c6ef372,
    0xa54ff53a,
    0x510e527f,
    0x9b05688c,
    0x1f83d9ab,
    0x5be0cd19,
];

//cube roots of first 64 primes (2-311)
pub const CUBE_ROOTS: [u32;64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

#[cfg(test)]
pub const TEST_STRINGS: [&str; 10] = [    
    "hello world",
    "robin williams",
    "a tired dog",
    "in a land far far away",
    "q",
    "shark tale",
    "ooh eeh ooh ah ah ting tang walla walla bing bang",
    "testtesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttest",
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    "5518519118eccw9w___==+"
];

//Correct SHA256 hashes of the above strings
#[cfg(test)]
pub const EXPECTED_RESULTS: [&str; 10] = [
    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9",
    "7695d2f2e1c6043580cbe7b080e011f0c45ba993d685a16141000f94686d7eaa",
    "7a9c038b29a8bca06d194727dcd7935325fd37cec5afb16f9e799b27f073f114",
    "03714784e124f7c54d7190a8e0061da234878b32a5aa577ea1bfd9d9c81d7a2a",
    "8e35c2cd3bf6641bdb0e2050b76932cbb2e6034a0ddacc1d9bea82a6ba57f7cf",
    "f7a27d430688a3db38ee63e1688859263b2ead72fb65eb83c983bf48d8986476",
    "c910483ae4027deb0626c9c6c03cb28ed9bc433ff0e5d850d5aa36cdf48a813f",
    "39cbdf2294029cb27a54140028a99c60a1872d7914c73bb93948f7996b1f6174",
    "35c28ee2e25f5ad70384f1ca9723f520c955fb5fe9f2e56b9dc809479a9ca8cc",
    "57220503905c7d42d693c5c96796a13a302c293047f37ea28e5e5f0adb94c358"
];