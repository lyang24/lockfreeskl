use thiserror::Error;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};

use crate::{arena::Arena, node::Node, key::{Key}, MaxHeight};

#[derive(Error, Debug)]
pub enum SkipListError {
}

pub(crate) struct SkipList {
    arena: Arena,
    comparison_fn: Arc<dyn Fn(&[u8], &[u8]) -> i8>,
    head: &'static Node,
    tail: &'static Node,
    current_height: AtomicUsize,
}

struct Splice {
	prev: Arc<&'static Node>,
	next: Arc<&'static Node>,
}



struct Inserter {
    pub splices: [Splice; MaxHeight],
    pub height: usize,
}

impl Inserter {
    // pub fn set(mut self, level: usize, prev: &Node, next: &Node) {
    //     self.splices[level].prev = prev.into();
    //     self.splices[level].next = next.into();
    // } 
}

impl SkipList {
    fn get_current_height(&self) -> usize {
        self.current_height.load(Ordering::Relaxed)
    }

    fn get_next(&self, node: &'static Node, level: usize) -> &'static Node {
        let offset = node.tower[level].next_offset;
        unsafe { &*(self.arena.get_pointer(offset) as *const Node) }
    }

    fn key_is_after_node(&self, key: &Key, node: &'static Node) -> bool {
        todo!()
    }

    fn find_splice(&self, key: &Key, mut inserter: Inserter) -> bool {
        let mut found = false;
        let mut level = 0;
        let mut prev = self.head;
        let current_height = self.get_current_height();
        match inserter.height < current_height {
            true => {
                inserter.height = current_height;
                level = current_height;
            },
            false => {
                while level < current_height {
                    let splice = &inserter.splices[level];
                    if self.get_next(*splice.prev, level) != *splice.next {
                        level += 1;
                        continue
                    }
                    if *splice.prev != self.head && ! self.key_is_after_node(key, *splice.prev) {
                        level = current_height;
                        break;
                    }
                    if *splice.next != self.tail && self.key_is_after_node(key, *splice.next) {
                        level = current_height;
                        break;
                    }
                    prev = *splice.prev;
                    break;
                }
            }
        }
        while level >= 0 {
            let (prev, next, found) = self.find_splice_for_level(key, level, prev);
            // TODO point nil next to tail
            // inserter.set(level, prev, next);
            level -= 1;
        }
        found
    }

    fn find_splice_for_level(&self, key: &Key, level: usize, start: &'static Node) -> (&'static Node, &'static Node, bool) {
        let mut prev = start;
        let mut next = start;
        let mut found = false;
        loop {
            next = self.get_next(prev, level);
            if next == self.tail {
                break;
            }
            let offset = next.key_offset;
            let size = next.key_size;
            // minus eight bytes meta data.
            let actual_key_size = size - 8;
            let next_key = &self.arena.buf[offset..offset + size];
            let cmp = &(self.comparison_fn)(&key.user_key, &next_key[..actual_key_size]);
            if *cmp < 0 {
                break;
            }
            if *cmp == 0 {
                let next_trailer = u64::from_le_bytes(next_key[actual_key_size..].try_into().unwrap());
                if key.trailer == next_trailer {
                    // Internal key equality.
                    found = true;
                    break
                }
                if key.trailer > next_trailer {
                    // We are done for this level, since prev.key < key < next.key.
                    break
                }
            }
            prev = next;

        }
        (prev, next, found)
    }

}