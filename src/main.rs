/*
 * main.rs - Console program to test tree_view.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

#![feature(allocator_api)]

use sicht::SichtMap;

mod tree_view;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TestMap {
    pub key: String,
    pub value: Vec<TestMap>,
}

impl tree_view::ToTreeView for TestMap {
    fn to_node(&self) -> tree_view::Node {
        tree_view::Node {
            node: self.key.clone(),
            children: self.value.iter().map(|v| v.to_node()).collect(),
        }
    }
}

impl tree_view::ToTreeView for String {
    fn to_node(&self) -> tree_view::Node {
        tree_view::Node {
            node: self.clone(),
            children: Vec::new(),
        }
    }
}

impl tree_view::ToTreeView for (&String, &Vec<String>) {
    fn to_node(&self) -> tree_view::Node {
        tree_view::Node {
            node: self.0.clone(),
            children: self.1.iter().map(|v| v.to_node()).collect(),
        }
    }
}

impl tree_view::ToTreeView for (String, SichtMap<String, String, Vec<String>>) {
    fn to_node(&self) -> tree_view::Node {
        tree_view::Node {
            node: self.0.clone(),
            children: self.1.iter().map(|v| v.to_node()).collect(),
        }
    }
}


fn main() {
    let mut tree0: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::new(),
    };
    tree0.value.push(TestMap {
        key: String::from("Leaf1"),
        value: Vec::new(),
    });
    tree0.value.push(TestMap {
        key: String::from("Node1"),
        value: Vec::from([
            TestMap {
                key: String::from("Leaf2"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Node2"),
                value: Vec::from([
                    TestMap {
                        key: String::from("Leaf3"),
                        value: Vec::new(),
                    },
                    TestMap {
                        key: String::from("Leaf4"),
                        value: Vec::new(),
                    },
                ]),
            },
        ]),
    });
    tree0.value.push(TestMap {
        key: String::from("Node3"),
        value: Vec::from([TestMap {
            key: String::from("Leaf5"),
            value: Vec::new(),
        }]),
    });
    tree0.value.push(TestMap {
        key: String::from("Node4"),
        value: Vec::from([TestMap {
            key: String::from("Leaf6"),
            value: Vec::new(),
        }]),
    });
    let tree1: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::new(),
    };
    let tree2: TestMap = TestMap {
        key: String::from("Root"),
        value: Vec::from([
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
            TestMap {
                key: String::from("Leaf"),
                value: Vec::new(),
            },
        ]),
    };

    let mut sicht0: SichtMap<String, String, Vec<String>> = SichtMap::new();
    sicht0.insert_with_both_keys(String::from("Key0"), String::from("Cokey0"), vec!(String::from("Value0")));
    sicht0.insert_with_both_keys(String::from("Key1"), String::from("Cokey1"), vec!(String::from("Value1")));
    sicht0.insert_with_both_keys(String::from("Key2"), String::from("Cokey2"), vec!(String::from("Value2")));
    sicht0.insert_with_both_keys(String::from("Key3"), String::from("Cokey3"), vec!(String::from("Value3")));
    sicht0.insert_with_both_keys(String::from("Key4"), String::from("Cokey4"), vec!(String::from("Value4")));
    sicht0.insert_with_both_keys(String::from("Key5"), String::from("Cokey5"), vec!(String::from("Value5")));
    println!("{:?}", sicht0);

    let view0: tree_view::TreeView<TestMap> = tree_view::TreeView::new(&tree0);
    let view1: tree_view::TreeView<TestMap> = tree_view::TreeView::new(&tree1);
    let view2: tree_view::TreeView<TestMap> = tree_view::TreeView::new(&tree2);
    let binding = (String::from("forklift"), sicht0);
    let view3: tree_view::TreeView<(String, SichtMap<String, String, Vec<String>>)> = tree_view::TreeView::new(&binding);
    println!("{:?}\n", tree0);
    println!("{}", view0);
    println!("{}", view1.print());
    println!("{}", view2.print());
    println!("{}", view3);

}
