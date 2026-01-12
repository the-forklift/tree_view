/*
 * tree_view.rs - Library to view structures in a tree view.
 *
 * (C) 2020 Tim Gravert <tim.gravert@web.de>
 *
 * License: MIT OR Apache-2.0
 *
 */

use std::fmt::{Display, Formatter, Result};

const INDENT_FREE: &str = "│   ";
const INDENT_NODE: &str = "├── ";
const INDENT_END: &str = "└── ";
const INDENT_EMPTY: &str = "    ";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Node {
    pub node: String,
    pub children: Vec<Node>,
}

pub trait ToTreeView {
    fn to_node(&self) -> Node;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TreeView<'a, T>
where
    T: ToTreeView,
{
    pub original: &'a T,
    pub root: Node,
}

#[allow(unused)]
impl Node {
    pub fn new() -> Self {
        Node {
            node: String::from(""),
            children: Vec::new(),
        }
    }

    pub fn from(n: Node) -> Self {
        Node {
            node: n.node,
            children: n.children,
        }
    }

    pub fn insert(mut self, n: Node) {
        self.children.push(n);
    }

    pub fn print_node(self, pre: &str, first: bool, last: bool) -> String {
        let node: &str = self.node.as_str();
        let n = self.children.len();

        let mut output: String = String::from("");
        let mut pre: String = String::from(pre);

        if last {
            output = format!("{pre}{INDENT_END}") + node + "\n";
            pre = format!("{pre}{INDENT_EMPTY}");
        } else if first {
            output = output + node + "\n";
        } else {
            output = format!("{pre}{INDENT_NODE}") + node + "\n";
            pre = format!("{pre}{INDENT_FREE}");
        }

        //output = output + &c.print_node(depth + 1, &format!("{pre}{node}"));
        for (i, c) in self.children.into_iter().enumerate() {
            output = output + &c.print_node(pre.as_str(), false, i == n - 1);
        }

        output
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> TreeView<'a, T>
where
    T: ToTreeView,
{
    pub fn new(t: &'a T) -> Self {
        TreeView {
            original: t,
            root: t.to_node(),
        }
    }

    pub fn print(self) -> String {
        self.root.print_node("", true, false)
    }
}

impl<T> Display for TreeView<'_, T>
where
    T: ToTreeView,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let output = self.root.clone().print_node("", true, false);
        write!(f, "{}", output)
    }
}
