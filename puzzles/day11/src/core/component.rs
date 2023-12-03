use anyhow::anyhow;

use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

fn compute_hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentKind {
    Generator,
    Microchip,
}

impl ComponentKind {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Generator => Self::Microchip,
            Self::Microchip => Self::Generator,
        }
    }
}

impl TryFrom<&str> for ComponentKind {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        match s {
            "generator" => Ok(Self::Generator),
            "microchip" => Ok(Self::Microchip),
            _ => Err(anyhow!("invalid component kind: {:?}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Component {
    pub material_hash: u64,
    pub kind: ComponentKind,
}

impl Component {
    pub fn new(material: &str, kind: ComponentKind) -> Self {
        Self {
            material_hash: compute_hash(material),
            kind,
        }
    }

    pub fn correspondent(&self) -> Self {
        Self {
            material_hash: self.material_hash,
            kind: self.kind.opposite(),
        }
    }
}
