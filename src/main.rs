extern crate image;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use image::*;


const X_SIZE: usize = 395;
const Y_SIZE: usize = 500;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EGroundType {
    OpenLand,
    RoughMeadow,
    EasyMovementForest,
    SlowRunForest,
    WalkForest,
    ImpassibleVegetation,
    LakeSwampMarsh,
    PavedRoad,
    FootPath,
    OutOfBounds,
    NotSet
}

impl EGroundType {
    pub fn walk_cost(&self) -> i8 {
        match *self {
            EGroundType::OpenLand => 10i8,
            EGroundType::RoughMeadow => 15i8,
            EGroundType::EasyMovementForest => 12i8,
            EGroundType::SlowRunForest => 20i8,
            EGroundType::WalkForest => 25i8,
            EGroundType::ImpassibleVegetation => i8::max_value(),
            EGroundType::LakeSwampMarsh => i8::max_value(),
            EGroundType::PavedRoad => 1i8,
            EGroundType::FootPath => 5i8,
            EGroundType::OutOfBounds => i8::max_value(),
            EGroundType::NotSet => i8::max_value()
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tile {
    pub pos: Vec2,
    pub prev: Vec2,
    pub ground_type: EGroundType,
    pub h: f32,
    pub g: f32
}

impl Tile {
    pub fn new (pos: Vec2, ground_type: EGroundType) -> Tile {
        Tile {
            pos,
            prev: Vec2 {
                x: 0usize,
                y: 0usize
            },
            ground_type,
            h: 0f32,
            g: 0f32
        }
    }

    pub fn new_empty() -> Tile {
        Tile {
            pos: Vec2 {
                x: 0usize,
                y: 0usize
            },
            prev: Vec2 {
                x: 0usize,
                y: 0usize
            },
            ground_type: EGroundType::NotSet,
            h: 0f32,
            g: 0f32
        }
    }

    pub fn set_prev(&mut self, prev: Vec2) {
        self.prev = prev;
    }
}


pub fn _apply_season() {

}

//we don't care really about exact length we keep them all as the expanded version
#[inline]
pub fn calc_dist_sqr(a: &Vec2, b: &Vec2) -> f32{
    let x_dif = (b.x as f32) - (a.x as f32);
    let y_dif = (b.y as f32) - (a.x as f32);
    let x_dif = x_dif * x_dif;
    let y_dif = y_dif * y_dif;
    let ret = x_dif + y_dif;
    return ret.abs();
}


//the best tile is the tile that has the lowest score of g + h, that would mean it has the lowest cost with it
pub fn find_best_next_tile(possible_tile: &Vec<Tile>)  -> usize {
    
    let mut index = 0;
    let mut current_score = possible_tile[index].g + possible_tile[index].h;//calculating F

    for i in 0..possible_tile.len() {
        let this_score = possible_tile[i].g + possible_tile[i].h;
        if this_score < current_score {
            index = i;
            current_score = this_score;
        }
    }

    return index;
}


//the reason that this assumes a uniform cost to get from the current tile to the final tile is that we simply cannot calculate that cost without just running A* from that for each tile
//so instead be naive, and since they all have the same uniform cost, it should even out
#[inline]
pub fn set_h_for_all_tiles(map_as_colors: &mut Vec<Vec<Tile>>, end_pos: &Vec2) {
    for x in 0..X_SIZE {
        for y in 0..Y_SIZE {
            map_as_colors.get_mut(x).unwrap().get_mut(y).unwrap().h = calc_dist_sqr(&map_as_colors[x][y].pos, end_pos);
        }
    }
}

pub struct Vec2Float {
    pub x: f32,
    pub y: f32
}

pub fn get_adjacent(start: Vec2) -> Vec<Vec2> {

    let mut fill : Vec<Vec2Float> = vec![
        Vec2Float {
            x: start.x as f32 - 1f32,
            y: start.y as f32 - 1f32
        },
        Vec2Float {
            x: start.x as f32,
            y: start.y as f32 - 1f32
        },
        Vec2Float {
            x: start.x as f32,
            y: start.y as f32 - 1f32
        },

        Vec2Float {
            x: start.x as f32 + 1f32,
            y: start.y as f32 + 1f32
        },
        Vec2Float {
            x: start.x as f32 + 1f32,
            y: start.y as f32
        },
        Vec2Float {
            x: start.x as f32,
            y: start.y as f32 + 1f32
        },

        Vec2Float {
            x: start.x as f32 + 1f32,
            y: start.y as f32 - 1f32
        },

        Vec2Float {
            x: start.x as f32 - 1f32,
            y: start.y as f32 + 1f32
        },
    ];

    let first_ret = fill.into_iter().filter(|pos| {
        if pos.x < 0f32 || pos.y < 0f32 {
            return false;
        }

        if pos.x >= X_SIZE as f32 || pos.y >= Y_SIZE as f32{
            return false
        } 

        return true;
    });

    let return_vec = first_ret.map(|element| {
        Vec2 {
            x: element.x as usize,
            y: element.y as usize
        }
    });

    return_vec.collect()
}

#[inline]
pub fn create_final_path(start_pos: Vec2, end_pos: Vec2, map: &Vec<Vec<Tile>>) -> Vec<Tile> {
    let mut final_path   = vec![];

    let mut current = end_pos;
    while current != start_pos {
        final_path.push(map[current.x][current.y]);
        current = map[current.x][current.y].prev;
    }
    final_path.push(map[start_pos.x][start_pos.y]);

    final_path
}


pub fn find_path_with_a_star(map_as_colors: &mut Vec<Vec<Tile>>, start_pos: Vec2, end_pos: Vec2) -> Vec<Tile> {
    
    //set our huerestic
    set_h_for_all_tiles(map_as_colors, &end_pos);

    let mut possible_tiles = vec![];
    let mut seen_set : HashSet<Vec2> = HashSet::new();
    let mut prev: Vec2 = Vec2{x: start_pos.x, y: start_pos.y};

    possible_tiles.push(map_as_colors[start_pos.x][start_pos.y]);
    seen_set.insert(start_pos);

    while possible_tiles.len() != 0 {
        
        //get the best looking tile in the tiles we have not looked at yet
        let next_index = find_best_next_tile(&possible_tiles);

        println!("before {}", possible_tiles.len());
        //we do a look up by the index that find_best_next_tile returns
        let mut current = possible_tiles.remove(next_index);
        println!("afterc {}", possible_tiles.len());

        if current.pos == end_pos {
            return create_final_path(start_pos, end_pos, map_as_colors);
        }
        //set prev so we know what tile we came from
        current.prev = prev;
        //g is our walk cost plus the cost it took to get here
        current.g = current.ground_type.walk_cost() as f32 + map_as_colors[prev.x][prev.y].g;

        //we can prev to be set to use current now that we are done with the book keeping
        prev = current.pos;
        //set 

        //get those indexs that we can look up, that is greater then zero, less then the grid size
        let possibles = get_adjacent(prev);
        
        //add to the possible tiles those tiles that we have not seen yet
        let add_list : Vec<Vec2> = possibles.into_iter().filter(|element|{
            /*
            println!("|||");
            println!("{:?}", element);
            println!("{:?}", seen_set.contains(element));
            println!("---");
            */
            return !seen_set.contains(element);
        }).collect();

        for pos in add_list {
            
            println!("pos {:?}", pos);
            let element = map_as_colors[pos.x][pos.y];
            println!("element added {:?}", element);
            possible_tiles.push(map_as_colors[pos.x][pos.y]);
            seen_set.insert(pos);
        }
    }

    println!("{:?}", seen_set);
    vec![]
}



fn main() {
    let img = image::open(&Path::new("map.png")).unwrap();
    //setting up a map for us so that we can easily look up what the cost of a tile is

    let mut rgba_to_enum: HashMap<image::Rgba<u8>, EGroundType> = HashMap::new();
    rgba_to_enum.insert(image::Rgba::from_channels(248u8, 148u8, 18u8, 255u8), EGroundType::OpenLand);
    rgba_to_enum.insert(image::Rgba::from_channels(255u8, 192u8, 0u8, 255u8), EGroundType::RoughMeadow);
    rgba_to_enum.insert(image::Rgba::from_channels(255u8, 255u8, 255u8, 255u8), EGroundType::EasyMovementForest);
    rgba_to_enum.insert(image::Rgba::from_channels(2u8, 208u8, 60u8, 255u8), EGroundType::SlowRunForest);
    rgba_to_enum.insert(image::Rgba::from_channels(2u8, 136u8, 40u8, 255u8), EGroundType::WalkForest);
    rgba_to_enum.insert(image::Rgba::from_channels(5u8, 73u8, 24u8, 255u8), EGroundType::ImpassibleVegetation);
    rgba_to_enum.insert(image::Rgba::from_channels(0u8, 0u8, 255u8, 255u8), EGroundType::LakeSwampMarsh);
    rgba_to_enum.insert(image::Rgba::from_channels(71u8, 51u8, 3u8, 255u8), EGroundType::PavedRoad);
    rgba_to_enum.insert(image::Rgba::from_channels(0u8, 0u8, 0u8, 255u8), EGroundType::FootPath);
    rgba_to_enum.insert(image::Rgba::from_channels(0u8, 0u8, 0u8, 0u8), EGroundType::FootPath);//I am not sure if there is a 0, 0, 0, 255 white tile, but I but in both to be careful
    rgba_to_enum.insert(image::Rgba::from_channels(205, 0, 101, 255), EGroundType::OutOfBounds);    
    
    let i = img.to_rgba();
    let k = i.pixels();
    
    //preallocate all of our tiles, no need to have this cost
    let mut map_as_colors: Vec<Vec<Tile>> = vec![];
    for x in 0..X_SIZE {
        map_as_colors.push(Vec::new());
        for _y in 0..Y_SIZE {
            map_as_colors[x].push(Tile::new_empty());
        }
    }

    let mut count = 0;

    //we fill in all the tiles that we created
    for p in k {
        let terrian_type = rgba_to_enum.get(&p);
        match terrian_type {
            Some(tt) => {
                //fill the tiles with the data we need
                let mut tile = map_as_colors[count % X_SIZE][count / X_SIZE];
                tile.pos.x = count & X_SIZE;
                tile.pos.y = count / X_SIZE;
                tile.ground_type = tt.clone();
            }, 
            None => {}
        };
        count = count + 1;
    }

    let start_pos = Vec2{x: 0, y: 0};
    let end_pos = Vec2{x : 5, y: 5};
    let path = find_path_with_a_star(&mut map_as_colors, start_pos, end_pos);
    for node in path {
        println!("{:?}", node);
    }

}