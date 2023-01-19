use std::{borrow::Borrow, cell::RefCell, collections::VecDeque, env, fmt, fs};

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
enum Cut {
    Vertical,   // *
    Horizontal, // +
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Element {
    Operand(Module),
    Operator(Cut),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Operand(module) => write!(f, "{} ", module.name),
            Element::Operator(cut) => match cut {
                Cut::Vertical => write!(f, "* "),
                Cut::Horizontal => write!(f, "+ "),
            },
        }
    }
}

#[derive(Clone)]
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

    // get the index of two adjacent operands
    // swap them
    pub fn m1(&mut self) {
        let (i, j) = self.get_random_operands();
        self.elements.swap(i, j)
    }

    // get a random chain's starting index
    // loop trough the chain and complement the elements
    pub fn m2(&mut self) {
        let mut i = *self.chain_starts().choose(&mut rand::thread_rng()).unwrap();
        while let Element::Operator(cut) = self.elements[i] {
            match cut {
                Cut::Vertical => self.elements[i] = Element::Operator(Cut::Horizontal),
                Cut::Horizontal => self.elements[i] = Element::Operator(Cut::Vertical),
            }
            if i == self.elements.len() - 1 {
                break;
            }
            i += 1;
        }
    }

    // get the adjacent operand-operators
    // swap them while keeping the polish expression normalised
    pub fn m3(&mut self) {
        let mut operand_operators = self.operand_operators();
        while !operand_operators.is_empty() {
            let random = rand::thread_rng().gen_range(0..operand_operators.len());
            let i = operand_operators.remove(random);

            if self.is_skewed(i) && self.is_balloting(i) {
                self.elements.swap(i, i + 1);
                break;
            }
        }
    }

    fn is_skewed(&self, i: usize) -> bool {
        match self.elements[i] {
            Element::Operand(_) => self.elements[i + 1] != self.elements[i - 1],
            Element::Operator(_) => self.elements[i] != self.elements[i + 2],
        }
    }

    fn is_balloting(&self, i: usize) -> bool {
        if matches!(
            (&self.elements[i], &self.elements[i + 1]),
            (Element::Operand(_), Element::Operator(_))
        ) {
            let d = self.elements[..=i + 1]
                .iter()
                .filter(|element| matches!(element, Element::Operator(_)))
                .count();

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

    // get all the operands' indexes
    // get a random adjacent pair from them
    fn get_random_operands(&self) -> (usize, usize) {
        let operands: Vec<usize> = self.operands();
        let random_operand_index = rand::thread_rng().gen_range(0..operands.len() - 1);
        (
            operands[random_operand_index],
            operands[random_operand_index + 1],
        )
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
                if let Element::Operand(module) = value {
                    if module.rotatable && module.width != module.height {
                        dimensions.push((module.width, module.height));
                        dimensions.push((module.height, module.width));
                    } else {
                        dimensions.push((module.width, module.height));
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
                let cut = match value {
                    Element::Operand(_) => unreachable!(),
                    Element::Operator(cut) => cut,
                };
                let dimensions = SlicingTree::get_dimensions(
                    left_dimensions.borrow(),
                    right_dimensions.borrow(),
                    cut,
                );
                RefCell::new(dimensions)
            },
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn get_dimensions(
        left_dimensions: &Vec<(f32, f32)>,
        right_dimensions: &Vec<(f32, f32)>,
        cut: Cut,
    ) -> Vec<(f32, f32)> {
        let mut dimensions: Vec<(f32, f32)> = Vec::new();
        for i in 0..left_dimensions.len() {
            for j in 0..right_dimensions.len() {
                let new_dimension = match cut {
                    Cut::Vertical => (
                        left_dimensions[i].0 + right_dimensions[j].0,
                        left_dimensions[i].1.max(right_dimensions[j].1),
                    ),
                    Cut::Horizontal => (
                        left_dimensions[i].0.max(right_dimensions[j].0),
                        left_dimensions[i].1 + right_dimensions[j].1,
                    ),
                };
                dimensions.push(new_dimension);
            }
        }
        dimensions
    }

    // create a tree from a polish expression
    pub fn build(polish_expression: &PolishExpression) -> SlicingTree {
        let elements = &polish_expression.elements;
        let mut stack: VecDeque<SlicingTree> = VecDeque::new();

        for element in elements {
            match element {
                Element::Operand(_) => stack.push_back(SlicingTree::new_leaf(*element)),
                Element::Operator(_) => {
                    let right = stack.pop_back().unwrap();
                    let left = stack.pop_back().unwrap();
                    let value = *element;
                    let new_node = SlicingTree::new_internal_node(value, left, right);
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
            if let Element::Operand(module) = left.as_ref().unwrap().value {
                left_most_center = (module.width / 2.0, module.height / 2.0)
            }
            left = &left.as_ref().unwrap().left
        }

        let mut right = &self.right;
        let mut right_most_center = (0f32, 0f32);
        while right.is_some() {
            if let Element::Operand(module) = right.as_ref().unwrap().value {
                right_most_center = (
                    area.0 - (module.width / 2.0),
                    area.1 - (module.height / 2.0),
                )
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

        alpha * (area / average_area) + (1.0 - alpha) * (hpwl / average_hpwl)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let module_inputs = fs::read_to_string(path).expect("Couldn't open file");

    let mut modules = Vec::new();
    for line in module_inputs.lines() {
        let input: Vec<&str> = line.split(", ").collect();
        let name = input[0].parse::<i32>().unwrap();
        let width = input[1].parse::<f32>().unwrap();
        let height = input[2].parse::<f32>().unwrap();
        let rotatable = input[3] == "true";
        modules.push(Module::new(name, width, height, rotatable));
    }

    let mut starter_expression_vec = Vec::new();
    for (i, module) in modules.iter().enumerate() {
        if i == 0 || i == 1 {
            starter_expression_vec.push(Element::Operand(*module));
        } else if i == 2 {
            starter_expression_vec.push(Element::Operator(Cut::Vertical));
            starter_expression_vec.push(Element::Operand(*module));
            starter_expression_vec.push(Element::Operator(Cut::Vertical));
        } else {
            starter_expression_vec.push(Element::Operand(*module));
            starter_expression_vec.push(Element::Operator(Cut::Vertical));
        }
    }
    let starter_expression = PolishExpression::new(starter_expression_vec);
    let starter_tree = SlicingTree::build(&starter_expression); // for the stats

    // get the average area and wirelength for the cost function
    let (average_area, average_hpwl) =
        get_averages(&mut starter_expression.clone(), modules.len().pow(2));

    // get the initial temperature
    let alpha = 0.7; // the importance of the area 
    let initial_temp = get_initial_temp(
        &mut starter_expression.clone(),
        alpha,
        average_area,
        average_hpwl,
    );

    // simulated annealing
    let r = 0.85; // temperature schedule
    let total_moves = modules.len() * 100; // k
    let mut best_expression = starter_expression.clone();
    let mut best_tree = SlicingTree::build(&best_expression);
    let mut best_cost = best_tree.get_cost(alpha, average_area, average_hpwl);
    let mut prev_expression = starter_expression.clone();
    let mut prev_tree = SlicingTree::build(&prev_expression);

    let mut rejected_moves = 0;
    let mut temp = initial_temp;

    while (rejected_moves as f32 / total_moves as f32) <= 0.95 && temp >= f32::EPSILON {
        for _ in 0..total_moves {
            let prev_cost = prev_tree.get_cost(alpha, average_area, average_hpwl);
            let mut temp_expression = prev_expression.clone();
            match rand::thread_rng().gen_range(1..4) {
                1 => temp_expression.m1(),
                2 => temp_expression.m2(),
                _ => temp_expression.m3(),
            }
            let tree = SlicingTree::build(&temp_expression);
            let cost = tree.get_cost(alpha, average_area, average_hpwl);
            let cost_dif = cost - prev_cost;

            let random = rand::thread_rng().gen_range(0.0..=1.0);

            if (cost_dif <= 0.0) || (random < -cost_dif / temp.powf(f32::EPSILON)) {
                prev_expression = temp_expression;
                prev_tree = SlicingTree::build(&prev_expression);
                if cost < best_cost {
                    best_cost = cost;
                    best_expression = prev_expression.clone();
                    best_tree = SlicingTree::build(&best_expression);
                }
            } else {
                rejected_moves += 1;
            }
        }
        temp *= r;
    }
    let best_area = best_tree.get_area_dims();
    let starter_area = starter_tree.get_area_dims();

    // display results
    println!("Starting expression: {}", starter_expression);
    println!(
        "Starting Width: {}\nStarting Height: {}",
        starter_area.0, starter_area.1
    );
    println!(
        "Starting floorplan area: {}",
        starter_area.0 * starter_area.1
    );
    println!("Starting wirelength: {}", starter_tree.get_hpwl());

    println!("{}", best_expression);
    println!("Width: {}\nHeight: {}", best_area.0, best_area.1);
    println!("Floorplan area: {}", best_area.0 * best_area.1);
    println!("Wirelength: {}", best_tree.get_hpwl());
}

fn get_initial_temp(
    pe: &mut PolishExpression,
    alpha: f32,
    average_area: f32,
    average_hpwl: f32,
) -> f32 {
    const P: f32 = 0.99;
    let mut previous_cost = 0.0;
    let mut uphill_moves = 0;
    let mut cost_difs = 0.0;
    for _ in 0..100 {
        match rand::thread_rng().gen_range(1..4) {
            1 => pe.m1(),
            2 => pe.m2(),
            _ => pe.m3(),
        }
        let tree = SlicingTree::build(&*pe);
        let cost = tree.get_cost(alpha, average_area, average_hpwl);
        let dif = cost - previous_cost;
        if dif > 0.0 {
            previous_cost = cost;
            cost_difs += dif;
            uphill_moves += 1;
        }
    }
    let average_cost = cost_difs / uphill_moves as f32;
    -average_cost / P.ln()
}

fn get_averages(pe: &mut PolishExpression, m: usize) -> (f32, f32) {
    let mut random_areas = Vec::new();
    let mut random_hpwls = Vec::new();
    for _ in 0..m {
        match rand::thread_rng().gen_range(1..4) {
            1 => pe.m1(),
            2 => pe.m2(),
            _ => pe.m3(),
        }
        let tree = SlicingTree::build(&*pe);
        random_areas.push(tree.get_area());
        random_hpwls.push(tree.get_hpwl());
    }
    let average_area: f32 = random_areas.iter().sum::<f32>() / random_areas.len() as f32;
    let average_hpwl = random_hpwls.iter().sum::<f32>() / random_hpwls.len() as f32;
    (average_area, average_hpwl)
}
