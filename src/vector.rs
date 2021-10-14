use flo_curves::Coordinate;
use num_traits::Float;
use std::ops::{Add, Mul, Sub};

/// A fully-generic N-dimensional vector, where no implementation details are known.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct VectorND<Prim, const SIZE: usize>([Prim; SIZE])
where
    Prim: Float + Into<f64>;

impl<Prim, const SIZE: usize> Add for VectorND<Prim, SIZE>
where
    Prim: Float + Into<f64>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_components(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| (*a + *b).into())
                .collect::<Vec<f64>>()
                .as_slice(),
        )
    }
}
impl<Prim, const SIZE: usize> Mul for VectorND<Prim, SIZE>
where
    Prim: Float + Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_components(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| (*a * (*b)).into())
                .collect::<Vec<f64>>()
                .as_slice(),
        )
    }
}
impl<Prim, const SIZE: usize> Mul<f64> for VectorND<Prim, SIZE>
where
    Prim: Float + Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_components(
            self.0
                .iter()
                .map(|a| rhs * a.to_f64().unwrap())
                .collect::<Vec<f64>>()
                .as_slice(),
        )
    }
}

impl<Prim, const SIZE: usize> Sub for VectorND<Prim, SIZE>
where
    Prim: Float + Into<f64>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_components(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| (*a - *b).into())
                .collect::<Vec<f64>>()
                .as_slice(),
        )
    }
}

impl<Prim, const SIZE: usize> Coordinate for VectorND<Prim, SIZE>
where
    Prim: Float + Into<f64>,
{
    fn from_components(components: &[f64]) -> Self {
        // Allocate an array to copy the components over
        let mut components_array = [Prim::zero(); SIZE];

        // Copy the components over the array
        for (i, component) in components.iter().enumerate() {
            if i < SIZE {
                components_array[i] = Prim::from(*component).unwrap();
            }
        }

        Self(components_array)
    }

    fn origin() -> Self {
        Self([Prim::zero(); SIZE])
    }

    fn len() -> usize {
        SIZE
    }

    fn get(&self, index: usize) -> f64 {
        self.0[index].to_f64().unwrap()
    }

    fn from_biggest_components(p1: Self, p2: Self) -> Self {
        let mut biggest = p1;
        let mut smallest = p2;

        if p1.0[0] < p2.0[0] {
            biggest = p2;
            smallest = p1;
        }

        for i in 1..SIZE {
            if biggest.0[i] < smallest.0[i] {
                biggest = smallest;
                smallest = p2;
            }
        }

        biggest
    }

    fn from_smallest_components(p1: Self, p2: Self) -> Self {
        let mut smallest = p1;
        let mut biggest = p2;

        if p1.0[0] > p2.0[0] {
            smallest = p2;
            biggest = p1;
        }

        for i in 1..SIZE {
            if smallest.0[i] > biggest.0[i] {
                smallest = biggest;
                biggest = p2;
            }
        }

        smallest
    }
}
