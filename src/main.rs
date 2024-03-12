use ::rand::prelude::*;
use macroquad::prelude::*;

const NUM_PARTICLES: usize = 100;
const PARTICLE_RADIUS: f32 = 5.0;
const PARTICLE_SPEED: f32 = 2.0;

struct Particle {
    position: Vec2,
    velocity: Vec2,
    connected_to: Option<usize>
}

impl Particle {
    fn new() -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        let vx = rng.gen_range(-PARTICLE_SPEED..PARTICLE_SPEED);
        let vy = rng.gen_range(-PARTICLE_SPEED..PARTICLE_SPEED);
        Particle {
            position: vec2(x, y),
            velocity: vec2(vx, vy),
            connected_to: None,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;

        // Colisión con las paredes de la pantalla
        if self.position.x < 0.0 || self.position.x > screen_width() {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y < 0.0 || self.position.y > screen_height() {
            self.velocity.y = -self.velocity.y;
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, PARTICLE_RADIUS, WHITE);
    }
}

#[macroquad::main("Particle Simulation")]
async fn main() {
    let mut particles = Vec::new();

    for _ in 0..NUM_PARTICLES {
        particles.push(Particle::new());
    }

    loop {
        clear_background(BLACK);

        // Encontrar la partícula más cercana para cada partícula
        for i in 0..particles.len() {
            let mut min_distance = f32::MAX;
            let mut closest_particle = None;

            for j in 0..particles.len() {
                if i != j {
                    let distance = particles[i].position.distance(particles[j].position);
                    if distance < min_distance {
                        min_distance = distance;                        closest_particle = Some(j);
                    }
                }
            }

            particles[i].connected_to = closest_particle;
        }

        // Dibujar las partículas y las líneas
        for i in 0..particles.len() {
            particles[i].update();
            particles[i].draw();

            if let Some(j) = particles[i].connected_to {
                draw_line(
                    particles[i].position.x,
                    particles[i].position.y,
                    particles[j].position.x,
                    particles[j].position.y,                    1.0,
                    GRAY,
                );
            }
        }

        next_frame().await
    }
}