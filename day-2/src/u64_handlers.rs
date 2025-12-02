#![allow(dead_code)]

pub fn write_u64_into_buf(mut n: u64, buf: &mut [u8; 20]) -> &str {
  let mut len = 0;

  loop {
    buf[len] = b'0' + (n % 10) as u8;
    n /= 10;
    len += 1;
    if n == 0 {
      break;
    }
  }

  buf[..len].reverse();

  // CONCERN: unwrap could be cascaded out with a Result err path for prod code
  core::str::from_utf8(&buf[..len]).unwrap()
}

pub fn parse_u64_bytes(s: &str) -> Option<u64> {
  let mut n: u64 = 0;

  for &b in s.as_bytes() {
    if !b.is_ascii_digit() {
      return None;
    }
    n = n * 10 + (b - b'0') as u64;
  }

  Some(n)
}
