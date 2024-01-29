#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Command {
    pub index: usize,
    pub term: usize,
    pub content: String,
}
