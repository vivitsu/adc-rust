use std::fs::read_to_string;

struct PresentBox {
    l: u32,
    w: u32,
    h: u32,
}

impl PresentBox {
    #[inline]
    fn surface_area(&self) -> u32 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    #[inline]
    fn wrapping_paper(&mut self) -> u32 {
        let (s1, s2) = self.smallest_two();
        self.surface_area() + (s1 * s2)
    }

    #[inline]
    fn volume(&self) -> u32 {
        self.l * self.h * self.w
    }

    #[inline]
    fn smallest_two(&mut self) -> (u32, u32) {
        let mut sides = [self.l, self.h, self.w];
        sides.sort_unstable();
        (sides[0], sides[1])
    }

    #[inline]
    fn ribbon(&mut self) -> u32 {
        let (s1, s2) = self.smallest_two();
        (2 * s1) + (2 * s2) + self.volume()
    }
}

fn main() -> anyhow::Result<()> {
    let file = read_to_string("y2015/src/bin/day2-2015/input.txt")?;
    let mut dims: Vec<PresentBox> = file
        .lines()
        .map(|line| {
            line.split('x')
                .map(|dim| dim.parse::<u32>().unwrap())
                .collect()
        })
        .map(|dim: Vec<u32>| {
            assert_eq!(dim.len(), 3);
            PresentBox {
                l: dim[0],
                w: dim[1],
                h: dim[2],
            }
        })
        .collect();

    let (total_wrapping_paper, total_ribbon): (u32, u32) =
        dims.iter_mut().fold((0, 0), |acc, present_box| {
            (
                acc.0 + present_box.wrapping_paper(),
                acc.1 + present_box.ribbon(),
            )
        });

    println!("Part 1: {total_wrapping_paper}");
    println!("Part 2: {total_ribbon}");
    Ok(())
}
