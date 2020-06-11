varintbuf
=====================
[![Build Status](https://travis-ci.com/mattsse/varintbuf.svg?branch=master)](https://travis-ci.com/mattsse/varintbuf)
[![Crates.io](https://img.shields.io/crates/v/varintbuf.svg)](https://crates.io/crates/varintbuf)
[![Documentation](https://docs.rs/varintbuf/badge.svg)](https://docs.rs/varintbuf)

This crate provides varint encoding/decoding and is based on the [varinteger](https://github.com/datrs/varinteger/) crate, but is adapted for [bytes](https://github.com/tokio-rs/bytes).


## Usage

## Decode
```rust
    extern crate bytes;

    // decode a `u64` integer from a buffer
    let mut buf = &[100u8, 232, 7][..];
    let value = varintbuf::decode(&mut buf);
    assert_eq!(value, 100);
    
    let mut buf = Bytes::from(buf);
    let value = varintbuf::decode(&mut buf);        
    assert_eq!(value, 1000)

```

## Encode
```rust
    extern crate bytes;
    
    // encode a `u64` integer into a buffer
    let mut buf = Vec::new();
    let mut bytes_encoded = encode(100, &mut buf);
    assert_eq!(buf[0], 100);
    
    let mut buf = BytesMut::from(&*buf);
    bytes_encoded += encode(1000, &mut buf);
    assert_eq!(buf[1], 232);
    assert_eq!(buf[2], 7);
    assert_eq!(bytes_encoded, 3);
    assert_eq!(bytes_encoded, buf.len());
```

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
   
