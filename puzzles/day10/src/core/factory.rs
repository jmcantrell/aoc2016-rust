use super::{Bot, Index, Instruction, Recipient, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transfer {
    pub origin: Index,
    pub values: [Value; 2],
}

#[derive(Debug, Clone)]
pub struct Factory {
    pub bots: Vec<Bot>,
    pub outputs: Vec<Vec<Value>>,
}

impl Factory {
    pub fn new(instructions: &[Instruction]) -> Self {
        let mut bots = Vec::new();

        for instruction in instructions {
            if let Instruction::Output {
                bot,
                recipient_low,
                recipient_high,
            } = instruction
            {
                bots.push((*bot, Bot::new(*recipient_low, *recipient_high)));
            }
        }

        bots.sort_by_key(|&(index, _)| index);

        let mut bots = bots.into_iter().map(|(_, bot)| bot).collect::<Vec<_>>();

        for instruction in instructions {
            if let Instruction::Input { value, bot } = instruction {
                bots[*bot].values.push(*value);
            }
        }

        let max_output_index = instructions
            .iter()
            .filter_map(|instruction| match instruction {
                Instruction::Output {
                    bot: _,
                    recipient_low,
                    recipient_high,
                } => match (recipient_low, recipient_high) {
                    (Recipient::Output(a), Recipient::Output(b)) => Some(std::cmp::max(*a, *b)),
                    (Recipient::Output(i), _) | (_, Recipient::Output(i)) => Some(*i),
                    _ => None,
                },
                _ => None,
            })
            .max();

        let outputs = match max_output_index {
            Some(i) => vec![Default::default(); i + 1],
            None => Default::default(),
        };

        Self { bots, outputs }
    }

    pub fn transfer(&mut self) -> Option<Transfer> {
        for origin in 0..self.bots.len() {
            if let Some(values) = self.bots[origin].give() {
                for (recipient, value) in self.bots[origin].recipients.into_iter().zip(values) {
                    match recipient {
                        Recipient::Bot(i) => self.bots[i].values.push(value),
                        Recipient::Output(i) => self.outputs[i].push(value),
                    }
                }

                return Some(Transfer { origin, values });
            }
        }

        None
    }
}
