use macroquad::prelude::*;

// Struct to hold all the values to draw the Lorenz attractor equation
struct State {
    linestrip: Vec<Vec3>,
}

// Defining a type for a value that moves from a max to a min
// the step function changes the value by the step size
struct BouncingVariable {
    pub value: f32,
    min: f32,
    max: f32,
    step: f32,
    direction: f32,
}

impl BouncingVariable {
    fn new(initial_value: f32, min: f32, max: f32, step: f32) -> Self {
        BouncingVariable {
            value: initial_value,
            min,
            max,
            step,
            direction: 1.0,
        }
    }

    fn step(&mut self) {
        self.value += self.direction * self.step;
        if self.value > self.max {
            self.value = 2.0 * self.max - self.value;
            self.direction = -1.0;
        } else if self.value < self.min {
            self.value = 2.0 * self.min - self.value;
            self.direction = 1.0;
        }
    }
}

// Find the center point given the list of points
fn find_center(points: &Vec<Vec3>) -> Vec3 {
    let num_points = points.len();

    if num_points == 0 {
        panic!("Cannot find center of an empty vector.");
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_z = 0.0;

    for point in points {
        sum_x += point.x;
        sum_y += point.y;
        sum_z += point.z;
    }

    let center_x = sum_x / (num_points as f32);
    let center_y = sum_y / (num_points as f32);
    let center_z = sum_z / (num_points as f32);

    Vec3 {
        x: center_x,
        y: center_y,
        z: center_z,
    }
}

// Lorenz attractor equation
fn lorenz(v: &Vec3) -> Vec3 {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;
    let dt = 0.001;

    Vec3 {
        x: ((sigma * (v.y - v.x)) * dt) + v.x,
        y: ((v.x * (rho - v.z) - v.y) * dt) + v.y,
        z: ((v.x * v.y - beta * v.z) * dt) + v.z,
    }
}

fn orbit_camera(camera: &mut Camera3D, distance: f32, angle_around_target: f32, pitch_angle: &f32) {
    // Calculate the position of the camera based on the distance, angle and pitch
    let x = distance * angle_around_target.to_radians().cos() * pitch_angle.to_radians().cos();
    let y = distance * pitch_angle.to_radians().sin();
    let z = distance * angle_around_target.to_radians().sin() * pitch_angle.to_radians().cos();
    let camera_pos = Vec3::new(x, y, z) + camera.target;

    // Set the camera position and up vector
    camera.position = camera_pos;
    camera.up = Vec3::new(0.0, 1.0, 0.0); // Assuming y-axis is up
}

#[macroquad::main("3D")]
async fn main() {

    // Define 3d camera type
    let mut camera = Camera3D {
        position: Vec3::new(0.0, 10.0, 10.0),
        target:Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        fovy: 90.0,
        aspect: None,
        projection: Projection::Perspective,
        render_target: None,
        viewport: None
    };

    // Define 3d camera behauviour vars
    let distance = 30.0;
    let mut angle_around_target = 0.0;
    let mut pitch_angle = BouncingVariable::new(0.0, -75.0, 75.0, 0.1);

    // Set the state of the application
    // Start the Lorenz equation at 1, 1, 1
    let mut state = State {
        linestrip: vec![Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }],
    };

    loop {
        if state.linestrip.len() > 200000 {
            state.linestrip = vec![Vec3 {x: 1.0, y: 1.0, z: 1.0}]
        }

        // Calculate the next 30 steps in the Lorenz equation
        for _ in 0..30 {
            state.linestrip.push(lorenz(state.linestrip.last().unwrap()));
        }

        clear_background(BLACK);

        // rotate the camera on the horizontal plane
        if angle_around_target == 359.9 {
            angle_around_target = 0.0;
        } else {
            angle_around_target += 0.1;
        }

        // change angle of the camera on the y axis
        pitch_angle.step();

        // Find the center of all the points in linestrip to use as target to look at
        camera.target = find_center(&state.linestrip);


        orbit_camera(&mut camera, distance, angle_around_target, &pitch_angle.value);
        set_camera(&camera);

        // Loop through the linestrip and draw a line between each point in the vector
        for (i, _) in state.linestrip.iter().enumerate() {
            if i == 0 { continue; }
            draw_line_3d(state.linestrip[i - 1], state.linestrip[i], RED);
        }

        

        // render next frame
        next_frame().await
    }
}
