use crate::core::{
    rand::{range, vec_range},
    terminal::{rgb, Element},
    Position, Sprite,
};

/// A simple firework sprite for testing
pub struct Firework {
    elements: Vec<(f32, f32, f32, f32)>,
    trails: Vec<(f32, f32, f32)>,
    rgb: (u8, u8, u8),
}

impl Firework {
    pub fn new(fx: usize, fy: usize, r: u8, g: u8, b: u8) -> Self {
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
            rgb: (r, g, b),
        }
    }
}

impl Sprite for Firework {
    fn elements(&self) -> Vec<(crate::core::terminal::Element, Position)> {
        let mut rands = vec_range::<u8>(40, 100, 200000)
            .into_iter()
            .map(|f| f as char);
        let mut elements: Vec<(Element, Position)> = self
            .elements
            .iter()
            .map(|(x, y, _, _)| {
                (
                    Element::new(
                        rands.next().unwrap(),
                        rgb(self.rgb.0, self.rgb.1, self.rgb.2),
                        0,
                    ),
                    Position(*x, *y),
                )
            })
            .collect();

        let trails: Vec<(Element, Position)> = self
            .trails
            .iter()
            .map(|(x, y, power)| {
                (
                    Element::new(
                        rands.next().unwrap(),
                        rgb(
                            ((*power / 5.0) * self.rgb.0 as f32) as u8,
                            ((*power / 5.0) * self.rgb.1 as f32) as u8,
                            ((*power / 5.0) * self.rgb.2 as f32) as u8,
                        ),
                        0,
                    ),
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
            .map(|(x, y, power)| (*x, *y, power - 0.2))
            .collect::<Vec<_>>();

        self.trails.retain(|(_, y, power)| {
            *power > 0.0 && *y < (crate::core::terminal::size().unwrap().1 as f32)
        });

        self.elements
            .retain(|(_, y, _, _)| *y < (crate::core::terminal::size().unwrap().1 as f32));

        self.elements = self
            .elements
            .iter()
            .map(|(x, y, vx, vy)| {
                self.trails.push((*x, *y, 5.0));
                (x + vx, y + vy, vx * 0.9, (vy + 0.1) * 0.9)
            })
            .collect::<Vec<_>>()
    }
}
pub struct Checkerboard {
    width: usize,
    height: usize,
    fg_color1: u8, // Foreground color for the first color
    bg_color1: u8, // Background color for the first color
    fg_color2: u8, // Foreground color for the second color
    bg_color2: u8, // Background color for the second color
    alt: bool,
}

impl Checkerboard {
    // New constructor that accepts custom foreground and background colors
    pub fn new(
        width: usize,
        height: usize,
        fg_color1: u8,
        bg_color1: u8,
        fg_color2: u8,
        bg_color2: u8,
    ) -> Self {
        Self {
            width,
            height,
            fg_color1,
            bg_color1,
            fg_color2,
            bg_color2,
            alt: false,
        }
    }
}
impl Sprite for Checkerboard {
    // Update generate_elements to use the custom colors
    fn elements(&self) -> Vec<(crate::core::terminal::Element, Position)> {
        (0..(self.height * self.width))
            .enumerate()
            .map(|(_, i)| {
                let x = i % self.width;
                let y = (i - x) / self.width;

                // Alternating between '█' and ' ' based on row and column
                let ch = if (x + y) % 2 == 0 { '█' } else { ' ' };

                let fg_code;
                let bg_code;

                if self.alt == false {
                    fg_code = self.fg_color1;
                    bg_code = self.bg_color1;
                } else {
                    fg_code = self.fg_color2;
                    bg_code = self.bg_color2;
                }
                // let fg_code = if (x + y) % 2 == 0 {
                //     self.fg_color1
                // } else {
                //     self.fg_color2
                // };

                // let bg_code = if (x + y) % 2 == 0 {
                //     self.bg_color1
                // } else {
                //     self.bg_color2
                // };

                (
                    Element::new(ch, fg_code, bg_code),
                    Position(x as f32, y as f32),
                )
            })
            .collect()
    }

    fn next(&mut self) {
        self.alt = !self.alt;
    }
}
