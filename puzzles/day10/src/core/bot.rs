use super::{Recipient, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot {
    pub recipients: [Recipient; 2],
    pub values: Vec<Value>,
}

impl Bot {
    pub fn new(recipient_low: Recipient, recipient_high: Recipient) -> Self {
        Self {
            recipients: [recipient_low, recipient_high],
            values: Default::default(),
        }
    }

    pub fn give(&mut self) -> Option<[Value; 2]> {
        if self.values.len() >= 2 {
            let a = self.values.pop().unwrap();
            let b = self.values.pop().unwrap();
            Some([std::cmp::min(a, b), std::cmp::max(a, b)])
        } else {
            None
        }
    }
}
