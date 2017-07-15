pub struct Map {
    width: u8,
    height: u8,
    region_size: u32,
    regions: Vec<Vec<Region>>,
    seed: i32,
}

pub enum BiomeType {
    Arid,
    Grassland,
    Ocean,
    Rocky,
}

struct Region {
    width: u32,
    height: u32,
    biome: BiomeType,
    tiles: Vec<Vec<Vec<Tile>>>,
}

pub enum TileType {
    Air,
    Grass,
    Sand,
    Soil,
    Stone,
    Water,
}

struct Tile {
    pub solid: bool,
    pub tile_type: TileType,
}

impl Map {
    pub fn new(width: u8, height: u8, region_size: u32) -> Map {
        Map {
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
        where F: Fn(i32) {

    }
}