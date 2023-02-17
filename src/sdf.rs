
use nalgebra as na;

use crate::{Scalar, Vector, Point};

pub trait SDF<I, O> {
    fn query(&self, point: I) -> O;
}

impl SDF<Point, Scalar> for Entity {
    fn query(&self, point: Point) -> Scalar {
        match self {
            Entity::Sphere { r } => todo!(),
            Entity::Box { dims } => todo!(),
            Entity::Elongation {  } => todo!(),
            Entity::Union {  } => todo!(),
            Entity::Transform {  } => todo!(),
            Entity::Symmetry {  } => todo!(),
            Entity::Repetition {  } => todo!(),
            Entity::FiniteRep {  } => todo!(),
            Entity::Displacement {  } => todo!(),
            Entity::Twist {  } => todo!(),
            Entity::Bend {  } => todo!(),
        }
    }
}

impl<T: SDF<Point, Scalar>> SDF<Point, Vector> for T {
    fn query(&self, point: Point) -> Vector {
        todo!()
    }
}

enum Entity {
    //Primitives
    Sphere { r: Scalar },
    Box { dims: Vector },
    //..

    //Alterations
    Elongation {},
    //..

    //Combinations
    Union {},
    //..

    //Positioning
    Transform {},
    //..

    //Duplication
    Symmetry {},
    Repetition {},
    FiniteRep {},

    //Deformations/Distortions
    Displacement {},
    Twist {},
    Bend {},
}


//https://iquilezles.org/articles/distfunctions/
