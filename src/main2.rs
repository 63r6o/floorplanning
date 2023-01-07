// todo:
// 1. create the simulated annealing algorithm backbone
//      1. we need the slicing floorplan (initial)
//      2. cost fucntion (area + wire length)
//          here: area + gamma(?) totalWireLength
//      3. the rules
//      4. initial T
//      5. freezing T (check paper)
//      6. cooling schedule (e.g. T= 0.95 * T) check paper
//      pseudo code:
//      https://slideplayer.com/slide/5918848/
//      22:37
// 2. create the temperature calculator for that
// 3. create the representation of the tree
// 4. create the representation of the moves
// 5.create the normalized polish expression tree, and the checker for that
//
enum PE {
    Operand(usize),
    Operator(String)
}




fn get_initial_temp(s: Vec<String>) -> f32 {
    // TODO
    100000.0
}
fn simulated_annealing(s: Vec<String>) {
    let best = s.clone();
    let temp = s.clone();

    let t0 = get_initial_temp(temp);

    let mut T = t0;
    
    // not sure about them
    // let N = nmoves * m.size*()
    let mut totm = 0;   // total moves
    let mut uphill = 0; // uphill moves
    let mut reject = 0; // rejected moves
    
    let mut i = 0;
    

}
fn main() {
    println!("Hello, world!")//
}
