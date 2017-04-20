/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {

    let mut output: Vec<u8> = Vec::new();

    for value in values {
        let mut processing_value = *value as u64;
        let mut buffer = processing_value & 0x7f;
        while {
                  processing_value >>= 7;
                  processing_value > 0
              } {
            buffer <<= 8;
            buffer |= (processing_value & 0x7f) | 0x80;
        }

        while {
                  let current_val: u8 = buffer as u8;
                  output.push(current_val);
                  buffer & 0x80 > 0
              } {
            buffer >>= 8;
        }
    }

    output
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut value = 0u32;
    let mut result: Vec<u32> = Vec::new();
    let mut count: usize = 0;
    for byte in bytes {
        let cur = *byte as u32;
        count += 1;
        if count == 5 && value > 0x01_ff_ff_ff {
            return Err("overflow");
        }
        value = (value << 7) + (cur & 0x7f);
        if cur & 0x80 == 0 {
            result.push(value);
            value = 0;
            count = 0;
        }
    }

    if count != 0 {
        Err("incomplete byte sequence")
    } else {
        Ok(result)
    }
}
