
/*
//// NOTE ////
Algorithms adapted to Rust from weigert's SimpleHidrology: https://github.com/weigert/SimpleHydrology with his permission and support.
A standalone version of the Rust implementation can be found on my GitHub: https://github.com/skalimoi/SimpleHydrologyRust
*/

pub mod map {
    use crate::erosion::world::Vec2;
    use errorfunctions::RealErrorFunctions;
    use nalgebra::Vector3;

    #[derive(Default)]
    pub struct Cell {
        pub height: f64,
        pub discharge: f64,
        pub momentum: Vec2,

        pub discharge_track: f64,
        pub momentum_track: Vec2,

        pub rootdensity: f64,
    }

    pub struct Map {
        pub width: usize,
        pub height: usize,
        pub heightmap: Vec<Cell>,
    }

    impl Map {
        pub fn new(width: usize, height: usize, heightmap: Vec<u16>) -> Self {
            Self {
                width,
                height,
                heightmap: heightmap
                    .into_iter()
                    .map(|x| Cell {
                        height: x as f64 / 255.0,
                        ..Default::default()
                    })
                    .collect(),
            }
        }
        pub fn get(&self, pos: Vec2) -> Option<&Cell> {
            self.heightmap
                .get(self.width * pos.y as usize + pos.x as usize)
        }
        pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Cell> {
            self.heightmap
                .get_mut(self.width * pos.y as usize + pos.x as usize)
        }
        pub fn oob(&self, pos: Vec2) -> bool {
            pos.x < 0.0 || pos.y < 0.0 || pos.x >= self.width as f64 || pos.y >= self.height as f64
        }
        pub fn height(&self, pos: Vec2) -> f64 {
            if self.oob(pos) {
                return 0.0;
            }
            self.get(pos).map(|x| x.height).unwrap_or(0.0)
        }
        pub fn discharge(&self, pos: Vec2) -> f64 {
            if self.oob(pos) {
                return 0.0;
            }
            self.get(pos)
                .map(|x| (0.4 * x.discharge as f64).erf() as f64)
                .unwrap_or(0.0)
        }
        pub fn normal(&self, pos: Vec2) -> Vector3<f64> {
            let mut normal = Vector3::zeros();
            let scale = Vector3::new(1.0, 80.0, 1.0);

            if !self.oob(pos + Vec2::new(1.0, 1.0)) {
                normal += scale
                    .component_mul(&Vector3::new(
                        0.0,
                        self.height(pos + Vec2::new(0.0, 1.0)) - self.height(pos),
                        1.0,
                    ))
                    .cross(&scale.component_mul(&Vector3::new(
                        1.0,
                        self.height(pos + Vec2::new(1.0, 0.0)) - self.height(pos),
                        0.0,
                    )));
            }

            if !self.oob(pos + Vec2::new(-1.0, -1.0)) {
                normal += scale
                    .component_mul(&Vector3::new(
                        0.0,
                        self.height(pos - Vec2::new(0.0, 1.0)) - self.height(pos),
                        -1.0,
                    ))
                    .cross(&scale.component_mul(&Vector3::new(
                        -1.0,
                        self.height(pos - Vec2::new(1.0, 0.0)) - self.height(pos),
                        0.0,
                    )));
            }

            //Two Alternative Planes (+X -> -Y) (-X -> +Y)
            if !self.oob(pos + Vec2::new(1.0, -1.0)) {
                normal += scale
                    .component_mul(&Vector3::new(
                        1.0,
                        self.height(pos + Vec2::new(1.0, 0.0)) - self.height(pos),
                        0.0,
                    ))
                    .cross(&scale.component_mul(&Vector3::new(
                        0.0,
                        self.height(pos - Vec2::new(0.0, 1.0)) - self.height(pos),
                        -1.0,
                    )));
            }

            if !self.oob(pos + Vec2::new(-1.0, 1.0)) {
                normal += scale
                    .component_mul(&Vector3::new(
                        -1.0,
                        self.height(pos - Vec2::new(1.0, 0.0)) - self.height(pos),
                        0.0,
                    ))
                    .cross(&scale.component_mul(&Vector3::new(
                        0.0,
                        self.height(pos + Vec2::new(0.0, 1.0)) - self.height(pos),
                        1.0,
                    )));
            }
            if normal.magnitude() > 0.0 {
                normal = normal.normalize();
            }
            normal
        }
    }
}

pub mod world {
    use godot::prelude::real;
    use nalgebra::Vector2;
    use rand::Rng;
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::erosion::map::Map;
    use crate::erosion::water;

    pub type Vec2 = Vector2<f64>;

    pub struct World {
        pub seed: i16,
        pub map: Map,

        pub lrate: f64,
        pub discharge_thresh: f64,
        pub maxdiff: f64,
        pub settling: f64,

        rng: ChaCha8Rng,
    }

