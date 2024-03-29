use colorgrad::Gradient;
use nannou::{color::IntoLinSrgba, draw::properties::ColorScalar, prelude::*};
use rand::{Rng, RngCore};
use std::{
    iter,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    window: WindowId,
    gradient: WrappedGradient,
    layers: Vec<Layer>,
    rng: Box<dyn RngCore>,
}

struct Layer {
    baseline_y: f32,
    points: Vec<f32>,
}

impl Layer {
    pub fn draw_to<C: IntoLinSrgba<ColorScalar>>(
        &self,
        draw: &Draw,
        rect: &Rect,
        z_index: f32,
        color: C,
    ) {
        let width_per_step = rect.w() / (self.points.len() as f32 - 1.0);
        let left_cap = iter::once((rect.left() - 50.0, rect.bottom() * 20.0));
        let right_cap = iter::once((rect.right() + 50.0, rect.bottom() * 20.0));
        let points = self.points.iter().cloned().enumerate().map(|(i, y)| {
            let x = (i as f32 * width_per_step) + rect.left();
            (x, y + self.baseline_y)
        });
        draw.polygon()
            .z(z_index)
            .color(color)
            .points(left_cap.chain(points).chain(right_cap));
    }
}

struct WrappedGradient(Gradient);

impl WrappedGradient {
    pub fn sample<N: Into<f64>>(&self, n: N) -> impl IntoLinSrgba<ColorScalar> {
        let c = self.0.at(n.into());
        Rgb::from_components((c.r, c.g, c.b))
    }
}

fn midpoint(i: usize, j: usize) -> usize {
    i + (j - i) / 2
}

fn fill<R: Rng>(
    rng: &mut R,
    points: &mut Vec<Option<f32>>,
    i: usize,
    j: usize,
    steps: u32,
    displacement: f32,
    smoothness: f32,
) {
    if steps == 0 {
        return;
    }
    let mid = midpoint(i, j);
    if mid == i || mid == j {
        return;
    }
    let i_val = points[i].expect("no value at i index");
    let j_val = points[j].expect("no value at j index");
    let mid_val = (i_val + j_val) / 2.0 + rng.gen_range(-displacement..=displacement);
    points[mid] = Some(mid_val);
    let new_displacement = displacement * 2.0f32.pow(-smoothness);
    fill(rng, points, i, mid, steps - 1, new_displacement, smoothness);
    fill(rng, points, mid, j, steps - 1, new_displacement, smoothness);
}

fn compute_points<R: Rng>(
    rng: &mut R,
    steps: u32,
    initial_displacement: f32,
    smoothness: f32,
) -> Vec<f32> {
    let size = 2usize.pow(steps) + 1;
    let mut points: Vec<Option<f32>> = vec![None; size];
    let i = 0usize;
    let j = points.len() - 1;
    points[i] = Some(0.0);
    points[j] = Some(0.0);
    fill(
        rng,
        &mut points,
        i,
        j,
        steps,
        initial_displacement,
        smoothness,
    );
    points
        .into_iter()
        .filter_map(std::convert::identity)
        .collect::<Vec<_>>()
}

fn temp_compute_points<R: Rng>(rng: &mut R) -> Vec<f32> {
    compute_points(rng, 10, 100.0, 0.9)
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(1500, 500)
        .view(view)
        .build()
        .expect("failed to build window");
    let mut rng = rand::thread_rng();
    let gradient = WrappedGradient(colorgrad::inferno());
    let steps = 10;
    let initial_displacement = 50.0;
    let scale = 50.0;
    let baseline = 0.0;
    let smoothness = 0.7;
    let layers = (-3..3)
        .map(|i| scale * i as f32 + baseline)
        .map(|baseline_y| Layer {
            baseline_y,
            points: compute_points(&mut rng, steps, initial_displacement, smoothness),
        })
        .collect::<Vec<_>>();
    Model {
        window,
        gradient,
        layers,
        rng: Box::new(rng),
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(e), ..
        } => {
            window_event(app, model, e);
        }
        _ => {}
    }
}

fn window_event(app: &App, model: &mut Model, window_event: WindowEvent) {
    match window_event {
        KeyPressed(key) => match key {
            Key::Space => {
                let layers = model
                    .layers
                    .iter()
                    .map(|l| Layer {
                        baseline_y: l.baseline_y,
                        points: temp_compute_points(&mut model.rng),
                    })
                    .collect::<Vec<_>>();
                model.layers = layers;
            }
            Key::S => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("failed to compute current timestamp")
                    .as_millis();
                let filename = format!("banner_{}.png", timestamp);
                let path = PathBuf::new()
                    .join(std::env::current_dir().expect("failed to locate current working dir"))
                    .join(filename);
                app.main_window().capture_frame(path)
            }
            _ => {}
        },
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gradient = &model.gradient;

    draw.background().color(gradient.sample(0.0));

    let rect = frame.rect();

    let stack_height = (1 + model.layers.len()) as f64;
    for (index, layer) in model.layers.iter().enumerate() {
        let color = gradient.sample((index as f64 + 1.0) / stack_height);
        let z_index = -(index as f32);
        layer.draw_to(&draw, &rect, z_index, color);
    }

    draw.to_frame(app, &frame).expect("failed to render sketch");
}
