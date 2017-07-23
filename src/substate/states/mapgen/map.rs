use std::io::{BufReader, Write, Read};
use std::fs::{DirBuilder, File};

use uuid::Uuid;
use bincode::{serialize, deserialize, Infinite};

const REGION_DEPTH: u32 = 16;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Map {
    id: Uuid,
    width: u32,
    height: u32,
    region_size: u32,
    regions: Vec<Vec<Region>>,
    seed: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum BiomeType {
    Arid,
    Grassland,
    Ocean,
    Rocky,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Region {
    id: Uuid,
    width: u32,
    height: u32,
    biome: BiomeType,
    tiles: Vec<Vec<Vec<Tile>>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum TileType {
    Air,
    Grass,
    Sand,
    Soil,
    Stone,
    Water,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Tile {
    pub solid: bool,
    pub tile_type: TileType,
}

pub fn get_biome_name(biome: &BiomeType) -> String {
    match biome {
        &BiomeType::Arid => String::from("Arid"),
        &BiomeType::Grassland => String::from("Grassland"),
        &BiomeType::Ocean => String::from("Oceanic"),
        &BiomeType::Rocky => String::from("Rocky"),        
    }
}

impl Map {
    pub fn new(width: u32, height: u32, region_size: u32) -> Map {
        Map {
            id: Uuid::new_v4(),
            width: width,
            height: height,
            region_size: region_size,
            regions: Vec::<Vec<Region>>::new(),
            seed: -1,
        }
    }

    pub fn set_seed(&mut self, seed: i32) {
        self.seed = seed;
    }

    pub fn generate_regions<F>(&mut self, progress_callback: F)
    where
        F: Fn(i32),
    {
        let region_count = (self.width * self.height) as f32;
        let mut current_index = 0;

        for x in 0..self.width {
            let mut column = Vec::<Region>::new();
            for y in 0..self.height {
                if x % 3 == 0 || y % 2 == 0 {
                    column.push(Region::new(self.region_size, BiomeType::Arid));
                } else if x % 4 == 0 {
                    column.push(Region::new(self.region_size, BiomeType::Grassland));
                } else {
                    column.push(Region::new(self.region_size, BiomeType::Rocky));
                }

                current_index += 1;
                let progress = ((current_index as f32 / region_count) as i32) * 100;
                progress_callback(progress);
            }
            self.regions.push(column);
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn save(&mut self) {
        let path = format!("maps/{}/", self.id);
        let _ = DirBuilder::new()
            .recursive(true)
            .create(path.clone())
            .unwrap();

        for mut c in self.regions.iter_mut() {
            for mut r in c.iter_mut() {
                r.unload(path.clone());
            }
        }

        let fname = format!("{}{}.map", path, self.id);
        let mut file = File::create(fname).unwrap();
        let encoded = serialize(&self, Infinite).unwrap();
        file.write_all(encoded.as_slice()).unwrap();
    }

    pub fn load(&mut self, id: String) {
        let path = format!("maps/{}/", id);
        let file = File::open(format!("{}{}.map", path.clone(), id)).unwrap();
        let mut buffer = Vec::<u8>::new();
        let _ = BufReader::new(file).read_to_end(&mut buffer).unwrap();
        *self = deserialize(&buffer).unwrap();
    }

    pub fn load_region(&mut self, x: u32, y: u32) -> &mut Region {
        if x < self.width && y < self.height {
            let path = format!("maps/{}/", self.id);
            let r = &mut self.regions[x as usize][y as usize];
            r.load_tiles(path.clone());
            return r;
        }
        panic!("Region offset at ({},{}) is outside of map bounds!", x, y);
    }

    pub fn get_biome_at_offset(&self, x: u32, y: u32) -> BiomeType {
        if x < self.width && y < self.height {
            return self.regions[x as usize][y as usize].biome.clone();
        }
        panic!("Region offset at ({},{}) is outside of map bounds!", x, y);
    }
}

impl Region {
    pub fn new(size: u32, biome: BiomeType) -> Region {
        Region {
            id: Uuid::new_v4(),
            width: size,
            height: size,
            biome: biome,
            tiles: Vec::<Vec<Vec<Tile>>>::new(),
        }
    }

    pub fn generate_tiles(&mut self) {
        for _ in 0..self.width {
            let mut column = Vec::<Vec<Tile>>::new();
            for _ in 0..self.height {
                let mut width = Vec::<Tile>::new();

                for z in 0..REGION_DEPTH {
                    // TODO: Replace this with map generation logic
                    if z > REGION_DEPTH / 2 {
                        width.push(Tile {
                            solid: false,
                            tile_type: TileType::Air,
                        });
                    } else {
                        width.push(Tile {
                            solid: true,
                            tile_type: TileType::Stone,
                        });
                    }
                }

                column.push(width);
            }
            self.tiles.push(column);
        }
    }

    pub fn load_tiles(&mut self, dir: String) {
        let file = File::open(format!("{}{}.region", dir, self.id)).unwrap();
        let mut buffer = Vec::<u8>::new();
        let _ = BufReader::new(file).read_to_end(&mut buffer).unwrap();
        self.tiles = deserialize(&buffer).unwrap();
    }

    pub fn unload(&mut self, dir: String) {
        if self.tiles.len() > 0 {
            self.save_tiles(dir);
        }
        self.dispose_tiles();
    }

    fn save_tiles(&self, dir: String) {
        let fname = format!("{}{}.region", dir, self.id);
        let mut file = File::create(fname).unwrap();
        let encoded = serialize(&self.tiles, Infinite).unwrap();
        file.write_all(encoded.as_slice()).unwrap();
    }

    pub fn dispose_tiles(&mut self) {
        self.tiles.clear();
    }
}
