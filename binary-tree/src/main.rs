enum BinaryTree<T> {
	Empty,
	NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
	element: T,
	left: BinaryTree<T>,
	right: BinaryTree<T>,
}

use self::BinaryTree::*;
use std::fmt::Display;



fn main() {

}
