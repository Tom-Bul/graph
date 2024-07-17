use Matrix;
use Point;

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Triangle { p1, p2, p3 }
    }

    // pub fn fill(&self, color: u32, buffer: &mut Vec<u32>) {
    //   let mat_abc
    // }
}
