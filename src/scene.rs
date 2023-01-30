
use nalgebra as na;

type Scalar = f64;
type Point = na::Point3<Scalar>;
type Vector = na::Vector3<Scalar>;


enum Entity {
    //Primitives
    Sphere { radius: Scalar },
    Box { dimensions: Vector },

    //Alterations
    Elongation {},

    //Combinations
    Union {},

    //Positioning
    Transform {},

    //Duplication
    Symmetry {},
    Repetition {},
    FiniteRep {},

    //Deformations/Distortions
    Displacement {},
    Twist {},
    Bend {},
}

enum Scene {
    Group(Vec<Scene>)
}


//https://iquilezles.org/articles/distfunctions/
