use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window: WindowId,
    points: Vec<f32>,
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

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .view(view)
        .build()
        .expect("failed to build window");
    let mut rng = rand::thread_rng();
    let points = compute_points(&mut rng, 10, 1.0, 0.9);
    Model { window, points }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLUE);

    let width_per_step = frame.rect().w() / (model.points.len() as f32 - 1.0);
    let start_x = frame.rect().left();
    let points = model.points.iter().cloned().enumerate().map(|(i, y)| {
        let x = (i as f32 * width_per_step) + start_x;
        (x, y * 100.0)
    });
    draw.polyline().weight(1.0).color(PINK).points(points);

    draw.to_frame(app, &frame).expect("failed to render sketch");
}
