fn manual_n<const N: usize>(n: [&u8; N]) -> u32 {
    n.iter()
        .map(|&&x| (x - b'0') as u32)
        .fold(0, |acc, x| acc * 10 + x)
}
pub fn run(i: &str) -> u32 {
    let mut i = i.as_bytes();
    let mut sum = 0;
    while let Some(idx) = memchr::memchr(b'm', i) {
        i = unsafe { &i.get_unchecked(idx + 1..) };
        match i {
            [b'u', b'l', b'(', rest @ ..] => {
                macro_rules! cases {
                    ($([$($i:ident)+,$($t:ident)+])+) => {
                        match rest {
                            $(
                                [$($i @ b'0'..=b'9'),+, b',', $($t @ b'0'..=b'9'),+, b')', rest @ ..] => {
                                    (manual_n([$($i),+]) * manual_n([$($t),+]), rest)
                                }
                            )+
                            _ => (0, i),
                        }

                    };
                }
                let (res, rest) = cases!(
                    [a b c , d e f]
                    [a b c , d e]
                    [a b c , d]
                    [a b , d e f]
                    [a b , d e]
                    [a b , d]
                    [a , d e f]
                    [a , d e]
                    [a , d]
                );
                sum += res;
                i = rest;
            }
            _ => {}
        }
    }

    sum
}
