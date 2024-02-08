use nannou::prelude::*;
use rand::{Rng, RngCore};
use std::iter;

fn main() {
    nannou::app(model).event(event).run();
}

struct Model {
    window: WindowId,
    points: Vec<f32>,
    rng: Box<dyn RngCore>,
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
    compute_points(rng, 10, 1.0, 0.9)
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .view(view)
        .build()
        .expect("failed to build window");
    let mut rng = rand::thread_rng();
    let points = temp_compute_points(&mut rng);
    Model {
        window,
        points,
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

fn window_event(_app: &App, model: &mut Model, window_event: WindowEvent) {
    match window_event {
        KeyPressed(key) => match key {
            Key::Space => {
                model.points = temp_compute_points(&mut model.rng);
            }
            _ => {}
        },
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLUE);

    let width_per_step = frame.rect().w() / (model.points.len() as f32 - 1.0);
    let rect = frame.rect();
    let left_cap = iter::once((rect.left() - 50.0, rect.bottom() * 20.0));
    let right_cap = iter::once((rect.right() + 50.0, rect.bottom() * 2.0));
    let points = model.points.iter().cloned().enumerate().map(|(i, y)| {
        let x = (i as f32 * width_per_step) + rect.left();
        (x, y * 100.0)
    });
    draw.polygon()
        .color(PINK)
        .points(left_cap.chain(points).chain(right_cap));

    draw.to_frame(app, &frame).expect("failed to render sketch");
}
