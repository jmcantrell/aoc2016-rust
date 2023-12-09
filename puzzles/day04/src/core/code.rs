use anyhow::Context;
use lazy_regex::{lazy_regex, Lazy, Regex};

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomCode<'a> {
    pub name_encrypted: &'a str,
    pub sector_id: usize,
    pub checksum: &'a str,
}

impl RoomCode<'_> {
    pub fn is_real(&self) -> bool {
        let counts: HashMap<char, usize> =
            self.name_encrypted
                .chars()
                .filter(|&c| c != '-')
                .fold(HashMap::new(), |mut acc, c| {
                    *acc.entry(c).or_default() += 1;
                    acc
                });

        let mut freq: BinaryHeap<(usize, Reverse<char>)> =
            counts.into_iter().map(|(c, n)| (n, Reverse(c))).collect();

        let mut checksum = String::new();

        for _ in 0..self.checksum.len() {
            checksum.push(freq.pop().unwrap().1 .0);
        }

        checksum == self.checksum
    }

    pub fn decrypt_name(&self) -> String {
        let n = 26u8;
        let offset = 97u8;
        let shift = (self.sector_id % n as usize) as u8;
        let mut name = String::new();

        for c in self.name_encrypted.chars() {
            if c == '-' {
                name.push(' ');
            } else {
                let i = (c as u8) - offset;
                let j = (i + shift) % n + offset;
                name.push(j as char);
            }
        }

        name
    }
}

impl<'a> TryFrom<&'a str> for RoomCode<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> anyhow::Result<Self> {
        static RE: Lazy<Regex> =
            lazy_regex!(r"^(?<name_encrypted>[a-z-]+)-(?<sector_id>\d+)\[(?<checksum>[a-z]+)\]$");

        let captures = RE
            .captures(s)
            .with_context(|| format!("expected input to match: {:?}", RE.as_str()))?;

        let name_encrypted = captures.name("name_encrypted").unwrap().as_str();
        let sector_id = captures["sector_id"].parse()?;
        let checksum = captures.name("checksum").unwrap().as_str();

        Ok(Self {
            name_encrypted,
            sector_id,
            checksum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real() {
        macro_rules! assert_real {
            ($input:expr) => {
                assert!(RoomCode::try_from($input).unwrap().is_real());
            };
        }
        macro_rules! assert_not_real {
            ($input:expr) => {
                assert!(!RoomCode::try_from($input).unwrap().is_real());
            };
        }

        assert_real!("aaaaa-bbb-z-y-x-123[abxyz]");
        assert_real!("a-b-c-d-e-f-g-h-987[abcde]");
        assert_real!("not-a-real-room-404[oarel]");

        assert_not_real!("totally-real-room-200[decoy]");
    }

    #[test]
    fn test_decrypt_name() {
        macro_rules! test {
            ($input:expr => $expected:expr) => {
                assert_eq!(
                    RoomCode::try_from($input).unwrap().decrypt_name(),
                    $expected
                );
            };
        }

        test!("qzmt-zixmtkozy-ivhz-343[xxxxx]" => "very encrypted name");
    }
}
