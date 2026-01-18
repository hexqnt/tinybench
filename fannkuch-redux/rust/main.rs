use std::env;
use std::process;

type Elem = i32;

#[derive(Debug)]
struct Pfannkuch {
    s: [Elem; 16],
    t: [Elem; 16],
    maxflips: i32,
    max_n: i32,
    odd: i32,
    checksum: i32,
}

impl Pfannkuch {
    fn new() -> Self {
        Pfannkuch {
            s: [0; 16],
            t: [0; 16],
            maxflips: 0,
            max_n: 0,
            odd: 0, // Initialized to 0
            checksum: 0,
        }
    }

    fn flip(&mut self) -> i32 {
        let mut flips_count = 1;
        
        let current_max_n = self.max_n as usize;
        self.t[..current_max_n].copy_from_slice(&self.s[..current_max_n]);

        loop {
            let mut x: usize = 0;
            let mut y: usize = self.t[0] as usize;

            while x < y {
                self.t.swap(x, y);
                x += 1;
                y -= 1;
            }
            flips_count += 1;
            
            if self.t[self.t[0] as usize] == 0 {
                break;
            }
        }
        flips_count
    }

    fn rotate(&mut self, n: i32) {
        let n_usize = n as usize;
        self.s[..=n_usize].rotate_left(1);
    }

    // n_param is self.max_n from main, which is the N for Fannkuch.
    fn tk(&mut self, n_param: i32) {
        let mut p_count = 0; // Permutation counter index, Go's 'i' in tk
        let mut c_perm_counts = [0 as Elem; 16]; // Permutation counts, Go's 'c' in tk
        
        while p_count < n_param {
            self.rotate(p_count); 
            
            let p_count_usize = p_count as usize;

            if c_perm_counts[p_count_usize] >= p_count as Elem {
                c_perm_counts[p_count_usize] = 0;
                p_count += 1;
                continue;
            }

            c_perm_counts[p_count_usize] += 1;
            p_count = 1; 
            
            self.odd = !self.odd; 

            if self.s[0] != 0 {
                let mut f = 1;
                if self.s[self.s[0] as usize] != 0 { 
                    f = self.flip();
                }

                if f > self.maxflips {
                    self.maxflips = f;
                }

                if self.odd != 0 { // If odd is -1
                    self.checksum -= f;
                } else { // If odd is 0
                    self.checksum += f;
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
    let n_str = match args.next() {
        Some(n) => n,
        None => {
            eprintln!("usage: {} number", program);
            process::exit(1);
        }
    };

    let n = match n_str.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number.", n_str);
            process::exit(1);
        }
    };

    if n < 3 || n > 15 {
        eprintln!("Error: N must be between 3 and 15, inclusive.");
        process::exit(1);
    }

    let mut pf = Pfannkuch::new();
    pf.max_n = n;

    for i in 0..(n as usize) {
        pf.s[i] = i as Elem;
    }

    pf.tk(n);

    println!("{}\nPfannkuchen({}) = {}", pf.checksum, pf.max_n, pf.maxflips);
}
