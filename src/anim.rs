use flo_curves::bezier::Curve;
use num_traits::Float;

use crate::vector::VectorND;

pub struct MultiVariableAnimation<Prim, const States: usize>
where
    Prim: Float + Into<f64>,
{
    curve: Curve<VectorND<Prim, States>>,
}

// impl MultiVariableAnimation {}
