// use std::{collections::HashMap, io::{stdin, BufReader, BufRead}, fs::File};
// use rand::{self, Rng};

// const H: &str = "H";
// const V: &str = "V";
// const nmoves: i32 = 10;
// const ratio: f32 = 0.85;
// const t0: f32 = -1.0;
// const lambdatf: f32 = 0.005;
// const iseed: i32 = 3;
// const epsilon: f32 = 0.001;
// const P: f32 = 0.99;

// struct Block {
//     name: String,
//     asp: Vec<(f32, f32)>,
// }

// struct NPE {
//     m: HashMap<String, (f32, f32)>,
// }

// impl NPE {
//     fn create_npe(&self, npe: &Vec<String>) -> Vec<Block> {
//         let mut npei = Vec::new();
//         for name in npe {
//             if name != "H" && name != "V" {
//                 let mut asp = vec![self.m[name]];
//                 let (w, h) = self.m[name];
//                 if w != h {
//                     let d1 = (h, w);
//                     asp.push(d1);
//                 }
//                 let b = Block {
//                     name: name.to_string(),
//                     asp,
//                 };
//                 npei.push(b);
//             } else {
//                 let c = Block {
//                     name: name.to_string(),
//                     asp: Vec::new(),
//                 };
//                 npei.push(c);
//             }
//         }
//         npei
//     }

//     fn do_stack(&self, i: &Block, j: &Block, k: &Block) {
//         for (w1, h1) in &i.asp {
//             for (w2, h2) in &j.asp {
//                 let (mut width, mut height) = (0.0, 0.0);
//                 if k.name == "H" {
//                     width = w1.max(*w2);
//                     height = h1 + h2;
//                 } else if k.name == "V" {
//                     width = w1 + w2;
//                     height = h1.max(*h2);
//                 }
//                 let aspt = (width, height);
//                 k.asp.push(aspt);
//             }
//         }
//     }

//     fn do_cost(&self, s: &Vec<String>) -> f32 {
//         let npe = self.create_npe(s);
//         let mut a = Vec::new();
//         for mut b in &npe {
//             if b.name != "H" && b.name != "V" {
//                 a.push(b);
//             } else {
//                 let s1 = a.pop().unwrap();
//                 let s2 = a.pop().unwrap();
//                 self.do_stack(s1, s2,  b);
//                 a.push(b);
//             }
//         }
//         let t = a.pop().unwrap();
//         let mut costarea = 1_000_000_000.0;
//         for (w, h) in &t.asp {
//             let area = w * h;
//             if area < costarea {
//                 costarea = area;
//             }
//         }
//         costarea
//     }

//     fn m1(s: &mut Vec<String>) {
//         let mut a = Vec::new();
//         let mut j = 0;
//         for i in 0..s.len() {
//             if s[i] != "H" && s[i] != "V" {
//                 a.push(i);
//                 j += 1;
//             }
//         }
//         let r = rand::thread_rng().gen_range(0..j - 1);
//         s.swap(a[r], a[r + 1]);
//     }

//     // Complements any operator chain.
//     fn m2(s: &mut Vec<String>) {
//         let mut a = Vec::new();

//         for i in 0..s.len() - 1 {
//             if s[i] != "H" && s[i] != "V" && (s[i + 1] == "H" || s[i + 1] == "V") {
//                 a.push(i + 1);
//             }
//         }

//         let r = rand::thread_rng().gen_range(0..a.len());
//         let mut k = a[r];

//         while k < s.len() {
//             if s[k] == "H" || s[k] == "V" {
//                 if s[k] == "H" {
//                     s[k] = "V".to_string();
//                 } else {
//                     s[k] = "H".to_string();
//                 }
//                 k += 1;
//             } else {
//                 break;
//             }
//         }
//     }

//     //swap two adjacent operator and operand
//     //
//     fn m3(s: &mut Vec<String>) {
//         let mut A = [0; 100];
//         let mut j = 0;

//         for i in 0..(s.len() - 1) {
//             // i denotes position of operand and i+1 position of operator
//             if (s[i] != "H" && s[i] != "V") && (s[i + 1] == "H" || s[i + 1] == "V") {
//                 A[j] = i; // inserts operand-operator position pair to array A where position number is the operand position
//                 j += 1;
//             }
//         }

//         loop {
//             let r = rand::random::<usize>() % (j - 1); // randomly picks an element number in A pointing to operand-operator position
//             let k = A[r]; // operand-operator position of the element number in A assigned to k

//             s.swap(k - 1, k); // swaps the operand and operator in vector s

//             if Self::ballot(s) && Self::skewed(s) {
//                 break;
//             }
//             // checks if after swapping balloting property is maintained and if the npe is skewed
//         }
//     }

//     fn ballot(s: &Vec<String>) -> bool {
//         let mut a = 0;
//         let mut b = 0;

