extern crate num;


use std::result::Result;
use std::rand::{task_rng, Rng, TaskRng};
use num::bigint::{ToBigUint, BigUint, RandBigInt};
use num::integer::Integer;
/*
 * A prime number generator useful in generating RSA keys
 */
struct PrimeRng<'a> {
    generator: &'a mut RandBigInt
}

impl <'a>PrimeRng<'a> {
    pub fn new() -> PrimeRng<'a> {
        PrimeRng {
            generator: &mut task_rng()
        }
    }

    pub fn from_rng<T: RandBigInt>(rng: &'a mut T) -> PrimeRng<'a> {
        PrimeRng {
            generator: rng  
        }
    }
    /*
     * Generates a prime number 'bit_size' number of bits in length 
     */
    pub fn generate(&self, bit_size: uint) -> Result<BigUint, &'static str> {
    
        /*
         * Generate an unsigned candidate number using the standard
         * library random number generator.
         */
        let candidate = self.generator.gen_biguint(bit_size);
    
                
    
    
    }
    
    /*
     * Implements the Miller-Rabin probablistic primality test.
     *
     *
     */
    pub fn probably_prime(&self, candidate: BigUint, security: uint) -> bool {

        let one = 1u.to_biguint().unwrap(); 

        if candidate <= one {
            return false;
        }

        let two = 2u.to_biguint().unwrap();

        if candidate == two {
            return true;
        }

        if candidate.is_even() {
            return false;
        }

        let (power, remainder) = factor_powers_of_two(candidate - one);

        'witness: for i in range(1, security) {
            let a: BigUint = self.generator.gen_range(two, candidate - one);
        
            let mut x = modular_exponent(a, remainder, candidate);

            if x == one || x == candidate - one {
                continue 'witness;
            }

            for j in range(1, power) {
                x = modular_exponent(x, two, candidate);
                if x == 1 { 
                    return false;
                }
                if x == candidate - one {
                    continue 'witness;
                }
            }

            return false;
        }

        return true;

    }

   
}

fn factor_powers_of_two(mut number: BigUint) -> (BigUint, BigUint){
    
    let mut power = 1u.to_biguint().unwrap();
    let two = 2u.to_biguint().unwrap();
    loop {
        number /= two;
        if number.is_odd() {
            return (power, number);
        }
        power += 1;
    }
}




/*
 *
 * Raises 
 *
 */
pub fn modular_exp(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {

    let result = 1u.to_biguint();
    let base = base.div()

    while power > 0 {
        if exponent.mod(2) == 1 {
            result = (result * base).mod(modulus);
        }
        exponent = exponent >> 1;
        base = (base * base).mod(modulus);
    }
    result
}


#[cfg(test)]
mod test {

    use super::probably_prime;
    use num::bigint::ToBigUint;

    #[test]
    fn test_definitely_prime() {
        assert!(probably_prime(17u.to_biguint().unwrap(), 1));
    }

    #[test]
    fn test_modular_exp() {
        
    }

}
