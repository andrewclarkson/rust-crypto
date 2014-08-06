extern crate num;


use std::result::Result;
use std::rand::{task_rng, Rng, TaskRng};
use num::bigint::{ToBigUint, BigUint, RandBigInt};
use num::integer::Integer;

/*
 * Raises 
 *
 */
pub fn modular_exp(mut base: BigUint, mut exponent: BigUint, modulus: BigUint) -> BigUint {



    let zero = 0u.to_biguint().unwrap();
    
    assert!(modulus != zero);
    
    let one = 1u.to_biguint().unwrap();
    let two = 2u.to_biguint().unwrap();


    let mut result = 1u.to_biguint().unwrap();
    base = base % modulus;
    exponent = exponent;

    while exponent > zero {
        if exponent % two == one {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    result
}


#[cfg(test)]
mod test {

    use super::modular_exp;
    use num::bigint::{ToBigUint};

    #[test]
    #[should_fail]
    fn test_modular_exp_bad() {
        assert_eq!(
            modular_exp(
                26614u.to_biguint().unwrap(),
                480u.to_biguint().unwrap(),
                0u.to_biguint().unwrap()
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
                4u.to_biguint().unwrap(),
                13u.to_biguint().unwrap(),
                497u.to_biguint().unwrap()
            ), 
            445u.to_biguint().unwrap()
        );

        /*
         * Zero's 
         */
        assert_eq!(
            modular_exp(
                0u.to_biguint().unwrap(),
                25871u.to_biguint().unwrap(),
                14932u.to_biguint().unwrap()
            ), 
            0u.to_biguint().unwrap()
        );

        assert_eq!(
            modular_exp(
                962u.to_biguint().unwrap(),
                0u.to_biguint().unwrap(),
                29008u.to_biguint().unwrap()
            ), 
            1u.to_biguint().unwrap()
        );
        
        /*
         * Random examples calculated using python
         */
        assert_eq!(
            modular_exp(
                6826u.to_biguint().unwrap(),
                25871u.to_biguint().unwrap(),
                14932u.to_biguint().unwrap()
            ), 
            2632u.to_biguint().unwrap()
        );
        
        assert_eq!(
            modular_exp(
                962u.to_biguint().unwrap(),
                6431u.to_biguint().unwrap(),
                29008u.to_biguint().unwrap()
            ), 
            10064u.to_biguint().unwrap()
        );
        
        assert_eq!(
            modular_exp(
                26614u.to_biguint().unwrap(),
                480u.to_biguint().unwrap(),
                18928u.to_biguint().unwrap()
            ), 
            15120u.to_biguint().unwrap()
        );





    }

}