//         for i in 0..s.len() {
//             if s[i] == "H" || s[i] == "V" {
//                 a += 1;
//             }

//             if s[i] != "H" && s[i] != "V" {
//                 b += 1;
//             }

//             if a >= b {
//                 return false;
//             }
//         }

//         return true;
//     }

//     fn skewed(s: &Vec<String>) -> bool {
//         for i in 1..s.len() {
//             if s[i] == "H" || s[i] == "V" {
//                 let j = s[i];
//                 if s[i - 1] == j {
//                     return false;
//                 }
//             }
//         }

//         return true;
//     }

//     fn initial_temp(&self, s: &mut Vec<String>) -> f32 {
//         let mut n = 0;
//         let mut k = 0;
//         let mut c = 0.0;
//         let oc = self.do_cost(s);
//         let mut nc = 0.0;
//         loop {
//             let mut d = 0.0;
//             let a = rand::random::<u8>() % 3 + 1;
//             match a {
//                 1 => Self::m1(s),
//                 2 => Self::m2(s),
//                 3 => Self::m3(s),
//                 _ => (),
//             }
//             nc = self.do_cost(s);
//             d = nc - oc;
//             if d > 0.0 {
//                 oc = nc;
//                 c += d;
//                 k += 1;
//             }
//             n += 1;
//             if n >= 40 {
//                 break;
//             }
//         }
//         let ac = c / k as f32;
//         let t = (t0 * ac) / f32::ln(P);
//         t
//     }

//     fn simulated_annealing(&self, s: &mut Vec<String>) {
//         let n = nmoves * self.m.len() as i32;
//         let mut best = s.clone();
//         let mut temp = s.clone();
//         let t0 = self.initial_temp(&mut &temp);
//         let mut t = t0;
//         let mut totm = 0;
//         let mut uphill = 0;
//         let mut reject = 0;
//         while reject as f32 / totm as f32 <= 0.95 && t >= epsilon {
//             totm = 0;
//             uphill = 0;
//             reject = 0;
//             while uphill < n && totm <= 2 * n {
//                 let oc = self.do_cost(&temp);
//                 let a = (rand::random::<u8>() % 3) + 1;
//                 match a {
//                     1 => NPE::m1(&mut temp),
//                     2 => NPE::m2(&mut temp),
//                     3 => NPE::m3(&mut temp),
//                     _ => unreachable!(),
//                 }
//                 totm += 1;
//                 let nc = self.do_cost(&temp);
//                 let d = nc - oc;
//                 let r = rand::random::<f32>();
//                 if d < 0.0 || r < (-d / t).exp() {
//                     if d > 0.0 {
//                         uphill += 1;
//                     }
//                     *s = temp.clone();
//                     if self.do_cost(s) < self.do_cost(&best) {
//                         best = s.clone();
//                     }
//                 } else {
//                     reject += 1;
//                 }
//             }
//             t = if t < lambdatf * t0 {
//                 0.1 * t
//             } else {
//                 ratio * t
//             };
//         }
//         *s = best;
//     }

//     fn print_npe(s: &Vec<String>) {
//         for i in s.iter() {
//             println!("{}", i);
//         }
//         println!("");
//     }
// }

// fn main() {
//     let mut m: HashMap<String, (f32, f32)> = HashMap::new();

//     println!("Enter input file");

//     let mut file_name = String::new();
//     stdin()
//         .read_line(&mut file_name)
//         .expect("Error reading file name");
//     let file_name = file_name.trim();

//     let file = File::open(file_name).expect("Error opening file");
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let parts: Vec<&str> = line.split_whitespace().collect();

//         if parts.len() > 0 {
//             let name = String::from(parts[0]);
//             let area: f32 = parts[1].parse().unwrap();
//             let artio: f32 = parts[2].parse().unwrap();
//             m.insert(name, (f32::sqrt(area * artio), f32::sqrt(area / artio)));
//         }
//     }

//     let npeo = NPE { m } ;
//     let mut npe = Vec::new();

//     let mut i = m.iter();

//     npe.push(i.next().unwrap().0.clone());

//     npe.push(i.next().unwrap().0.clone());

//     for j in i {
//         npe.push(V.to_string());
//         npe.push(j.0.clone());
//     }

//     npe.push(V.to_string());

//     println!("\n Initial Topology is ");
//     NPE::print_npe(&npe);

//     println!("\n Initial cost is {}", npeo.do_cost(&npe));

//     //rand::SeedableRng::seed_from_u64(iseed.try_into().unwrap());

//     npeo.simulated_annealing(&mut npe);

//     println!("\n Optimized Topology is ");
//     NPE::print_npe(&npe);

//     println!("\n Optimized cost is {}", npeo.do_cost(&npe));
// }
