use std::collections::{ HashSet, HashMap, VecDeque };
use pathfinding::prelude::fringe;
use crate::error::Error;
use self::map::{ Map, Feature };
use self::keys::{ Keys };

pub fn both_parts(input: &str) -> Result<(), Error> {
    let map = Map::from_str(input);
    let n_keys = map.count_keys();

    // Star 1: There's one start. Work out the best route to get
    // all of the keys..
    {
        let graph = build_graph(&map);
        let best_single_route = fringe(
            &(Feature::Start(0), Keys::new()),
            // Reachable locations from here:
            |&(f,k)| possible_moves(f,k,&graph).map(|(f,k,c)| ((f,k),c)),
            // Min possible distance from end:
            |(_,k)| n_keys - k.len(),
            // When are we done?
            |(_,k)| k.len() == n_keys
        );
        println!("Star 1: {:?}", best_single_route.unwrap().1);
    }

    // Star 2: There are 4 starts. Again, work out the best route
    // each robot can take from a start to pick up all of the keys.
    // This takes a couple of minutes to solve (wheras the above takes
    // a small numebr of ms..)
    {
        let map = map.make_4_starts();
        let graph = build_graph(&map);
        let best_multi_route = fringe(
            &([Feature::Start(0),Feature::Start(1),Feature::Start(2),Feature::Start(3)], Keys::new()),
            // Reachable locations from here:
            |&([f1,f2,f3,f4],k)| {
                let f1m = possible_moves(f1, k, &graph).map(move |(f,k,d)| (([f,f2,f3,f4],k),d));
                let f2m = possible_moves(f2, k, &graph).map(move |(f,k,d)| (([f1,f,f3,f4],k),d));
                let f3m = possible_moves(f3, k, &graph).map(move |(f,k,d)| (([f1,f2,f,f4],k),d));
                let f4m = possible_moves(f4, k, &graph).map(move |(f,k,d)| (([f1,f2,f3,f],k),d));
                f1m.chain(f2m).chain(f3m).chain(f4m)
            },
            // Min possible distance from end:
            |(_,k)| n_keys - k.len(),
            // When are we done?
            |(_,k)| k.len() == n_keys
        );
        println!("Star 2: {:?}", best_multi_route.unwrap().1);
    }

    Ok(())
}

// For a given feature and set of Keys, what are the possible resulting features, keys and cost of each move
fn possible_moves(feature: Feature, keys: Keys, graph: &Graph) -> impl Iterator<Item=(Feature,Keys,usize)> + '_ {
    graph.get(&feature)
        .unwrap()
        .into_iter()
        .filter(move |(f,_)| {
            match f {
                Feature::Door(d) => keys.contains(*d),
                _ => true
            }
        }).map(move |&(f,d)| {
            match f {
                Feature::Key(new_k) => (f,keys.with(new_k),d),
                _ => (f,keys,d)
            }
        })
}

/// Iterate over the closest reachable features (keys and doors) and their distances from some location.
fn features_in_range<'a>(location: (i16,i16), map: &'a Map) -> impl Iterator<Item=(Feature, usize)> + 'a {
    let mut seen = HashSet::new();
    seen.insert(location);
    let mut next: VecDeque<_> = step(location, map).map(|xy| (xy,1)).collect();
    std::iter::from_fn(move || {
        let (xy, dist) = if let Some(o) = next.pop_front() {
            o
        } else {
            return None
        };

        let feature = map.get(xy);
        let keep_moving = !feature.is_door_or_key();

        if keep_moving {
            for next_xy in step(xy, map) {
                if !seen.contains(&next_xy) {
                    next.push_back((next_xy, dist+1));
                    seen.insert(next_xy);
                }
            }
        }

        if !keep_moving {
            Some(Some((feature, dist)))
        } else {
            Some(None)
        }
    }).filter_map(|r| r)
}

