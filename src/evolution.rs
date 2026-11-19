
use rand::{
    *, rngs::SmallRng,
    seq::SliceRandom,
};

use crate::problem::*;
use crate::genetic::*;

struct Generation {
    pop   : Box<[Point]>, // linear matrix
    chairs: Box<[Point]>, // linear matrix
    best  : Tour        ,
}

impl Generation {
    fn genesis (
        pop_size  : usize        , 
        chairs    : usize        ,
        tournament: usize        , 
        rng       : &mut SmallRng,
        t_len     : usize        ,
    ) -> Box<[Point]> {
        let mut pop = Vec::with_capacity(pop_size * t_len);
        for _ in 0..pop_size {
            let start = pop.len();
            
            pop.extend((0..t_len).map(|i| i as Point));
            
            pop[start..].shuffle(rng);
        }
        
        pop.into_boxed_slice()
    }

    fn step (
        &mut self                , 
        rng       : &mut SmallRng,
        t_len     : usize        ,
    ) {
        
    }


}
