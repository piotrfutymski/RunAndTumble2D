use rand::{Rng, rngs::ThreadRng};
use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VertexState {
    Left, Right, Top, Bottom, None
}

impl VertexState {
    fn get_index(&self, x: usize, y: usize, size: usize) -> (usize, usize){
        match self {
            VertexState::Left => if x == 0 {(size-1, y)} else {(x-1, y)},
            VertexState::Right => if x == size-1 {(0, y)} else {(x+1, y)},
            VertexState::Top => if y == 0 {(x, size-1)} else {(x, y-1)},
            VertexState::Bottom => if y == size-1 {(x, 0)} else {(x, y+1)},
            VertexState::None => (x,y)
        }
    }
}

pub struct System{
    particles: Vec<Vec<VertexState>>,
    alfa: f64,
    size: usize
}

impl System {
    
    pub fn new(alfa: f64, size: usize, particle_count: i32) -> System{
        let density = (particle_count as f64)/((size * size) as f64);
        let mut rng = rand::thread_rng();
        let particles: Vec<Vec<VertexState>> = (0..size)
            .into_iter()
            .map(|_|(0..size).into_iter().map(|_|
                match rng.gen::<f64>() < density {
                    true => Self::generate_random_direction(&mut rng),
                    false => VertexState::None
                }
                ).collect()
            ).collect();
        System { alfa, particles, size }
    }

    pub fn get_particles(&self) -> &Vec<Vec<VertexState>> {&self.particles}

    pub fn step(&mut self){
        let mut set_to_exclude: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if !set_to_exclude.contains(&(i,j)) {
                    if self.particles[i][j] != VertexState::None {
                        let (x,y) = self.particles[i][j].get_index(i, j, self.size);
                        if self.particles[x][y] == VertexState::None {
                            self.particles[x][y] = self.particles[i][j];
                            self.particles[i][j] = VertexState::None;
                            set_to_exclude.insert((x,y));
                        }
                    }
                }
            }
        }
        self.change_directions();
    }

    fn change_directions(&mut self){
        let mut rng = rand::thread_rng();
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if self.particles[i][j] != VertexState::None && rng.gen::<f64>() < self.alfa{
                    let mut new_direction = Self::generate_random_direction(&mut rng);
                    while new_direction == self.particles[i][j] {
                        new_direction = Self::generate_random_direction(&mut rng)
                    }
                    self.particles[i][j] = new_direction;
                }
            }
        }
    }

    fn generate_random_direction(rng: &mut ThreadRng) -> VertexState{
        match (rng.gen::<f64>()*4.0) as i32 {
            0 => VertexState::Left,
            1 => VertexState::Right,
            2 => VertexState::Top,
            _ => VertexState::Bottom,
        }
    }

}