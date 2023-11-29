fn digits_to_usize(chars: impl Iterator<Item = char>, radix: u32) -> Option<usize> {
    let mut result = 0;

    for digit in chars {
        result *= 10;
        result += digit.to_digit(radix)? as usize;
    }

    Some(result)
}

pub fn decompressed_size(mut chars: impl Iterator<Item = char>, recurse: bool) -> usize {
    let mut count = 0;

    loop {
        let maybe_c = chars.next();

        if maybe_c.is_none() {
            break;
        }

        let c = maybe_c.unwrap();

        if c == '(' {
            let len = digits_to_usize((&mut chars).take_while(|&c| c != 'x'), 10).unwrap();
            let repeat = digits_to_usize((&mut chars).take_while(|&c| c != ')'), 10).unwrap();

            let inner_chars = (&mut chars).take(len).collect::<Vec<_>>();

            let inner_size = if recurse {
                decompressed_size(inner_chars.into_iter(), recurse)
            } else {
                len
            };

            count += inner_size * repeat;
        } else if !c.is_whitespace() {
            count += 1;
        }
    }

    count
}
