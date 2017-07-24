use std::collections::HashMap;
use std::f32::consts::PI;

use noise::{Seedable, Fbm, NoiseModule};

use super::map::BiomeType;

const SEED_MOD_ARID: usize = 2_196_648_173;
const SEED_MOD_GRASSLAND: usize = 4_288_859_287;
const SEED_MOD_OCEAN: usize = 1_880_045_689;
const SEED_MOD_ROCKY: usize = 688_937_993;
const NOISE_SCALE: f32 = 0.1;
const SMOOTHNESS: f32 = 1.570796;
const SEA_LEVEL: f32 = 0.0;
const VALLEY_MAX_STEEPNESS: f32 = 0.1;
const GRASSLAND_COVERAGE: f32 = 0.5;

pub struct RegionsGenerator {
    seed: usize,
    biome_noise: HashMap<BiomeType, Fbm<f32>>,
    terrain_noise: Fbm<f32>,
}

impl RegionsGenerator {
    pub fn new(seed: usize) -> RegionsGenerator {
        let mut noise_factories = HashMap::<BiomeType, Fbm<f32>>::new();

        RegionsGenerator::register_biome_type(&seed, &mut noise_factories, BiomeType::Arid);
        RegionsGenerator::register_biome_type(&seed, &mut noise_factories, BiomeType::Grassland);
        RegionsGenerator::register_biome_type(&seed, &mut noise_factories, BiomeType::Ocean);
        RegionsGenerator::register_biome_type(&seed, &mut noise_factories, BiomeType::Rocky);

        RegionsGenerator {
            seed: seed,
            biome_noise: noise_factories,
            terrain_noise: Fbm::<f32>::new().set_seed(seed),
        }
    }

    pub fn get_biome_at_point(&self, map_x: u32, map_y: u32) -> BiomeType {
        let x = NOISE_SCALE * (map_x as f32);
        let y = NOISE_SCALE * (map_y as f32);
        let local_avg = self.get_noise_neighbor_average(&self.terrain_noise, x, y);
        let height: f32 = self.terrain_noise.get([x, y]);
        let steepness = (height - local_avg).abs();
        
        if height <= SEA_LEVEL {
            BiomeType::Ocean
        } else {
            if steepness > VALLEY_MAX_STEEPNESS {
                BiomeType::Rocky
            } else {
                if let Some(factory) = self.biome_noise.get(&BiomeType::Grassland) {
                    if factory.get([x, y]) <= GRASSLAND_COVERAGE {
                        BiomeType::Grassland
                    } else {
                        BiomeType::Arid
                    }
                } else {
                    BiomeType::Arid
                }
            }
        }
    }

    fn get_noise_neighbor_average(&self, noise: &Fbm<f32>, x: f32, y: f32) -> f32 {
        let mut angle = 0.0;
        let mut total = 0.0;
        while angle <= 2.0 * PI {
            let nx = x + (NOISE_SCALE * angle.cos());
            let ny = y + (NOISE_SCALE * angle.sin());
            let val = noise.get([nx, ny]);
            total += val;
            angle += SMOOTHNESS * 1.0;
        }
        total / ((2.0 * PI) / (SMOOTHNESS * 1.0))
    }

    fn register_biome_type(seed: &usize, map: &mut HashMap<BiomeType, Fbm<f32>>, biome: BiomeType) {
        map.insert(
            biome.clone(), 
            Fbm::<f32>::new()
                .set_seed(
                    seed % RegionsGenerator::get_biome_seed(biome)
                )
        );
    }

    fn get_biome_seed(biome: BiomeType) -> usize {
        match biome {
            BiomeType::Arid => SEED_MOD_ARID,
            BiomeType::Grassland => SEED_MOD_GRASSLAND,
            BiomeType::Ocean => SEED_MOD_OCEAN,
            BiomeType::Rocky => SEED_MOD_ROCKY,
        }
    }
}


