// outdated info, maybe I'll need it in the future:
// hard-coding Element for some reason, it's not really necessary I think
#[derive(PartialEq)]
struct SlicingTree {
    value: Element,
    left: Option<Box<SlicingTree>>,
    right: Option<Box<SlicingTree>>,
}

impl SlicingTree {
    pub fn new(value: Element) -> Self {
        SlicingTree {
            value,
            left: None,
            right: None,
        }
    }

    // manual insertion for testing purposes
    pub fn left(mut self, node: SlicingTree) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    // manual insertion for testing purposes
    pub fn right(mut self, node: SlicingTree) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    // TODO insert method with inorder traversal
    pub fn build_from_expression() {}

    pub fn inorder_print(&self) {
        self.inorder_print_recursive(&self.left);
        print!("{}", self.value);
        self.inorder_print_recursive(&self.right);
        println!();
    }

    fn inorder_print_recursive(&self, root: &Option<Box<SlicingTree>>) {
        match root {
            Some(node) => {
                self.inorder_print_recursive(&node.left);
                print!("{}", node.value);
                self.inorder_print_recursive(&node.right)
            }
            None => (),
        }
    }
}

fn main() {
    println!("Hello world!");
    let slicing_tree = SlicingTree::new(Element::Operator('*'))
        .left(
            SlicingTree::new(Element::Operator('+'))
                .left(SlicingTree::new(Element::Operand(1)))
                .right(SlicingTree::new(Element::Operand(2))),
        )
        .right(
            SlicingTree::new(Element::Operator('+'))
                .left(
                    SlicingTree::new(Element::Operator('+'))
                        .left(SlicingTree::new(Element::Operand(3)))
                        .right(SlicingTree::new(Element::Operand(4))),
                )
                .right(SlicingTree::new(Element::Operand(5))),
        );
    slicing_tree.inorder_print();
}