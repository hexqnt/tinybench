use std::env;
use std::process;

type Elem = usize;

#[derive(Debug)]
struct Pfannkuch {
    s: [Elem; 16],
    t: [Elem; 16],
    maxflips: i32,
    max_n: usize,
    odd: bool,
    checksum: i32,
}

impl Pfannkuch {
    fn new() -> Self {
        Pfannkuch {
            s: [0; 16],
            t: [0; 16],
            maxflips: 0,
            max_n: 0,
            odd: false,
            checksum: 0,
        }
    }

    fn flip(&mut self) -> usize {
        let mut flips_count: usize = 1;

        self.t[..self.max_n].copy_from_slice(&self.s[..self.max_n]);

        loop {
            let mut x: usize = 0;
            let mut y: usize = self.t[0];

            while x < y {
                self.t.swap(x, y);
                x += 1;
                y -= 1;
            }
            flips_count += 1;
            
            if self.t[self.t[0]] == 0 {
                break;
            }
        }
        flips_count
    }

    fn rotate(&mut self, n: usize) {
        let c = self.s[0];
        for i in 0..n {
            self.s[i] = self.s[i + 1];
        }
        self.s[n] = c;
    }

    // n_param is self.max_n from main, which is the N for Fannkuch.
    fn tk(&mut self, n_param: usize) {
        let mut p_count: usize = 0; // Permutation counter index, Go's 'i' in tk
        let mut c_perm_counts = [0usize; 16]; // Permutation counts, Go's 'c' in tk

        while p_count < n_param {
            self.rotate(p_count); 

            if c_perm_counts[p_count] >= p_count {
                c_perm_counts[p_count] = 0;
                p_count += 1;
                continue;
            }

            c_perm_counts[p_count] += 1;
            p_count = 1; 

            self.odd = !self.odd;

            if self.s[0] != 0 {
                let mut f = 1usize;
                if self.s[self.s[0]] != 0 {
                    f = self.flip();
                }
                let f_i32 = f as i32;

                if f_i32 > self.maxflips {
                    self.maxflips = f_i32;
                }

                if self.odd {
                    self.checksum -= f_i32;
                } else {
                    self.checksum += f_i32;
                }
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "fannkuch_redux_rust".to_string());
    let n_arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("usage: {} number", program);
            process::exit(1);
        }
    };

    let mut pf = Pfannkuch::new();

    match n_arg.parse::<usize>() {
        Ok(n) => pf.max_n = n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", n_arg);
            process::exit(1);
        }
    }

    if pf.max_n < 3 || pf.max_n > 15 {
        eprintln!("Error: N must be between 3 and 15, inclusive.");
        process::exit(1);
    }

    for i in 0..pf.max_n {
        pf.s[i] = i;
    }

    pf.tk(pf.max_n);

    println!("{}\nPfannkuchen({}) = {}", pf.checksum, pf.max_n, pf.maxflips);
}
