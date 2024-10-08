use plotly::common::Mode;
use plotly::{Plot, Scatter};
use rand::{thread_rng, Rng};

struct Walker {
    x: i16,
    y: i16,
    position: Vec<(i16, i16)>,
}

impl Walker {
    fn new(width: i16, height: i16) -> Self {
        let x = width / 2;
        let y = height / 2;
        Walker {
            x,
            y,
            position: vec![(x, y)],
        }
    }
    fn stepping_right(&mut self) {
        let mut rng = thread_rng();
        let choice: f64 = rng.gen_range(0.0..1.0);
        if choice <= 0.7 {
            self.x += 1;
        } else if choice <= 0.8 {
            self.x -= 1;
        } else if choice <= 0.9 {
            self.y += 1;
        } else {
            self.y -= 1;
        }
        self.position.push((self.x, self.y));
    }
    fn stepping_down(&mut self) {
        let mut rng = thread_rng();
        let choice: f64 = rng.gen_range(0.0..1.0);
        if choice >= 0.7 {
            self.x += 1;
        } else if choice >= 0.8 {
            self.x -= 1;
        } else if choice >= 0.9 {
            self.y += 1;
        } else {
            self.y -= 1;
        }
        self.position.push((self.x, self.y));
    }
    fn show(&mut self, filename: &str) {
        let (x_vals, y_vals): (Vec<i16>, Vec<i16>) = self.position.iter().cloned().unzip();
        let mut plot = Plot::new();
        let trace = Scatter::new(x_vals, y_vals).mode(Mode::Markers);
        plot.add_trace(trace);
        plot.write_html(filename);
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut walker = Walker::new(width, height);
    {
        let walker_right = &mut walker;
        for _ in 1..10000 {
            walker_right.stepping_right();
        }
        walker_right.show("randomness/results/01_right.html");
    }
    {
        let walker_down = &mut walker;
        for _ in 1..10000 {
            walker_down.stepping_down();
        }
        walker_down.show("randomness/results/01_down.html");
    }
}
