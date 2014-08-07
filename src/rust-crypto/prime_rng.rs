extern crate num;

use std::rand::task_rng;
use num::bigint::{ToBigUint, BigUint, RandBigInt};
use num::integer::Integer;

/// Primes for trial division
static primes: [u8, ..53] = [
     3u8,    5u8,   7u8,  11u8,  13u8,  17u8,  19u8,  23u8,  29u8,  31u8,
     37u8,  41u8,  43u8,  47u8,  53u8,  59u8,  61u8,  67u8,  71u8,  73u8, 
     79u8,  83u8,  89u8,  97u8, 101u8, 103u8, 107u8, 109u8, 113u8, 127u8,
    131u8, 137u8, 139u8, 149u8, 151u8, 157u8, 163u8, 167u8, 173u8, 179u8, 
    181u8, 191u8, 193u8, 197u8, 199u8, 211u8, 223u8, 227u8, 229u8, 233u8, 
    239u8, 241u8, 251u8
];

/// Generates a weak prime
///
///
pub fn generate_weak_prime(bit_size: uint, security: uint) -> BigUint {
    loop {
        let n = task_rng().gen_biguint(bit_size);
    
        if probably_prime_faster(&n, security) {
            return n;
        }
    }
}

#[inline]
pub fn probably_prime_faster(n: &BigUint, security: uint) -> bool { 
 
        /*
         * Trial division
         */
        for i in primes.iter() {
            let big = i.to_biguint().unwrap();
            if n.is_multiple_of(&big) {
                return false;
            }
        }

        /*
         * Use a more expensive primality test
         */
        if probably_prime(n, security) {
           return true;
        }

        return false;
}

/// Generates a strong prime
///
///
pub fn generate_strong_prime(bit_size: uint, security: uint) -> BigUint {
    
    let two = 2u.to_biguint().unwrap();
    let one = 1u.to_biguint().unwrap();

    let half = bit_size.div_floor(&2u);
    let s = generate_weak_prime(half, security);
    let t = generate_weak_prime(half + 1, security);
    
    let mut i = task_rng().gen_biguint(32u);

    let r: BigUint;
    
    loop {
        let candidate = (two * i * t) + one;
        if probably_prime_faster(&candidate, security) {
            r = candidate;
            break;
        }
        i = i + one;
    };

    let p0 = (two * modular_exp(&s, &(r - two), &r)) * s - one;


    let mut j = task_rng().gen_biguint(32u);
    
    loop {
        let candidate = p0 + (two * j * r * s);
        if probably_prime_faster(&candidate, security) {
            return candidate;
        }
        j = j + one;
    }



}

/// Implements the Miller-Rabin probablistic primality test.
///
///
///
pub fn probably_prime(candidate: &BigUint, security: uint) -> bool {
    let one = 1u.to_biguint().unwrap(); 

    if *candidate <= one {
        return false;
    }

    let two = 2u.to_biguint().unwrap();

    if *candidate == two {
        return true;
    }

    if candidate.is_even() {
        return false;
    }

    let bound = *candidate - one;

    let (power, remainder) = factor_powers_of_two(&bound);

    'witness: for i in range(1, security) {
        let a: BigUint = task_rng().gen_biguint_range(&two, &bound);
    
        let mut x = modular_exp(&a, &remainder, candidate);

        if x == one || x == bound {
            continue 'witness;
        }

        for j in range(1, power) {
            x = modular_exp(&x, &two, candidate);
            if x == one { 
                return false;
            }
            if x == bound {
                continue 'witness;
            }
        }

        return false;
    }

    return true;

}

fn factor_powers_of_two(number: &BigUint) -> (uint, BigUint){
    let mut n = number.clone();
    let mut power = 1u;
    let two = 2u.to_biguint().unwrap();
    loop {
        n = n / two;
        if n.is_odd() {
            return (power, n);
        }
        power += 1;
    }
}




/// Raises a number `base` to the exponent `exponent` modulo `modulus` and
/// returns the result. 
///
/// Equivalent to:
/// ```rust
/// num::pow(base, exponent) % modulus
/// ```
pub fn modular_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {



    let zero = 0u.to_biguint().unwrap();
    
    assert!(*modulus != zero);
    
    let one = 1u.to_biguint().unwrap();
    let two = 2u.to_biguint().unwrap();

    let mut result = 1u.to_biguint().unwrap();
    let mut b = *base % *modulus;
    let mut e = exponent.clone();

    while e > zero {
        if e % two == one {
            result = (result * b) % *modulus;
        }
        e = e >> 1;
        b = (b * b) % *modulus;
    }
    result
}


#[cfg(test)]
mod test {

    use super::{modular_exp, probably_prime};
    use num::bigint::{ToBigUint};


    #[test]
    fn test_probably_prime() {
        let seventeen = 17u.to_biguint().unwrap();
        assert!(probably_prime(&seventeen, 1));
        
        let eighteen = 18u.to_biguint().unwrap();
        assert!(!probably_prime(&eighteen, 1));
    }

    #[test]
    #[should_fail]
    fn test_modular_exp_bad() {
        assert_eq!(
            modular_exp(
                &26614u.to_biguint().unwrap(),
                &480u.to_biguint().unwrap(),
                &0u.to_biguint().unwrap()
            ), 
            15120u.to_biguint().unwrap()
        );
        
    }

    #[test]
    fn test_modular_exp() {

        /*
         * The wikipedia example
         */
        assert_eq!(
            modular_exp(
                &4u.to_biguint().unwrap(),
                &13u.to_biguint().unwrap(),
                &497u.to_biguint().unwrap()
            ), 
            445u.to_biguint().unwrap()
        );

        /*
         * Zero's 
         */
        assert_eq!(
            modular_exp(
                &0u.to_biguint().unwrap(),
                &25871u.to_biguint().unwrap(),
                &14932u.to_biguint().unwrap()
            ), 
            0u.to_biguint().unwrap()
        );

        assert_eq!(
            modular_exp(
                &962u.to_biguint().unwrap(),
                &0u.to_biguint().unwrap(),
                &29008u.to_biguint().unwrap()
            ), 
            1u.to_biguint().unwrap()
        );
        
        /*
         * Random examples calculated using python
         */
        assert_eq!(
            modular_exp(
                &6826u.to_biguint().unwrap(),
                &25871u.to_biguint().unwrap(),
                &14932u.to_biguint().unwrap()
            ), 
            2632u.to_biguint().unwrap()
        );
        
        assert_eq!(
            modular_exp(
                &962u.to_biguint().unwrap(),
                &6431u.to_biguint().unwrap(),
                &29008u.to_biguint().unwrap()
            ), 
            10064u.to_biguint().unwrap()
        );
        
        assert_eq!(
            modular_exp(
                &26614u.to_biguint().unwrap(),
                &480u.to_biguint().unwrap(),
                &18928u.to_biguint().unwrap()
            ), 
            15120u.to_biguint().unwrap()
        );
    }
}
