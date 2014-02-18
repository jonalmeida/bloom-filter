#[crate_id = "murmur3#0.1"];

pub mod murmur {
    
    use std::iter::range_step;

    pub fn murmur3_32_seeded(key: &str, seed: u32) -> u32 {
        
        let c1 = 0xcc9e2d51;
        let c2 = 0x1b873593;
        let r1 : u32 = 15;
        let r2 : u32 = 13;
        let m : u32 = 5;
        let n = 0xe6546b64;

        let mut hash = seed;
        let key_bytes: &[u8] = key.as_bytes();
        let len = key_bytes.len();

        for byte_index in range_step(0, len, 4) {

            // Check against len -1 since we index from 0
            if (byte_index + 3) <= (len - 1) {

                // Slice is from [x, y) so we'll use byte_index, byte_index +4
                let mut chunk = key_bytes_to_u32_chunk(
                    key_bytes.slice(byte_index, byte_index+4));
                
                chunk = chunk * c1;
                chunk = (chunk << r1) | (chunk >> (32 - r1));
                chunk = chunk * c2;
                
                hash = hash ^ chunk;
                hash = (hash << r2) | (hash >> (32 - r2));
                hash = (hash * m) + n;
                
            } else {
                // If we have less than four...
                // Make sure to slice to len + 1 to cover the final byte
                let mut chunk = key_bytes_to_u32_chunk(
                    key_bytes.slice(byte_index, len));


                chunk = chunk * c1;
                chunk = (chunk << r1) | (chunk >> (32 - r1));
                chunk = chunk * c2;

                hash = hash ^ chunk;
            }
        }

        hash = hash ^ (len as u32);
        hash = hash ^ (hash >> 16);
        hash = hash * 0x85ebca6b;
        hash = hash ^ (hash >> 13);
        hash = hash * 0xc2b2ae35;
        hash = hash ^ (hash >> 16);

        return hash;

    }


    fn key_bytes_to_u32_chunk(bytes: &[u8]) -> u32 {
        
        // TODO: Ensure that we're dealing with LE architecture, 
        // if not flip the bytes

        let chunk: u32 = match bytes.len() {
            
            4 => {
                ((bytes[3] as u32 << 24) + 
                 (bytes[2] as u32 << 16) + 
                 (bytes[1] as u32 << 8) + 
                 (bytes[0] as u32)) as u32
            },
            
            3 => { 
                ((bytes[2] as u32 << 16) + 
                 (bytes[1] as u32 << 8) + 
                 (bytes[0] as u32)) as u32
            },
            
            2 => {
                ((bytes[1] as u32 << 8) + 
                 (bytes[0] as u32)) as u32
            },

            1 => { bytes[0] as u32 },
            
            _ => 0
                
        };
        
        return chunk;
        
    }

    pub fn murmur3_32(key: &str) -> u32 { murmur3_32_seeded(key, 0) }

}


#[test]
fn murmur3_32_hello_test() {
    let hello_hash = murmur::murmur3_32("hello");
    assert!(hello_hash == 613153351);
}
