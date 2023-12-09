use anyhow::anyhow;

use std::convert::TryFrom;

fn is_abba(chars: &[char]) -> bool {
    chars[0] == chars[3] && chars[1] == chars[2] && chars[0] != chars[1]
}

fn is_aba(chars: &[char]) -> bool {
    chars[0] == chars[2] && chars[0] != chars[1]
}

fn is_aba_bab(chars1: &[char], chars2: &[char]) -> bool {
    is_aba(chars1) && is_aba(chars2) && chars1[0] == chars2[1] && chars1[1] == chars2[0]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub chars: Vec<char>,
    pub in_brackets: bool,
}

impl Chunk {
    pub fn new(s: &str, in_brackets: bool) -> Self {
        Self {
            chars: s.chars().collect(),
            in_brackets,
        }
    }

    pub fn contains_abba(&self) -> bool {
        self.chars.windows(4).any(is_abba)
    }

    pub fn abas(&self) -> impl Iterator<Item = [char; 3]> + '_ {
        self.chars
            .windows(3)
            .filter_map(|window| is_aba(window).then_some([window[0], window[1], window[2]]))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(Vec<Chunk>);

impl Address {
    pub fn supports_tls(&self) -> bool {
        let mut found = false;
        for chunk in self.0.iter().filter(|chunk| chunk.contains_abba()) {
            if chunk.in_brackets {
                return false;
            } else {
                found = true;
            }
        }
        found
    }

    pub fn supports_ssl(&self) -> bool {
        let (inside, outside): (Vec<_>, Vec<_>) =
            self.0.iter().partition(|chunk| chunk.in_brackets);

        for chunk_outside in outside.iter() {
            for aba in chunk_outside.abas() {
                for chunk_inside in inside.iter() {
                    for bab in chunk_inside.abas() {
                        if is_aba_bab(&aba, &bab) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl TryFrom<&str> for Address {
    type Error = anyhow::Error;

    fn try_from(mut s: &str) -> anyhow::Result<Self> {
        let mut chunks = Vec::new();

        loop {
            if let Some(i) = s.find('[') {
                let (chunk, rest) = s.split_at(i);
                chunks.push(Chunk::new(chunk, false));
                if let Some(i) = rest.find(']') {
                    let (chunk, rest) = rest.split_at(i + 1);
                    chunks.push(Chunk::new(&chunk[1..chunk.len() - 1], true));
                    s = rest;
                } else {
                    return Err(anyhow!("unmatched opening bracket at index {}", i));
                }
            } else {
                if !s.is_empty() {
                    chunks.push(Chunk::new(s, false));
                }
                break;
            }
        }

        Ok(Self(chunks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        macro_rules! supported {
            ($input:expr) => {
                assert!(Address::try_from($input).unwrap().supports_tls());
            };
        }

        macro_rules! unsupported {
            ($input:expr) => {
                assert!(!Address::try_from($input).unwrap().supports_tls());
            };
        }

        supported!("abba[mnop]qrst");
        unsupported!("abcd[bddb]xyyx");
        unsupported!("aaaa[qwer]tyui");
        supported!("ioxxoj[asdfgh]zxcvbn");
    }

    #[test]
    fn test_supports_ssl() {
        macro_rules! supported {
            ($input:expr) => {
                assert!(Address::try_from($input).unwrap().supports_ssl());
            };
        }

        macro_rules! unsupported {
            ($input:expr) => {
                assert!(!Address::try_from($input).unwrap().supports_ssl());
            };
        }

        supported!("aba[bab]xyz");
        unsupported!("xyx[xyx]xyx");
        supported!("aaa[kek]eke");
        supported!("zazbz[bzb]cdb");
    }
}
