use nom::*;

const START_END: u8 = 0xC0;
const ESCAPE: u8 = 0xDB;

/// Encodes slice of bytes according to the SLIP-protocol.
fn slip_encode(msg: &[u8]) -> Vec<u8> {
    // preserve minimal length
    let mut encoded = Vec::with_capacity(msg.len() + 2); 

    // First byte is 0xC0
    encoded.push(START_END);

    // Go through bytes and look for values that must be escaped
    msg.iter().for_each(|byte| match byte {
        &START_END => {
            encoded.push(ESCAPE);
            encoded.push(0xDC);
        }
        &ESCAPE => {
            encoded.push(ESCAPE);
            encoded.push(0xDD);
        }
        b => {
            encoded.push(*b);
        }
    });
    // Last byte is 0xC0
    encoded.push(START_END);

    encoded
}

/// Decoder-function with the parser-combinator of the 
/// [`nom`-crate](https://crates.io/crates/nom).
fn slip_decode(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    complete!(          // no streaming data
        input,
        do_parse!(
            tag!(&[START_END])                  // Should start with `0xC0`
                >> bytes:
                    many_till!(                 // Take bytes until ...
                        alt!(                               // Bytes are either ...
                            do_parse!(
                                tag!(&[ESCAPE])             // ... escaped or...
                                    >> byte: return_error!(             // Early return if not escaped correctly
                                        ErrorKind::Custom(1u32),
                                        switch!(take!(1),
                                        &[0xDC] => value!(START_END) |  // Map escaped ...
                                        &[0xDD] => value!(ESCAPE)       // ... bytes
                                        )
                                    )
                                    >> (byte)
                            ) | map!(take!(1), |b| b[0])    // ... unescaped
                        ),
                        tag!(&[START_END])      // ... again `0xC0` occures
                    )
                >> (bytes.0)
        )
    )
}

fn main() {
    let msg = [0xaa, 0xbb, 0xcc];
    let msg_escape = [0x00, 0xc0, 0x00, 0xdb, 0x00];

    let encoded = slip_encode(&msg);
    let encoded_escape = slip_encode(&msg_escape);

    println!("Normal: {:02X?}", encoded);
    println!("Escaped: {:02X?}", encoded_escape);

    let (_, decoded) = slip_decode(&encoded).unwrap();
    let (_, decoded_escaped) = slip_decode(&encoded_escape).unwrap();

    assert_eq!(&msg, decoded.as_slice());
    assert_eq!(&msg_escape, decoded_escaped.as_slice());
}
