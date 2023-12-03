use lazy_regex::{lazy_regex, Lazy, Regex};

use std::collections::HashSet;
use std::convert::TryFrom;

use super::{Component, ComponentKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Floor(HashSet<Component>);

impl Floor {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Component> {
        self.0.iter()
    }

    pub fn move_in(&mut self, components: impl IntoIterator<Item = Component>) {
        for component in components.into_iter() {
            self.0.insert(component);
        }
    }

    pub fn move_out<'a>(&mut self, components: impl IntoIterator<Item = &'a Component>) {
        for component in components.into_iter() {
            self.0.remove(component);
        }
    }

    pub fn is_safe(&self) -> bool {
        let unique_kinds = self
            .0
            .iter()
            .map(|component| component.kind)
            .collect::<HashSet<_>>();

        if unique_kinds.len() < 2 {
            return true;
        }

        self.0
            .iter()
            .filter(|component| component.kind == ComponentKind::Microchip)
            .all(|component| self.0.contains(&component.correspondent()))
    }
}

impl FromIterator<Component> for Floor {
    fn from_iter<I: IntoIterator<Item = Component>>(items: I) -> Self {
        Self(items.into_iter().collect())
    }
}

impl TryFrom<&str> for Floor {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        static RE: Lazy<Regex> = lazy_regex!(r"\ba (?<material>\w+)(-compatible)? (?<kind>\w+)\b");

        let components = RE
            .captures_iter(s)
            .map(|captures| {
                let material = &captures["material"];
                let kind = (&captures["kind"]).try_into()?;
                Ok::<_, anyhow::Error>(Component::new(material, kind))
            })
            .collect::<Result<HashSet<Component>, _>>()?;

        Ok(Self(components))
    }
}
