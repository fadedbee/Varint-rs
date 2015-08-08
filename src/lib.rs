//! An implementation of Google Protobuf's Variable-Length Integers

use std::collections::VecDeque;

extern crate bit_utils;

use bit_utils::BitInformation;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_32_MAX_BYTES: usize = 5;

/// The maximum number of bytes used by a 32-bit Varint
pub const VARINT_64_MAX_BYTES: usize = 10;

/// A struct defining a variable-length integer
#[derive(Clone, Debug)]
pub struct Varint {

    /// The internal data representation
    pub data: VecDeque<u8>

}

impl Varint {

    /// Gets the number of bytes currently contained by this Varint
    pub fn number_of_bytes(&self) -> usize {
        self.data.len()
    }

}

/// Transforms a signed int to an unsigned int via zig-zag transformation
pub fn zigzag_signed_int(input: i32) -> u32 {
    ((input << 1) ^ (input >> 31)) as u32
}

/// Transforms a signed long to an unsigned long via zig-zag transformation
pub fn zigzag_signed_long(input: i64) -> u64 {
    ((input << 1) ^ (input >> 63)) as u64
}

/// Transforms an unsigned int to a signed int via zig-zag transformation
pub fn zigzag_unsigned_int(input: u32) -> i32 {
    ((input >> 1) as i32) ^ (-((input & 1) as i32))
}

/// Transforms an unsignigned long to a signed long via zig-zag transformation
pub fn zigzag_unsigned_long(input: u64) -> i64 {
    ((input >> 1) as i64) ^ (-((input & 1) as i64))
}

/// Encodes a signed i32 as a Varint
pub fn encode_signed_varint32(input: i32) -> Varint {
    encode_unsigned_varint32(zigzag_signed_int(input))
}

/// Encodes a signed i64 as a Varint
pub fn encode_signed_varint64(input: i64) -> Varint {
    encode_unsigned_varint64(zigzag_signed_long(input))
}
    
/// Encodes an unsigned u32 as a Varint, returning the Varint.
pub fn encode_unsigned_varint32(input: u32) -> Varint {

    let mut returnable: Varint = Varint { data: VecDeque::<u8>::new() };
    
    let mut value: u32 = input;
    
    if value == 0 {
        returnable.data.push_back(0);
        return returnable;
    } else {
        
        while value >= 0b10000000 {
            let next_byte: u8 = ((value & 0b01111111) as u8) | 0b10000000;
            
            value = value >> 7;
                    
            returnable.data.push_back(next_byte);
        }
        
        returnable.data.push_back((value & 0b01111111) as u8);
        
        return returnable;
    }

}

/// Encodes an unsigned u64 as a Varint, returning the Varint
pub fn encode_unsigned_varint64(input: u64) -> Varint {

    let mut returnable: Varint = Varint { data: VecDeque::<u8>::new() };
        
    let mut value: u64 = input;
    
    if value == 0 {
        returnable.data.push_back(0);
        return returnable;
    } else {
        
        while value >= 0b10000000 {
            let next_byte: u8 = ((value & 0b01111111) as u8) | 0b10000000;
            
            value = value >> 7;
                    
            returnable.data.push_back(next_byte);
        }
        
        returnable.data.push_back((value & 0b01111111) as u8);
        
        return returnable;
    }

}

/// Decodes an unsigned varint32, returning a result of either a u32 or a string explaining the error
pub fn decode_unsigned_varint32(original_input: &Varint) -> Result<u32, &'static str> {

    let mut input = original_input.clone();
    
    if input.number_of_bytes() == 0 {
        return Err("Varint somehow has zero bytes! Are you decoding something you wrote?");
    } else if input.number_of_bytes() > VARINT_32_MAX_BYTES {
        return Err("Varint is larger than VARINT_32_MAX_BYTES");
    } else if input.number_of_bytes() == 1 {
        let returnable = input.data.pop_front();
        
        if returnable.is_none() {
            return Err("Only byte contained in a VecDeque is a byte but is also None. You might want to run memtest");
        } else {
            return Ok(returnable.unwrap() as u32);
        }
    } else {
        let mut shift_amount: u32 = 0;
        let mut decoded_value: u32 = 0;
        while input.number_of_bytes() >= 1 {
            let byte_result = input.data.pop_front();
            
            if byte_result.is_none() {
                return Err("Byte contained in a VecDeque is a byte that is also None. Run memtest, please");
            } else {
                let byte_value: u8 = byte_result.unwrap();
                
                decoded_value |= ((byte_value & 0b01111111) as u32) << shift_amount; //<< 0 for first byte
                
                if byte_value.has_most_signifigant_bit() == false {
                    return Ok(decoded_value);
                } else {
                    shift_amount += 7;
                }
            }
        }
        
        Err("No bytes were marked as the end byte. Check your numbers or run a memtest")
    }
}

