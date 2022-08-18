// Rust program for implementation
// of Sieve of Atkin

fn sieve_of_atkin() {
    const LIMIT: usize = 20;
    let mut sieve: [bool; LIMIT + 1] = [false; LIMIT + 1];
    
    // 2 and 3 are known to be prime
    if LIMIT > 2 {
        sieve[2] = true;
    }
    if LIMIT > 3 {
        sieve[3] = true;
    }

    /* 
    Mark sieve[n] true if one of these condtiions are met
    
    a) n = (4 * x * x) + (y * y) has an odd number of solutions
        and n % 12 == 1 or n % 12 == 5
    b) n = (3 * x * x) + (y * y) has an odd number of solutions
        and n % 12 == 7
    c) n = (3 * x * x) - (y * y) has an odd number of solutions,
        x > y and n % 12 == 11 
        
        @See https://www.geeksforgeeks.org/sieve-of-atkin/ for examples
    */
    const START: usize = 1;
    for xu in START..LIMIT {
        let x: i32 = xu as i32;
        if x * x > LIMIT as i32 {
            break;
        }
        for yu in START..LIMIT {
            let y: i32 = yu as i32;
            if y * y > LIMIT as i32 {
                break;
            }

            // Condition 1
            let n: i32 = (4 * x * x) + (y * y);
            if n <= LIMIT as i32 && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] ^= true;
            }

            // Condition 2
            let n: i32 = (3 * x * x) + (y * y);
            if n <= LIMIT as i32 && n % 12 == 7 {
                sieve[n as usize] ^= true;
            }

            // Condition 3
            let n: i32 = (3 * x * x) - (y * y);
            if x > y && n <= LIMIT as i32 && n % 12 == 11 {
                sieve[n as usize] ^= true
            }
        }
    }

    // Mark all multiples of squares as non-prime
    let mut r: i32 = 5;
    loop {
        if r * r > LIMIT as i32 {
            break;
        }

        if sieve[r as usize] {
            let mut i: i32 = r * r;
            loop {
                if i > LIMIT as i32 {
                    break;
                }

                sieve[i as usize] = false;

                i += r * r;
            }
        }

        r += 1;
    }

    // Print primes contained within sieve's index.
    for a in START..=LIMIT {
        if sieve[a] {
            println!("{}", a)
        }
    }
}

fn main() {
    sieve_of_atkin();
    return ();
}
