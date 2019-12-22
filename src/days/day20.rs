use crate::error::Error;
use self::map::{ Map };
use pathfinding::prelude::dijkstra;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let map = Map::from_str(input);
    let start = map.find_start();
    let end = map.find_end();

    {
        let steps = dijkstra(
            &start,
            |&xy| map
                .possible_moves(xy)
                .map(|(xy,_)| (xy,1)),
            |&xy| xy == end
        );
        println!("Star 1: {:?}", steps.map(|(_,n)| n).unwrap_or(0));
    }

    {
        let steps = dijkstra(
            &(start,0),
            |&(xy,level)| {
                let m = &map;
                map.possible_moves(xy)
                    // Can't go up a level if at level 0:
                    .filter(move |&(_,used_portal)| {
                        if level == 0 && used_portal && m.is_outer(xy) {
                            false
                        } else {
                            true
                        }
                    })
                    // If we used a portal, we changed level, so note that:
                    .map(move |(xy2,used_portal)| {
                        let o = if used_portal {
                            (xy2, level + if m.is_outer(xy) { -1 } else { 1 })
                        } else {
                            (xy2, level)
                        };
                        (o,1)
                    })
            },
            // We only finish when we are back on level 0:
            |&(xy,level)| xy == end && level == 0
        );
        println!("Star 2: {:?}", steps.map(|(_,n)| n).unwrap_or(0));
    }

    Ok(())
}

mod map {

    use std::collections::HashMap;

    #[derive(Debug,Clone)]
    pub struct Map {
        // map of coords to feature at those coords.
        inner: HashMap<(usize,usize),Feature>,
        // Location of exit and portal name for each coords you can transport from
        portal_locations: HashMap<(usize,usize),((usize,usize), [u8;2])>,
        // Size of the map
        width: usize,
        height: usize
    }

    impl Map {
        pub fn from_str(input: &str) -> Map {
            // make an ascii map first so that we can access chars by coords.
            let ascii_map: HashMap<(usize,usize),u8> = input
                .lines()
                .enumerate()
                .flat_map(|(y,line)| line.chars().enumerate().map(move |(x,c)| ((x,y),c as u8)))
                .collect();

            // Convert this into a proper feature based map with portals.
            let map: HashMap<(usize,usize),Feature> = ascii_map.iter().map(|(&xy,&c)| {
                let f = match c {
                    b'#' => Feature::Wall,
                    b'.' => Feature::Empty,
                    b'A' ..= b'Z' => {
                        if surrounding_items(xy, &ascii_map).any(|b| b == b'.') {
                            let other = surrounding_items(xy, &ascii_map)
                                .filter(|b| (b'A'..=b'Z').contains(b))
                                .next()
                                .unwrap();
                            let pair = if other < c { [other,c] } else { [c,other] };
                            Feature::Portal(pair)
                        } else {
                            Feature::Void
                        }
                    }
                    _ => Feature::Void
                };
                (xy,f)
            }).collect();

            // Store all of the portal locations for easy transportation.
            let portal_locations: HashMap<(usize,usize),((usize,usize), [u8;2])> = map.iter()
                .map(|(&xy,&f)| (xy,f))
                .filter(|&(_,f)| f == Feature::Empty)
                .filter_map(|(xy,_)| {
                    surrounding_items(xy, &map)
                        .filter_map(|f| f.get_portal())
                        .map(|p| (xy,p))
                        .next()
                })
                .filter_map(|(xy,p)| {
                    map.iter()
                        .filter(|&(_,&f)| f == Feature::Empty)
                        .map(|(&xy2,_)| xy2)
                        .filter(|&xy2| xy2 != xy)
                        .find(|&xy2| {
                            surrounding_items(xy2, &map)
                                .filter_map(|f| f.get_portal())
                                .any(|p2| p2 == p)
                        })
                        .map(|xy2| (xy, (xy2,p)))
                })
                .collect();

            // Get dimensions (this includes emptiness):
            let (width, height) = map.keys().fold((0,0), |(x1,y1),&(x2,y2)| (x1.max(x2),y1.max(y2)));

            Map { inner: map, portal_locations, width, height }
        }
        pub fn possible_moves(&self, pos: (usize,usize)) -> impl Iterator<Item=((usize,usize),bool)> + '_ {
            let normal_moves = surrounding(pos)
                .filter(move |xy| self.inner.get(xy) == Some(&Feature::Empty))
                .map(|xy| (xy,false));
            let portal_jump = self.portal_locations
                .get(&pos)
                .copied()
                .map(|(xy2,_)| (xy2,true));
            normal_moves.chain(portal_jump)
        }
        pub fn find_start(&self) -> (usize,usize) {
            self.find_portal([b'A',b'A']).unwrap()
        }
        pub fn find_end(&self) -> (usize,usize) {
            self.find_portal([b'Z',b'Z']).unwrap()
        }
        fn find_portal(&self, portal: [u8;2]) -> Option<(usize,usize)> {
            self.inner.iter().find(|&(&xy,&f)| {
                let is_empty = f == Feature::Empty;
                let next_to_portal = surrounding_items(xy, &self.inner)
                    .filter_map(|f| f.get_portal())
                    .any(|p| p == portal);
                is_empty && next_to_portal
            }).map(|(&xy,_)| xy)
        }
        pub fn is_outer(&self, (x,y): (usize,usize)) -> bool {
            x <= 2 || y <= 2 || x >= self.width - 2 || y >= self.height - 2
        }
    }

    fn surrounding_items<T: Clone>(pos: (usize,usize), map: &HashMap<(usize,usize),T>) -> impl Iterator<Item=T> + '_ {
        surrounding(pos)
            .filter_map(move |p| map.get(&p))
            .cloned()
    }

    fn surrounding((x,y): (usize,usize)) -> impl Iterator<Item=(usize,usize)> {
        let left = if x > 0 { Some((x-1,y)) } else { None };
        let right = Some((x+1,y));
        let up = if y > 0 { Some((x,y-1)) } else { None };
        let down = Some((x,y+1));
        left.into_iter().chain(right).chain(up).chain(down)
    }

    #[derive(Debug,Clone,Copy,Eq,PartialEq)]
    pub enum Feature {
        Portal([u8;2]),
        Empty,
        Void,
        Wall
    }

    impl Feature {
        pub fn get_portal(&self) -> Option<[u8;2]> {
            match self {
                Feature::Portal(id) => Some(*id),
                _ => None
            }
        }
    }

}