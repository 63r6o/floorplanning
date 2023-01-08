use std::{cell::RefCell, collections::VecDeque, fmt};

use rand::{seq::SliceRandom, Rng};

#[derive(PartialEq, Debug, Clone, Copy)]
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
    Operand(Module),
    Operator(char), // + -> horizontal (a+b = a under b)
                    // * -> vertical (a*b) = a on the left of b
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Operand(c) => write!(f, "{} ", c.name),
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
                break;
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
    dimensions: RefCell<Vec<(f32, f32)>>,
    left: Option<Box<SlicingTree>>,
    right: Option<Box<SlicingTree>>,
}

impl SlicingTree {
    pub fn new_leaf(value: Element) -> Self {
        SlicingTree {
            value,
            dimensions: {
                let mut dimensions = Vec::new();
                if let Element::Operand(c) = value {
                    if c.rotatable && c.width != c.height {
                        dimensions.push((c.width, c.height));
                        dimensions.push((c.height, c.width));
                    } else {
                        dimensions.push((c.width, c.height));
                    }
                }
                RefCell::new(dimensions)
            },
            left: None,
            right: None,
        }
    }

    pub fn new_internal_node(value: Element, left: SlicingTree, right: SlicingTree) -> Self {
        SlicingTree {
            value,
            dimensions: {
                let left_dimensions = left.dimensions.borrow();
                let right_dimensions = right.dimensions.borrow();
                let mut dimensions: Vec<(f32, f32)> = Vec::new();
                let b = match value {
                    Element::Operand(_) => unreachable!(),
                    Element::Operator(b) => b,
                };
                for i in 0..left_dimensions.len() {
                    for j in 0..right_dimensions.len() {
                        if b == '*' {
                            let new_dimension = (
                                left_dimensions[i].0 + right_dimensions[j].0,
                                left_dimensions[i].1.max(right_dimensions[j].1),
                            );
                            // TODO pruning acording to the book
                            //dimensions.retain(|&dim| (dim.0 >= new_dimension.0) && (dim.1 >= new_dimension.1));
                            dimensions.push(new_dimension);
                        } else if b == '+' {
                            let new_dimension = (
                                left_dimensions[i].0.max(right_dimensions[j].0),
                                left_dimensions[i].1 + right_dimensions[j].1,
                            );
                            // TODO pruning acording to the book
                            //dimensions.retain(|&dim| (dim.0 >= new_dimension.0) && (dim.1 >= new_dimension.1));
                            dimensions.push(new_dimension);
                        }
                    }
                }
                RefCell::new(dimensions)
            },
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    // create a tree from a polish expression
    pub fn build_from_polish_expression(polish_expression: &PolishExpression) -> SlicingTree {
        let elements = &polish_expression.elements;
        let mut stack: VecDeque<SlicingTree> = VecDeque::new();

        for i in 0..elements.len() {
            match elements[i] {
                Element::Operand(_) => stack.push_back(SlicingTree::new_leaf(elements[i])),
                Element::Operator(_) => {
                    let right = stack.pop_back().unwrap();
                    let left = stack.pop_back().unwrap();
                    let value = elements[i];
                    // test
                    let new_node = SlicingTree::new_internal_node(value, left, right);
                    //let new_node = SlicingTree::new(value, left, right);
                    stack.push_back(new_node);
                }
            }
        }

        stack.pop_back().unwrap()
    }

    pub fn get_pin_positions(&self, area: (f32, f32)) -> (f32, f32, f32, f32) {
        let mut left = &self.left;
        let mut left_most_center = (0f32, 0f32);
        while left.is_some() {
            match left.as_ref().unwrap().value {
                Element::Operand(c) => left_most_center = (c.width / 2.0, c.height / 2.0),
                Element::Operator(_) => (),
            }
            left = &left.as_ref().unwrap().left
        }

        let mut right = &self.right;
        let mut right_most_center = (0f32, 0f32);
        while right.is_some() {
            match right.as_ref().unwrap().value {
                Element::Operand(c) => {
                    right_most_center = (area.0 - (c.width / 2.0), area.1 - (c.height / 2.0));
                }
                Element::Operator(_) => (),
            }
            right = &right.as_ref().unwrap().right
        }

        (
            right_most_center.0,
            left_most_center.0,
            right_most_center.1,
            left_most_center.1,
        )
    }

    fn get_hpwl(&self) -> f32 {
        let area_dims = self.get_area_dims();
        let (max_x, min_x, max_y, min_y) = self.get_pin_positions(area_dims);
        (max_x - min_x) + (max_y - min_y)
    }

    fn get_area_dims(&self) -> (f32, f32) {
        self.dimensions
            .borrow_mut()
            .sort_by(|a, b| (b.0 * b.1).partial_cmp(&(a.0 * a.1)).unwrap());
        *self.dimensions.borrow().last().unwrap()
    }

    fn get_area(&self) -> f32 {
        let area_dims = self.get_area_dims();
        area_dims.0 * area_dims.1
    }

    pub fn get_cost(&self, alpha: f32, average_area: f32, average_hpwl: f32) -> f32 {
        let area = self.get_area();
        let hpwl = self.get_hpwl();

        alpha*(area/average_area) + (1.0-alpha)*(hpwl/average_hpwl)
    }
}

fn main() {
    let modules = vec![
        Module::new(1, 4.0, 6.1, true),
        Module::new(2, 4.0, 4.2, true),
        Module::new(3, 3.0, 4.3, true),
        Module::new(4, 4.0, 4.4, true),
        Module::new(5, 3.0, 4.5, true),
    ];

    let test_polish_vec = vec![
        Element::Operand(modules[0]),
        Element::Operand(modules[1]),
        Element::Operator('+'),
        Element::Operand(modules[2]),
        Element::Operand(modules[3]),
        Element::Operator('+'),
        Element::Operand(modules[4]),
        Element::Operator('+'),
        Element::Operator('*'),
    ];

    let mut test_polish = PolishExpression::new(test_polish_vec);

    let mut random_areas = Vec::new();
    let mut random_hpwls = Vec::new();
    for _ in 0..modules.len() {
        match rand::thread_rng().gen_range(1..4) {
            1 => test_polish.m1(),
            2 => test_polish.m2(),
            _ => test_polish.m3(),
        }
        let tree = SlicingTree::build_from_polish_expression(&test_polish);
        random_areas.push(tree.get_area());
        random_hpwls.push(tree.get_hpwl());
    }
    let average_area: f32 = random_areas.iter().sum::<f32>() / random_areas.len() as f32;
    let average_hpwl = random_hpwls.iter().sum::<f32>() / random_hpwls.len() as f32;
    let tree = SlicingTree::build_from_polish_expression(&test_polish);
    dbg!(&tree.get_cost(0.5, average_area, average_hpwl));
    println!("{}", average_area);
    println!("{}", average_hpwl);
    println!("{}", test_polish);
    
}
