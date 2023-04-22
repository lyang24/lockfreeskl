use crate::{MaxHeight, links::Links};

#[derive(PartialEq, Eq)]
pub(crate) struct Node {
    pub key_offset: usize,
    pub key_size: usize,
    pub value_size: usize,

    pub tower: [Links; MaxHeight]
}