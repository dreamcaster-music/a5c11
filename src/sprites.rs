use crate::core::{rand::range, terminal::Element, Position, Sprite};

/// A simple firework sprite for testing
pub struct Firework {
    elements: Vec<(f32, f32, f32, f32)>,
    trails: Vec<(f32, f32, u8)>,
}

impl Firework {
    pub fn new(fx: usize, fy: usize) -> Self {
        let rand = crate::core::rand::vec_range(-2.0, 5.0, 256);
        let rand2 = crate::core::rand::vec_range(-3.0, 0.0, 256);
        let mut elements = Vec::with_capacity(rand.len());
        let mut rand = rand.into_iter();
        let mut rand2 = rand2.into_iter();
        for _ in 0..elements.capacity() {
            let x = rand.next().unwrap();
            let y = rand2.next().unwrap();

            elements.push((fx as f32, fy as f32, x, y));
        }

        Self {
            elements,
            trails: Vec::new(),
        }
    }
}

impl Sprite for Firework {
    fn elements(&self) -> Vec<(crate::core::terminal::Element, Position)> {
        let mut elements: Vec<(Element, Position)> = self
            .elements
            .iter()
            .map(|(x, y, _, _)| {
                (
                    Element::new(range::<u8>(40, 100) as char, 207, 0),
                    Position(*x, *y),
                )
            })
            .collect();

        let trails: Vec<(Element, Position)> = self
            .trails
            .iter()
            .map(|(x, y, power)| {
                (
                    Element::new(range::<u8>(40, 100) as char, 232 + power, 0),
                    Position(*x, *y),
                )
            })
            .collect();

        elements.extend(trails);
        elements
    }

    fn next(&mut self) {
        self.trails = self
            .trails
            .iter()
            .map(|(x, y, power)| (*x, *y, power - 1))
            .collect::<Vec<_>>();

        self.trails.retain(|(_, _, power)| *power > 0);

        self.elements = self
            .elements
            .iter()
            .map(|(x, y, vx, vy)| {
                self.trails.push((*x, *y, 20));
                (x + vx, y + vy, vx * 0.9, (vy + 0.1) * 0.9)
            })
            .collect::<Vec<_>>()
    }
}
