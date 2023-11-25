use md5::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoorID<'a>(&'a str);

impl DoorID<'_> {
    pub fn relevant_hashes(&self) -> impl Iterator<Item = String> {
        let mut context = Context::new();
        context.consume(self.0.as_bytes());

        (0..)
            .map(move |index: usize| {
                let mut context = context.clone();
                context.consume(index.to_string().as_bytes());
                context.compute()
            })
            .map(|digest| format!("{:x}", digest))
            .filter(|s| s.starts_with("00000"))
    }
}

impl<'a> From<&'a str> for DoorID<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}
