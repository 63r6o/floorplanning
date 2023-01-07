use std::{fmt, collections::VecDeque};

use rand::{
    seq::SliceRandom,
    Rng,
};

struct Rectangle {
    name: i32,
    area: f32,
    width: f32,
    height: f32,
}

struct Module {
    name: i32,
    area: f32,
    width: f32,
    height: f32,
    rotatable: bool,
}

impl Module {
    pub fn new(name: i32, width: f32, height: f32, rotatable: bool) -> Self {
        Module {
            name,
            area: width * height,
            width,
            height,
            rotatable,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Element {
    Operand(i32),
    Operator(char),     // + -> horizontal (a+b = a under b)
                        // * -> vertical (a*b) = a on the left of b
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Operand(c) => write!(f, "{} ", c),
            Element::Operator(b) => write!(f, "{} ", b),
        }
    }
}

struct PolishExpression {
    elements: Vec<Element>,
}

impl fmt::Display for PolishExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pe_string: String = self.elements.iter().map(|e| e.to_string()).collect();
        write!(f, "{}", pe_string)
    }
}

impl PolishExpression {
    pub fn new(elements: Vec<Element>) -> Self {
        PolishExpression { elements }
    }

    pub fn m1(&mut self) {
        // get the index of two adjacent operands
        let (i, j) = self.get_random_operands();

        // swap them
        self.elements.swap(i, j)
    }

    pub fn m2(&mut self) {
        // get a random chain's starting index
        let mut i = *self.chain_starts().choose(&mut rand::thread_rng()).unwrap();
        // loop trough the chain and complement them
        loop {
            if i > self.elements.len() - 1 {
                break;
            }
            if let Element::Operator(b) = self.elements[i] {
                if b == '*' {
                    self.elements[i] = Element::Operator('+')
                } else {
                    self.elements[i] = Element::Operator('*')
                }
                i += 1;
            } else {
                break;
            }
        }
    }

    pub fn m3(&mut self) {
        // get the adjacent operand-operators
        let mut operand_operators = self.operand_operators();
        // swap them while keeping the polish expression normalised
        loop {
            // get a random operand-operator index from the list, and remove it (so we can't choose it twice)
            if operand_operators.is_empty() {
                break
            }
            let random = rand::thread_rng().gen_range(0..operand_operators.len());
            let i = operand_operators.remove(random);

            // swap them
            if self.is_skewed(i) && self.is_balloting(i) {
                self.elements.swap(i, i + 1);
                break;
            }
        }
    }

    fn is_skewed(&self, i: usize) -> bool {
        if !(self.elements[i + 1] == self.elements[i - 1]) {
            if i + 2 >= self.elements.len() {
                true
            } else {
                !(self.elements[i] == self.elements[i + 2])
            }
        } else {
            false
        }
    }

    fn is_balloting(&self, i: usize) -> bool {
        if let (Element::Operand(_), Element::Operator(_)) =
            (&self.elements[i], &self.elements[i + 1])
        {
            let mut d = 0;
            for j in 0..=i + 1 {
                match self.elements[j] {
                    Element::Operand(_) => (),
                    Element::Operator(_) => d += 1,
                }
            }
            2 * d <= i
        } else {
            true
        }
    }

    fn operand_operators(&self) -> Vec<usize> {
        self.elements
            .windows(2)
            .enumerate()
            .filter_map(|(i, neighbours)| match (&neighbours[0], &neighbours[1]) {
                (Element::Operand(_), Element::Operator(_))
                | (Element::Operator(_), Element::Operand(_)) => Some(i),
                _ => None,
            })
            .collect()
    }

    fn chain_starts(&self) -> Vec<usize> {
        self.elements
            .windows(2)
            .enumerate()
            .filter_map(|(i, neighbours)| match (&neighbours[0], &neighbours[1]) {
                (Element::Operand(_), Element::Operator(_)) => Some(i + 1),
                _ => None,
            })
            .collect()
    }

    fn operands(&self) -> Vec<usize> {
        self.elements
            .iter()
            .enumerate()
            .filter_map(|(i, element)| match element {
                Element::Operand(_) => Some(i),
                Element::Operator(_) => None,
            })
            .collect()
    }

    fn get_random_operands(&self) -> (usize, usize) {
        // get all the operands' indexes
        let operands: Vec<usize> = self.operands();
        // get a random adjacent pair
        let random_operand_index = rand::thread_rng().gen_range(0..operands.len() - 1);
        let i = operands.get(random_operand_index).unwrap();
        let j = operands.get(random_operand_index + 1).unwrap();

        (*i, *j)
    }
}

