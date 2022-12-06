struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

#[derive(Debug)]
enum Tree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

impl<T: Ord> Tree<T> {
    fn add(&mut self, value: T){
        match *self {
            Tree::Empty => {
                *self = Tree::NonEmpty(Box::new(TreeNode{
                    element: value,
                    left: Tree::Empty,
                    right: Tree::Empty,
                }))
            }
            Tree::NonEmpty(ref mut node) => {
                if value < node.element {
                    node.left.add(value);
                }else{
                    node.right.add(value);
                }
            }
        }
    }
}

use self::Tree::*;

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: Tree<T>,
    right: Tree<T>
}

struct TreeIterator<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIterator<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a Tree<T>) {
        while let NonEmpty(ref node) = *tree { self.unvisited.push(node);
            tree = &node.left;
        } 
    }
}

impl<T> Tree<T> {
    fn iter(&self) -> TreeIterator<T> {
        let mut iter = TreeIterator {unvisited: Vec::new()};
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a Tree<T> {
    type Item = &'a T;
    type IntoIter = TreeIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIterator<'a, T> { 
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // Find the node this iteration must produce,
        // or finish the iteration. (Use the `?` operator // to return immediately if it's `None`.)
        let node = self.unvisited.pop()?;
        // After `node`, the next thing we produce must be the leftmost // child in `node`'s right subtree, so push the path from here // down. Our helper method turns out to be just what we need. self.push_left_edge(&node.right);
        // Produce a reference to this node's value.
        Some(&node.element)
    }
}

fn main() {
    for k in (I32Range{start: 0, end: 14}){
        println!("{}", k);
    }

    let mut tree = Tree::Empty;
    tree.add("jaeger"); 
    tree.add("robot"); 
    tree.add("droid"); 
    tree.add("mecha");

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }
    println!("{:?}", tree);
    println!("{:?}", v);
}
