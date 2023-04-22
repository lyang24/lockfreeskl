pub mod arena;
pub mod node;
pub mod links;
pub mod key;
pub mod skiplist;

const MaxHeight: usize = 20;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