/// Find the surrounding coords we can step to from our current position.
fn step<'a>((x,y): (i16,i16), map: &'a Map) -> impl Iterator<Item=(i16,i16)> + 'a {
    [(0,1),(1,0),(-1,0),(0,-1)]
        .into_iter()
        .map(move |&(xd,yd)| (x+xd,y+yd))
        .filter(move |&(x,y)| {
            match map.get((x,y)) {
                Feature::Key(_) | Feature::Door(_) | Feature::Empty => true,
                _ => false
            }
        })
}

// Turn the Map into a graph of connected locations of interest with distances.
fn build_graph(map: &Map) -> Graph {
    let mut graph = HashMap::new();
    for (xy,f) in map.iter().filter(|(_,f)| f.is_door_or_key() || f.is_start()) {
        graph.insert(f, vec![]);
        for (other_f, dist) in features_in_range(xy,map) {
            graph.get_mut(&f).unwrap().push((other_f, dist));
        }
    }
    graph
}

type Graph = HashMap<Feature, Vec<(Feature,usize)>>;

/// An efficient means to work with keys/doors
pub mod keys {

    #[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
    pub struct Key(u8);

    impl Key {
        pub fn from_char(c: char) -> Key {
            Key(c.to_ascii_uppercase() as u8)
        }
        fn mask(&self) -> u32 {
            1u32 << self.0 - b'A'
        }
    }

    #[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
    pub struct Keys(u32);

    impl Keys {
        pub fn new() -> Keys {
            Keys(0)
        }
        pub fn len(&self) -> usize {
            self.0.count_ones() as usize
        }
        pub fn with(self, key: Key) -> Keys {
            Keys(self.0 | key.mask())
        }
        pub fn contains(&self, key: Key) -> bool {
            self.0 & key.mask() > 0
        }
    }

}

/// A map of the area.
pub mod map {
    use std::collections::HashMap;
    use super::keys::Key;

    #[derive(Debug,Clone)]
    pub struct Map {
        inner: HashMap<(i16,i16), Feature>
    }

    impl Map {
        pub fn from_str(input: &str) -> Map {
            let inner = input.trim().lines().enumerate().flat_map(|(y, line)| {
                line.trim().chars().enumerate().map(move |(x, c)| {
                    let f = match c {
                        '#' => Feature::Wall,
                        'A'..='Z' => Feature::Door(Key::from_char(c)),
                        'a'..='z' => Feature::Key(Key::from_char(c)),
                        '@' => Feature::Start(0),
                        _ => Feature::Empty,
                    };
                    ((x as i16,y as i16), f)
                })
            }).collect();
            Map { inner }
        }
        pub fn iter(&self) -> impl Iterator<Item=((i16,i16),Feature)> + '_ {
            self.inner.iter().map(|(&d,&f)| (d,f))
        }
        pub fn count_keys(&self) -> usize {
            self.inner.values()
                .filter(|v| if let Feature::Key(_) = v { true } else { false } )
                .count()
        }
        pub fn get(&self, xy: (i16,i16)) -> Feature {
            self.inner.get(&xy).map(|&f| f).unwrap_or(Feature::Empty)
        }
        pub fn make_4_starts(self) -> Map {
            let mut inner = self.inner;
            let (x,y) = *inner.iter().find(|(_,f)| f.is_start()).unwrap().0;
            inner.insert((x,y), Feature::Wall);
            inner.insert((x-1,y), Feature::Wall);
            inner.insert((x+1,y), Feature::Wall);
            inner.insert((x,y-1), Feature::Wall);
            inner.insert((x,y+1), Feature::Wall);
            inner.insert((x-1,y-1), Feature::Start(0));
            inner.insert((x+1,y+1), Feature::Start(1));
            inner.insert((x+1,y-1), Feature::Start(2));
            inner.insert((x-1,y+1), Feature::Start(3));
            Map { inner }
        }
    }

    #[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
    pub enum Feature {
        Empty,
        Wall,
        Start(u8),
        Door(Key),
        Key(Key)
    }

    impl Feature {
        pub fn is_door_or_key(&self) -> bool {
            match self {
                Feature::Door(_) | Feature::Key(_) => true,
                _ => false
            }
        }
        pub fn is_start(&self) -> bool {
            if let Feature::Start(_) = self { true } else { false }
        }
    }
}
