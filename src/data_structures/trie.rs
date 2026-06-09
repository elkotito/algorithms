use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TrieNode<T> {
    count: usize,
    children: BTreeMap<T, TrieNode<T>>,
}

impl<T> TrieNode<T> {
    pub fn new(count: usize) -> Self {
        Self {
            count: count,
            children: BTreeMap::new(),
        }
    }
}

impl<T: Clone + Ord> TrieNode<T> {
    pub fn insert(&mut self, sequence: &[T]) {
        let mut current = self;
        for item in sequence {
            current = current
                .children
                .entry(item.clone())
                .and_modify(|e| e.count += 1)
                .or_insert_with(|| TrieNode::new(1));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TriePath<T> {
    pub support: usize,
    pub sequence: Vec<T>,
}

impl<T> TriePath<T> {
    pub fn new(sequence: Vec<T>, support: usize) -> Self {
        Self { sequence, support }
    }
}

pub struct TriePathIter<'a, T> {
    stack: Vec<(&'a TrieNode<T>, Vec<T>)>,
}

impl<'a, T: Clone> Iterator for TriePathIter<'a, T> {
    type Item = TriePath<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, path)) = self.stack.pop() {
            for (key, child) in node.children.iter().rev() {
                let mut new_path = path.clone();
                new_path.push(key.clone());
                self.stack.push((child, new_path));
            }

            if !path.is_empty() {
                return Some(TriePath::new(path, node.count));
            }
        }

        None
    }
}

impl<T: Clone> TrieNode<T> {
    pub fn paths(&self) -> TriePathIter<'_, T> {
        TriePathIter {
            stack: vec![(self, Vec::new())],
        }
    }
}
