use bytes::{Buf, BufMut};

/// Returns how many bytes are needed to encode a value.
#[inline]
pub fn length(value: u64) -> usize {
    let zero_len = 64 - value.leading_zeros();
    let offset = if zero_len == 0 { 7 } else { 6 };
    ((offset + zero_len) / 7) as usize
}

/// Encode a `u64` integer to the [`bytes::BufMut`]. Returns how many bytes were
/// encoded.
#[inline]
pub fn encode(mut val: u64, mut buf: impl BufMut) -> usize {
    let mut written = 1;

    while val > 127 {
        buf.put_u8((val as u8) | 128);
        val >>= 7;
        written += 1;
    }
    buf.put_u8(val as u8);
    written
}

/// Read a `u64` from a [`bytes::Buf`].
#[inline]
pub fn decode(mut buf: impl Buf) -> u64 {
    let mut val = 0 as u64;
    let mut fac = 1 as u64;

    loop {
        let byte = buf.get_u8();
        val += fac * u64::from(byte & 127);
        fac <<= 7;
        if byte & 128 == 0 {
            break;
        }
    }
    val
}

/// Returns how many bytes are needed to encode a value.
#[inline]
pub fn signed_length(value: i64) -> usize {
    length(unsign(value))
}

/// Encode a `i64` (signed) integer in the [`bytes::BufMut`].
/// Returns how many bytes were encoded.
#[inline]
pub fn signed_encode(value: i64, buf: impl BufMut) -> usize {
    encode(unsign(value), buf)
}

/// Decode a single `i64` (signed) integer from a [`bytes::Buf`].
#[inline]
pub fn signed_decode(buf: impl Buf) -> i64 {
    sign(decode(buf))
}

/// Convert an `i64` into a `u64`.
#[inline]
fn unsign(value: i64) -> u64 {
    if value >= 0 {
        (value * 2) as u64
    } else {
        (value * -2 - 1) as u64
    }
}

/// Convert a `u64` into a `i64`.
#[inline]
fn sign(value: u64) -> i64 {
    if value & 1 != 0 {
        -(((value + 1) / 2) as i64)
    } else {
        (value / 2) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_encode() {
        let mut buf = Vec::new();
        assert_eq!(encode(100, &mut buf), 1);
        assert_eq!(buf[0], 100);

        assert_eq!(encode(1000, &mut buf), 2);
        assert_eq!(buf[1], 232);
        assert_eq!(buf[2], 7);
    }

    #[test]
    fn encoded2() {
        // encode a `u64` integer into a buffer
        let mut buf = Vec::new();
        let mut bytes_encoded = encode(100, &mut buf);
        assert_eq!(buf[0], 100);

        let mut buf = BytesMut::from(&*buf);
        bytes_encoded += encode(1000, &mut buf);
        assert_eq!(buf[1], 232);
        assert_eq!(buf[2], 7);
        assert_eq!(bytes_encoded, 3);
    }

    #[test]
    fn test_decode() {
        let mut buf = &[100u8, 232, 7][..];
        assert_eq!(decode(&mut buf), 100);
        assert_eq!(buf, &[232, 7][..]);

        assert_eq!(decode(&mut buf), 1000);
        assert_eq!(buf.remaining(), 0);

        let mut buf = Bytes::from(&[100u8, 232, 7][..]);
        assert_eq!(decode(&mut buf), 100);
        assert_eq!(decode(&*buf), 1000);
    }

    #[test]
    fn test_length() {
        assert_eq!(length(100), 1);
        assert_eq!(length(1000), 2);

        assert_eq!(length(1 << 49), 8);
        assert_eq!(length((1 << 56) - 1), 8);

        assert_eq!(length(1 << 56), 9);
        assert_eq!(length((1 << 63) - 1), 9);

        assert_eq!(length(1 << 63), 10);
    }

    #[test]
    fn test_signed_encode() {
        let mut buf = Vec::new();
        assert_eq!(signed_encode(100, &mut buf), 2);
        assert_eq!(buf[0], 200);
        assert_eq!(buf[1], 1);

        assert_eq!(signed_encode(-100, &mut buf), 2);
        assert_eq!(buf[2], 199);
        assert_eq!(buf[3], 1);
        assert_eq!(buf.len(), 4);
    }

    #[test]
    fn test_signed_decode() {
        let mut buf = &[200u8, 1][..];
        assert_eq!(signed_decode(&mut buf), 100);
        assert_eq!(buf.remaining(), 0);

        let mut buf = Bytes::from(&[199, 1][..]);
        assert_eq!(signed_decode(&mut buf), -100);
        assert_eq!(buf.remaining(), 0);
    }

    #[test]
    fn test_signed_length() {
        assert_eq!(signed_length(100), 2);
        assert_eq!(signed_length(-100), 2);
    }
}
