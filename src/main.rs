use std::fmt;

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

#[derive(PartialEq, Debug)]
enum Element {
    Operand(i32),
    Operator(char),     // + -> horizontal (a+b = a under b)
                        // * -> vertical (a*b) = a on the left of b
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Operand(c) => write!(f, "{}", c),
            Element::Operator(b) => write!(f, "{}", b),
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
        // match (&self.elements[i], &self.elements[i + 1]) {
        //     (Element::Operand(_), Element::Operator(_)) => {
        //         let mut d = 1;
        //         for j in 0..i + 1 {
        //             match self.elements[j] {
        //                 Element::Operand(_) => (),
        //                 Element::Operator(_) => d += 1,
        //             }
        //         }
        //         2 * d < i
        //     }
        //     _ => true,
        // }
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
}
