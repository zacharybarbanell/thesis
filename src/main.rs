use std::{cmp::Ordering, iter::zip};

struct CoefficientState<const W: usize> {
    values: [isize; W],
    ω: usize,
}

impl<const W: usize> CoefficientState<W> {
    pub fn new(ω: usize) -> CoefficientState<W> {
        assert!(W != 0);
        assert!(ω != 0);
        CoefficientState { values: [0; W], ω }
    }
}

impl<const W: usize> Iterator for CoefficientState<W> {
    type Item = [isize; W];

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.values.clone();
        let total: isize = self.values.iter().map(|x| x.abs()).sum();
        match total.cmp(&(self.ω as isize)) {
            Ordering::Greater => {
                return None;
            }
            Ordering::Less => {
                if self.values[0].is_negative() {
                    self.values[0] = -self.values[0];
                } else {
                    self.values[0] = -1 - self.values[0];
                }
            }
            Ordering::Equal => {
                let mut i = 0;
                loop {
                    match self.values[i].cmp(&0) {
                        Ordering::Less => {
                            self.values[i] = -self.values[i];
                            break;
                        }
                        Ordering::Equal => i += 1,
                        Ordering::Greater => {
                            if i == W - 1 {
                                self.values[i] += 1; //make future invocations return None
                            } else {
                                self.values[i] = 0;
                                if self.values[i + 1].is_negative() {
                                    self.values[i + 1] = -self.values[i + 1];
                                } else {
                                    self.values[i + 1] = -1 - self.values[i + 1];
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
        return Some(val);
    }
}

struct WeightState<const W: usize> {
    values: [usize; W],
    cap: usize,
}

impl<const W: usize> WeightState<W> {
    pub fn new(cap: usize) -> WeightState<W> {
        assert!(W != 0);
        let mut val = WeightState { values: [0; W], cap };
        for i in 0..W {
            val.values[i] = i + 1;
        }
        return val;
    }
}

impl<const W: usize> Iterator for WeightState<W> {
    type Item = [usize; W];

    fn next(&mut self) -> Option<Self::Item> {
        if self.values[W - 1] >= self.cap {
            return None;
        }
        let val = self.values;

        for i in 0..W {
            if i == W - 1 || self.values[i + 1] - self.values[i] > 1 {
                self.values[i] += 1;
                for j in 0..i {
                    self.values[j] = j + 1;
                }
                break;
            }
        }

        return Some(val);
    }
}

fn brute_force_gadgets<const W: usize>(ω: usize) -> (usize, Vec<[usize; W]>) {
    assert!(W != 0);
    assert!(ω != 0);

    let upper_bound = 2 * (ω + 1).pow(u32::try_from(W).unwrap() - 1);

    let mut best_length = 0;
    let mut best_weights = vec![];

    let mut results = vec![];
    for weights in WeightState::<W>::new(upper_bound) {
        results.clear();
        results.resize(best_length + 1, false);
        'a: loop {
            for coefficients in CoefficientState::<W>::new(ω) {
                let result: isize = zip(weights, coefficients)
                    .map(|(x, y)| isize::try_from(x).unwrap() * y)
                    .sum();
                if 0 <= result && result < results.len().try_into().unwrap() {
                    results[usize::try_from(result).unwrap()] = true;
                }
            }
            for (i, v) in results.iter().enumerate() {
                if !*v {
                    if i - 1 > best_length {
                        best_length = i - 1;
                        best_weights.clear();
                        best_weights.push(weights);
                    } else if i - 1 == best_length {
                        best_weights.push(weights);
                    }
                    break 'a;
                }
            }
            results.resize(results.len() * 2, false);
        }
    }

    return (best_length, best_weights);
}

fn main() {
    for i in 1..20 {
        println!("{i}: {:?}", brute_force_gadgets::<3>(i));
    }
}