#[derive(PartialEq, Debug)]
struct SlicingTree {
    value: Element,
    left: Option<Box<SlicingTree>>,
    right: Option<Box<SlicingTree>>,
}

impl SlicingTree {
    pub fn new(value: Element, left: SlicingTree, right: SlicingTree) -> Self {
        SlicingTree {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    pub fn new_leaf(value: Element) -> Self {
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

    // create a tree from a polish expression
    pub fn build_from_polish_expression(polish_expression: &PolishExpression) -> SlicingTree {
        let elements = &polish_expression.elements;
        let mut stack: VecDeque<SlicingTree> = VecDeque::new();

        for i in 0..elements.len()  {
            match elements[i] {
                Element::Operand(_) => stack.push_back(SlicingTree::new_leaf(elements[i])),
                Element::Operator(_) => {
                    let right = stack.pop_back().unwrap();
                    let left = stack.pop_back().unwrap();
                    let value = elements[i];
                    let new_node = SlicingTree::new(value, left, right);
                    dbg!(&new_node);
                    stack.push_back(new_node);
                },
            }
        }

        stack.pop_back().unwrap()
    }

    pub fn build_polish_expression(&self) -> PolishExpression {
        let mut polish_vec = Vec::new();
        self.build_polish_expression_recursive(&self.left, &mut polish_vec);
        self.build_polish_expression_recursive(&self.right, &mut polish_vec);
        polish_vec.push(Some(self.value));
        dbg!(&polish_vec);
        PolishExpression::new(polish_vec.iter().filter_map(|x| *x).collect())
    }

    fn build_polish_expression_recursive(&self, root: &Option<Box<SlicingTree>>, polish_vec: &mut Vec<Option<Element>>) {
        match root {
            Some(node) => {
                self.build_polish_expression_recursive(&node.left, polish_vec);
                self.build_polish_expression_recursive(&node.right, polish_vec);
                polish_vec.push(Some(node.value));
            }
            None => (),
        }
    }

    // for debugging
    pub fn inorder_print(&self) {
        self.inorder_print_recursive(&self.left);
        self.inorder_print_recursive(&self.right);
        print!("{}", self.value);
        println!();
    }

    fn inorder_print_recursive(&self, root: &Option<Box<SlicingTree>>) {
        match root {
            Some(node) => {
                self.inorder_print_recursive(&node.left);
                self.inorder_print_recursive(&node.right);
                print!("{}", node.value);
            }
            None => (),
        }
    }
}

fn main() {
    let test_polish_vec = vec![
        Element::Operand(1),
        Element::Operand(2),
        Element::Operator('*'),
        Element::Operand(3),
        Element::Operator('+'),
        Element::Operand(4),
        Element::Operator('*'),
        Element::Operand(5),
        Element::Operator('+'),
    ];

    let _c = [[0.1f32; 5]; 5];

    let mut test_polish = PolishExpression::new(test_polish_vec);
    println!("{}", test_polish);
    test_polish.m1();
    println!("{}", test_polish);
    test_polish.m3();
    println!("{}", test_polish);
    test_polish.m3();
    println!("{}", test_polish);
    test_polish.m3();
    println!("{}", test_polish);
    test_polish.m3();
    println!("{}", test_polish);
    test_polish.m3();
    println!("{}", test_polish);

    // * = V vertical
    // + = H horizontal
    let test_tree_polish_vec = vec![
        Element::Operand(1),
        Element::Operand(2),
        Element::Operator('+'),
        Element::Operand(3),
        Element::Operand(4),
        Element::Operator('+'),
        Element::Operand(5),
        Element::Operator('+'),
        Element::Operator('*'),
        Element::Operand(6),
        Element::Operand(7),
        Element::Operator('+'),
        Element::Operand(8),
        Element::Operand(9),
        Element::Operator('+'),
        Element::Operand(10),
        Element::Operator('+'),
        Element::Operator('*'),
        Element::Operand(11),
        Element::Operand(12),
        Element::Operator('+'),
        Element::Operand(13),
        Element::Operand(14),
        Element::Operator('+'),
        Element::Operand(15),
        Element::Operator('+'),
        Element::Operator('*'),
        Element::Operator('*'),
        Element::Operator('*'),
    ];
    let test_tree_polish = PolishExpression::new(test_tree_polish_vec);
    println!("{}", test_tree_polish);
    let test_tree = SlicingTree::build_from_polish_expression(&test_tree_polish);
    dbg!(&test_tree);
    let test_output = test_tree.build_polish_expression();
    println!("{}", test_output);
}
