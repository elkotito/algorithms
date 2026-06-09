use algorithms::data_structures::trie::{TrieNode, TriePath};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_iter() {
        let mut root = TrieNode::new(0);
        root.insert(&['a', 'b', 'c']);
        root.insert(&['a', 'e']);
        root.insert(&['a', 'b', 'd']);

        let paths: Vec<_> = root.paths().collect();

        assert_eq!(
            paths,
            vec![
                TriePath::new(vec!['a'], 3),
                TriePath::new(vec!['a', 'b'], 2),
                TriePath::new(vec!['a', 'b', 'c'], 1),
                TriePath::new(vec!['a', 'b', 'd'], 1),
                TriePath::new(vec!['a', 'e'], 1),
            ]
        );
    }
}
