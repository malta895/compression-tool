pub mod huffman {
    use std::{
        cmp::{Ordering, Reverse},
        collections::{BinaryHeap, HashMap},
    };

    enum Tree {
        Leaf {
            data: char,
            freq: u64,
        },
        Node {
            freq: u64,
            left: Box<Tree>,
            right: Box<Tree>,
        },
    }

    impl Tree {
        fn freq(&self) -> u64 {
            match self {
                Tree::Leaf { freq, .. } => *freq,
                Tree::Node { freq, .. } => *freq,
            }
        }
    }

    impl PartialOrd for Tree {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.freq().partial_cmp(&other.freq())
        }
    }

    impl Ord for Tree {
        fn cmp(&self, other: &Self) -> Ordering {
            self.freq().cmp(&other.freq())
        }
    }

    impl PartialEq for Tree {
        fn eq(&self, other: &Self) -> bool {
            self.freq().eq(&other.freq())
        }
    }

    impl Eq for Tree {}

    #[derive(Clone, Debug)]
    pub struct Symbol {
        pub data: Vec<bool>,
    }

    impl Symbol {
        fn new() -> Symbol {
            Symbol { data: vec![] }
        }

        fn append(&self, unit: bool) -> Symbol {
            Symbol {
                data: vec![self.data.clone(), vec![unit]].concat(),
            }
        }
    }

    pub fn encode(freq_table: &HashMap<char, u64>) -> Vec<(char, Symbol)> {
        let forest = freq_table
            .iter()
            .map(|(ch, freq)| Tree::Leaf {
                data: *ch,
                freq: *freq,
            })
            .collect::<Vec<Tree>>();

        let mut heap: BinaryHeap<Reverse<Tree>> = BinaryHeap::new();
        for tree in forest {
            heap.push(Reverse(tree));
        }

        let mut tree: Option<Tree> = None;

        loop {
            let Reverse(t1) = match heap.pop() {
                None => break,
                Some(t1) => t1,
            };

            let Reverse(t2) = match heap.pop() {
                None => {
                    tree = Some(t1);
                    break;
                }
                Some(t2) => t2,
            };

            let t = Tree::Node {
                freq: t1.freq() + t2.freq(),
                left: Box::new(t1),
                right: Box::new(t2),
            };

            heap.push(Reverse(t));
        }

        fn build_symbol_table(tree: Tree, path: Option<Symbol>) -> Vec<(char, Symbol)> {
            let path = path.unwrap_or(Symbol::new());
            match tree {
                Tree::Leaf { data, .. } => vec![(data, path)],
                Tree::Node { left, right, .. } => vec![
                    build_symbol_table(*left, Some(path.append(true))),
                    build_symbol_table(*right, Some(path.append(false))),
                ]
                .concat(),
            }
        }

        match tree {
            None => vec![],
            Some(t) => build_symbol_table(t, None),
        }
    }
}