    impl World {
        pub fn new(heightmap: Vec<u16>, width: usize, height: usize, seed: i16) -> Self {
            Self {
                seed,
                map: Map::new(width, height, heightmap),
                lrate: 0.1,
                discharge_thresh: 0.0,
                maxdiff: 0.01,
                settling: 0.8,
                rng: ChaCha8Rng::seed_from_u64(seed as u64),
            }
        }
        pub fn erode(&mut self, cycles: usize) {
            self.map.heightmap.iter_mut().for_each(|cell| {
                cell.discharge_track = 0.0;
                cell.momentum_track = Vec2::zeros();
            });
            for _ in 0..cycles {
                let pos = Vec2::new(
                    self.rng.gen_range(0..self.map.width) as f64,
                    self.rng.gen_range(0..self.map.height) as f64,
                );
                if self.map.height(pos) < 0.1 {
                    continue;
                }
                let mut drop = water::Drop::new(pos);
                while drop.decend(self) {}
            }
            self.map.heightmap.iter_mut().for_each(|cell| {
                cell.discharge =
                    (1.0 - self.lrate) * cell.discharge + self.lrate * cell.discharge_track;
                cell.momentum =
                    (1.0 - self.lrate) * cell.momentum + self.lrate * cell.momentum_track;
            });
        }
        pub fn cascade(&mut self, prev_pos: Vec2) {
            let mut neighbors = Vec::new();
            for x in -1..=1 {
                for y in -1..=1 {
                    let offset = Vec2::new(x as f64, y as f64);
                    let npos = prev_pos + offset;
                    if self.map.oob(npos) || npos == prev_pos {
                        continue;
                    }
                    neighbors.push((npos, self.map.height(npos), offset.magnitude()))
                }
            }
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            for neighbor in neighbors {
                let (npos, height, distance) = neighbor;

                let diff = self.map.height(prev_pos) - height;
                if diff == 0.0 {
                    continue;
                }

                let excess = if height > 0.1 {
                    diff.abs() - distance * self.maxdiff
                } else {
                    diff.abs()
                };

                if excess <= 0.0 {
                    continue;
                }

                let transfer = self.settling * excess / 2.0;

                if diff > 0.0 {
                    self.map.get_mut(prev_pos).unwrap().height -= transfer;
                    self.map.get_mut(npos).unwrap().height += transfer;
                } else {
                    self.map.get_mut(prev_pos).unwrap().height += transfer;
                    self.map.get_mut(npos).unwrap().height -= transfer;
                }
            }
        }
    }
}

pub mod water {
    use crate::erosion::world::{Vec2, World};

    pub struct Drop {
        age: i32,
        pos: Vec2,
        speed: Vec2,

        volume: f64,
        sediment: f64,

        max_age: i32,
        min_vol: f64,
        evap_rate: f64,
        deposition_rate: f64,
        entrainment: f64,
        gravity: f64,
        momentum_transfer: f64,
    }

    impl Drop {
        pub fn new(pos: Vec2) -> Self {
            Self {
                age: 0,
                pos,
                speed: Vec2::new(0.0, 0.0),
                volume: 1.0,
                sediment: 0.0,

                max_age: 500,
                min_vol: 0.01,
                evap_rate: 0.001,
                deposition_rate: 0.1,
                entrainment: 10.0,
                gravity: 1.0,
                momentum_transfer: 1.0,
            }
        }
        pub fn decend(&mut self, world: &mut World) -> bool {
            let prev_pos = self.pos;
            let normal_vector = world.map.normal(prev_pos);
            let Some(cell) = world.map.get_mut(prev_pos) else {
                return false;
            };

            if self.age > self.max_age || self.volume < self.min_vol {
                cell.height += self.sediment;
                return false;
            };

            let eff_d = (self.deposition_rate * (1.0 - cell.rootdensity)).max(0.0);

            self.speed += self.gravity * Vec2::new(normal_vector.x, normal_vector.z) / self.volume;
            if cell.momentum.magnitude() > 0.0 && self.speed.magnitude() > 0.0 {
                self.speed += self.momentum_transfer
                    * cell.momentum.normalize().dot(&self.speed.normalize())
                    / (self.volume + cell.discharge)
                    * cell.momentum;
            };

            if self.speed.magnitude() > 0.0 {
                self.speed = 2.0f64.sqrt() * self.speed.normalize();
            };

            self.pos += self.speed;

            cell.discharge_track += self.volume;
            cell.momentum_track += self.volume * self.speed;

            let Some(cell) = world.map.get(prev_pos) else {
                return false;
            };

            let h2 = if world.map.oob(self.pos) {
                cell.height - 0.002
            } else {
                world.map.height(self.pos)
            };

            let c_eq = ((1.0 + self.entrainment * world.map.discharge(prev_pos))
                * (cell.height - h2))
                .max(0.0);
            let cdiff = c_eq - self.sediment;

            self.sediment += eff_d * cdiff;
            let Some(cell) = world.map.get_mut(prev_pos) else {
                return false;
            };
            cell.height -= eff_d * cdiff;

            self.sediment /= 1.0 - self.evap_rate;
            self.volume *= 1.0 - self.evap_rate;

            if world.map.oob(self.pos) {
                self.volume = 0.0;
                return false;
            };

            world.cascade(self.pos);
            self.age += 1;
            true
        }
    }
}

