use algorithms::data_structures::trie::{TrieNode, TriePath};
use rstest::rstest;

#[rstest]
#[case::branching_paths(
    vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e'],
        vec!['a', 'b', 'd'],
    ],
    vec![
        TriePath::new(vec!['a'], 3),
        TriePath::new(vec!['a', 'b'], 2),
        TriePath::new(vec!['a', 'b', 'c'], 1),
        TriePath::new(vec!['a', 'b', 'd'], 1),
        TriePath::new(vec!['a', 'e'], 1),
    ]
)]
#[case::duplicate_sequences_increment_support(
    vec![
        vec!['a'],
        vec!['a'],
    ],
    vec![
        TriePath::new(vec!['a'], 2),
    ]
)]
#[case::separate_roots_are_sorted(
    vec![
        vec!['c'],
        vec!['a'],
        vec!['b'],
    ],
    vec![
        TriePath::new(vec!['a'], 1),
        TriePath::new(vec!['b'], 1),
        TriePath::new(vec!['c'], 1),
    ]
)]
#[case::empty_input(
    Vec::<Vec<char>>::new(),
    Vec::<TriePath<char>>::new()
)]
#[case::empty_sequence_is_ignored(
    vec![
        vec![],
        vec!['a'],
    ],
    vec![
        TriePath::new(vec!['a'], 1),
    ]
)]
fn test_insert_and_iter(
    #[case] sequences: Vec<Vec<char>>,
    #[case] expected_paths: Vec<TriePath<char>>,
) {
    let mut root = TrieNode::new();
    for sequence in sequences {
        root.insert(&sequence);
    }

    let paths: Vec<_> = root.paths().collect();
    assert_eq!(paths, expected_paths);
}