/// Decodes an unsigned varint64, returning a result of either a u64 or a string explaining the error
pub fn decode_unsigned_varint64(original_input: &Varint) -> Result<u64, &'static str> {

    let mut input = original_input.clone();

    if input.number_of_bytes() == 0 {
        return Err("Varint somehow has zero bytes! Are you decoding something you wrote?");
    } else if input.number_of_bytes() > VARINT_64_MAX_BYTES {
        return Err("Varint is larger than VARINT_64_MAX_BYTES");
    } else if input.number_of_bytes() == 1 {
        let returnable = input.data.pop_front();
        
        if returnable.is_none() {
            return Err("Only byte contained in a VecDeque is a byte but is also None. You might want to run memtest");
        } else {
            return Ok(returnable.unwrap() as u64);
        }
    } else {
        let mut shift_amount: u64 = 0;
        let mut decoded_value: u64 = 0;
        while input.number_of_bytes() >= 1 {
            let byte_result = input.data.pop_front();
            
            if byte_result.is_none() {
                return Err("Byte contained in a VecDeque is a byte that is also None. Run memtest, please");
            } else {
                let byte_value: u8 = byte_result.unwrap();
                
                decoded_value |= ((byte_value & 0b01111111) as u64) << shift_amount; //<< 0 for first byte
                
                if byte_value.has_most_signifigant_bit() == false {
                    return Ok(decoded_value);
                } else {
                    shift_amount += 7;
                }
            }
        }
        
        Err("No bytes were marked as the end byte. Check your numbers or run a memtest")
    }
}

#[cfg(test)]
mod test {

    use super::*;
    
    use std::collections::VecDeque;
    
    extern crate bit_utils;
    
    use bit_utils::BitInformation;
    
    #[test]
    fn test_endecoding_zero() {
        let value = 0;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_unsigned_varint32(value);
        
        assert_eq!(1, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint32(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, result);
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_one() {
        let value = 1;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_unsigned_varint32(value);
        
        assert_eq!(1, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint32(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, result);
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_trillion() {
        let value = 1_000_000_000_000;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_unsigned_varint64(value);
        
        assert_eq!(6, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint64(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, result);
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_minus_one() {
        let value = -1;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_signed_varint32(value);
        
        assert_eq!(1, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint32(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, zigzag_unsigned_int(result));
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_minus_one_fifty() {
        let value = -150;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_signed_varint32(value);
        
        assert_eq!(2, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint32(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, zigzag_unsigned_int(result));
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_minus_three_hundred() {
        let value = -300;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_signed_varint32(value);
        
        assert_eq!(2, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint32(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, zigzag_unsigned_int(result));
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_endecoding_minus_one_hundred_trillion() {
        let value = -100_000_000_000_000;
        
        assert_eq!(value, value); //Congratulations. You've sucessfully switched to a parallel universe where 0 is actually 1
        
        let encoded: Varint = encode_signed_varint64(value);
                
        assert_eq!(7, encoded.number_of_bytes());
        
        let result = decode_unsigned_varint64(&encoded);
        
        if result.is_ok() {
            let result = result.unwrap();
            
            assert_eq!(value, zigzag_unsigned_long(result));
        } else {
            assert_eq!(0, 1); //You're still in that alternate universe, bro
        }
    }
    
    #[test]
    fn test_zigzag_unsigned_value() {
        let mut unsigned: u32 = 0;
        
        assert_eq!(unsigned, zigzag_unsigned_int(unsigned) as u32);
        
        unsigned = 1;
        
        assert_eq!(-1, zigzag_unsigned_int(unsigned));
        
        unsigned = 2;
        
        assert_eq!(1, zigzag_unsigned_int(unsigned));
        
        let unsigned: u64 = 18446744073709551612;
        
        assert_eq!(9223372036854775806, zigzag_unsigned_long(unsigned));
    }
    
    #[test]
    fn test_zigzag_signed_value() {
        let mut signed: i32 = 0;
        
        assert_eq!(signed, zigzag_signed_int(signed) as i32);
        
        signed = -1;
        
        assert_eq!(1, zigzag_signed_int(signed));
        
        signed = 1;
        
        assert_eq!(2, zigzag_signed_int(signed));
        
        signed = -2;
        
        assert_eq!(3, zigzag_signed_int(signed));
        
        let mut signed: i64 = 9223372036854775806;
        
        assert_eq!(18446744073709551612, zigzag_signed_long(signed));
        
        signed = -9223372036854775808;
        
        assert_eq!(18446744073709551615, zigzag_signed_long(signed));
    }
    
    #[test]
    fn test_new_varint_has_no_bytes() {
        
        let abc: Varint = Varint { data: VecDeque::<u8>::new() };
        
        assert_eq!(0, abc.number_of_bytes());
        
    }
    
    #[test]
    fn test_most_signifigant_bit() {
        let mut value: u8 = 1;
        
        assert!(value.has_most_signifigant_bit() == false);
        
        value = 120;
        
        assert!(value.has_most_signifigant_bit() == false);
        
        value = 128;
        
        assert!(value.has_most_signifigant_bit() == true);
        
        value = 129;
        
        assert!(value.has_most_signifigant_bit() == true);
    }
    
}