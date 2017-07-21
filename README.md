# README

Implementation of the [huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) compression algorithm in Rust

# Setup

Install the git pre-commit hook in order to run the unit tests before a commit:

  $ ln -s "$(pwd)/pre-commit.sh" .git/hooks/pre-commit

# Tree structure

```bash
let a = HuffmanNode::new_leaf('a', 1);
let b = HuffmanNode::new_leaf('b', 2);
let c = HuffmanNode::new_leaf('c', 4);

let ab = HuffmanNode::new_node(&a, &b);
let root = HuffmanNode::new_node(&ab, &c);

println!("{:#?}", root);
```

```bash
HuffmanNode {
    node_type: Node,
    key: ' ',
    value: 7,
    left: Some(
        HuffmanNode {
            node_type: Node,
            key: ' ',
            value: 3,
            left: Some(
                HuffmanNode {
                    node_type: Leaf,
                    key: 'a',
                    value: 1,
                    left: None,
                    right: None
                }
            ),
            right: Some(
                HuffmanNode {
                    node_type: Leaf,
                    key: 'a',
                    value: 1,
                    left: None,
                    right: None
                }
            )
        }
    ),
    right: Some(
        HuffmanNode {
            node_type: Node,
            key: ' ',
            value: 3,
            left: Some(
                HuffmanNode {
                    node_type: Leaf,
                    key: 'a',
                    value: 1,
                    left: None,
                    right: None
                }
            ),
            right: Some(
                HuffmanNode {
                    node_type: Leaf,
                    key: 'a',
                    value: 1,
                    left: None,
                    right: None
                }
            )
        }
    )
}
```
