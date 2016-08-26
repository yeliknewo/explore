use time::precise_time_s;
use specs::Entity;

use comps::path_finding_data::PathStatus;
use utils::Delta;

pub struct System {
    target_delta_time: Delta,
    multiplier: Delta,
    done_but_has_links: Vec<Entity>,
}

impl System {
    pub fn new(target_delta_time: Delta) -> System {
        System {
            target_delta_time: target_delta_time,
            multiplier: 0.1,
            done_but_has_links: vec!(),
        }
    }
}

impl ::specs::System<Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, delta_time: Delta) {
        use specs::Join;

        let new_time = ::time::precise_time_s();

        let (tiles, tile_map, mut path_finding_datas, mut paths_storage) = arg.fetch(|w|
            (
                w.read::<::comps::Tile>(),
                w.read_resource::<::comps::TileMap>(),
                w.write::<::comps::PathFindingData>(),
                w.write_resource::<::comps::PathsStorage>(),
            )
        );

        let mut current_status_level = PathStatus::Empty;

        let mut exit = false;

        while !exit && match current_status_level {
            PathStatus::Empty => {
                'closing: for (tile, mut path_finding_data) in (&tiles, &mut path_finding_datas).iter() {
                    if path_finding_data.get_entity().is_none() || match path_finding_data.get_path_status()  {
                        &PathStatus::Empty => false,
                        &PathStatus::DoneButHasLinks => true,
                        &PathStatus::WaitForNewTiles => true,
                    } {
                        continue;
                    }
                    let (mut nodes, mut open, mut closed) = match path_finding_data.get_mut_path_data_opt().take() {
                        Some((nodes, open, closed)) => {
                            // warn!("using old first tile");
                            (nodes, open, closed)
                        },
                        None => {
                            // warn!("switching to next first tile");
                            (
                                vec!((match tile_map.get_tile(&tile.get_location()) {
                                    Some(entity) => *entity,
                                    None => break,
                                }, 0usize, 0.0f64)),
                                vec!(0usize),
                                Vec::<usize>::new()
                            )
                        },
                    };

                    while open.len() > 0 {
                        open.sort_by(|first: &usize, second: &usize| {
                            nodes[*first].2.partial_cmp(&nodes[*second].2).unwrap_or(::std::cmp::Ordering::Equal)
                        });

                        let node_index = open.remove(0);
                        let node = nodes[node_index].clone();

                        if let Some(tile) = tiles.get(node.0) {
                            for link in tile.get_fast_links() {
                                if closed.binary_search_by(|probe| {
                                    nodes[*probe].0.cmp(&link.0)
                                }).is_err() {
                                    let distance = node.2 + link.1;
                                    let mut found = false;
                                    for open_index in &open {
                                        let open_node = nodes[*open_index].clone();
                                        if open_node.0 == node.0 {
                                            if distance < open_node.2 {
                                                nodes[*open_index].2 = distance;
                                                nodes[*open_index].1 = node_index;
                                            }
                                            found = true;
                                        }
                                    }
                                    if !found {
                                        open.push(nodes.len());
                                        nodes.push((link.0, node_index, distance));
                                    }
                                }
                            }
                        }

                        if let Err(index) = closed.binary_search_by(|probe| {
                            nodes[*probe].0.cmp(&node.0)
                        }) {
                            closed.insert(index, node_index);
                            closed.sort_by(|first, second|
                                nodes[*first].0.cmp(&nodes[*second].0)
                            );
                            let mut path = vec!();
                            let mut current = node_index;
                            let mut last = nodes.len();
                            while let Some(node) = nodes.get(current) {
                                if last == current {
                                    //at the start
                                    break;
                                }
                                last = current;
                                path.push(node.0);
                                current = node.1;
                            }
                            if path.len() >= 2 {
                                // warn!("path lengths: {}", path.len());
                                // warn!("first tile location: {}, last tile location: {}", tiles.get(*path.first().unwrap()).unwrap().get_location(), tiles.get(*path.last().unwrap()).unwrap().get_location());
                                paths_storage.add_path(*path.last().unwrap(), *path.first().unwrap(), path);
                            }
                        }

                        if precise_time_s() > new_time + delta_time * self.multiplier {
                            *path_finding_data.get_mut_path_data_opt() = Some((nodes, open, closed));
                            exit = true;
                            break 'closing;
                        }
                    }
                    // warn!("finished tile path");
                    if tile.get_links().is_empty() {
                        *path_finding_data.get_mut_path_status() = PathStatus::WaitForNewTiles;
                    } else {
                        *path_finding_data.get_mut_path_status() = PathStatus::DoneButHasLinks;
                        self.done_but_has_links.push(*path_finding_data.get_entity().unwrap())
                    }
                }
                current_status_level = PathStatus::DoneButHasLinks;
                // if !exit {
                //     warn!("finished filling in empty paths");
                // }
                true
            },
            PathStatus::DoneButHasLinks => {
                'closing2: loop {
                    if !self.done_but_has_links.is_empty() {
                        let first = self.done_but_has_links.remove(0);

                        let tile = tiles.get(first).unwrap();
                        let mut path_finding_data = path_finding_datas.get_mut(first).unwrap();

                        let (mut nodes, mut open, mut closed) = match path_finding_data.get_mut_path_data_opt().take() {
                            Some((nodes, open, closed)) => {
                                // warn!("using old first tile");
                                (nodes, open, closed)
                            },
                            None => {
                                // warn!("switching to next first tile");
                                (
                                    vec!((match tile_map.get_tile(&tile.get_location()) {
                                        Some(entity) => *entity,
                                        None => {
                                            exit = true;
                                            break 'closing2;
                                        },
                                    }, 0usize, 0.0f64)),
                                    vec!(0usize),
                                    Vec::<usize>::new()
                                )
                            },
                        };

                        while open.len() > 0 {
                            open.sort_by(|first: &usize, second: &usize| {
                                nodes[*first].2.partial_cmp(&nodes[*second].2).unwrap_or(::std::cmp::Ordering::Equal)
                            });

                            let node_index = open.remove(0);
                            let node = nodes[node_index].clone();

                            if let Some(tile) = tiles.get(node.0) {
                                for link in tile.get_fast_links() {
                                    if closed.binary_search_by(|probe| {
                                        nodes[*probe].0.cmp(&link.0)
                                    }).is_err() {
                                        let distance = node.2 + link.1;
                                        let mut found = false;
                                        for open_index in &open {
                                            let open_node = nodes[*open_index].clone();
                                            if open_node.0 == node.0 {
                                                if distance < open_node.2 {
                                                    nodes[*open_index].2 = distance;
                                                    nodes[*open_index].1 = node_index;
                                                }
                                                found = true;
                                            }
                                        }
                                        if !found {
                                            open.push(nodes.len());
                                            nodes.push((link.0, node_index, distance));
                                        }
                                    }
                                }
                            }

                            if let Err(index) = closed.binary_search_by(|probe| {
                                nodes[*probe].0.cmp(&node.0)
                            }) {
                                closed.insert(index, node_index);
                                closed.sort_by(|first, second|
                                    nodes[*first].0.cmp(&nodes[*second].0)
                                );
                                let mut path = vec!();
                                let mut current = node_index;
                                let mut last = nodes.len();
                                while let Some(node) = nodes.get(current) {
                                    if last == current {
                                        //at the start
                                        break;
                                    }
                                    last = current;
                                    path.push(node.0);
                                    current = node.1;
                                }
                                if path.len() >= 2 {
                                    // warn!("path lengths: {}", path.len());
                                    // warn!("first tile location: {}, last tile location: {}", tiles.get(*path.first().unwrap()).unwrap().get_location(), tiles.get(*path.last().unwrap()).unwrap().get_location());
                                    paths_storage.add_path(*path.last().unwrap(), *path.first().unwrap(), path);
                                }
                            }

                            if precise_time_s() > new_time + delta_time * self.multiplier {
                                *path_finding_data.get_mut_path_data_opt() = Some((nodes, open, closed));
                                exit = true;
                                break 'closing2;
                            }
                        }
                        // warn!("finished tile path");
                        if tile.get_links().is_empty() {
                            *path_finding_data.get_mut_path_status() = PathStatus::WaitForNewTiles;
                        } else {
                            self.done_but_has_links.push(first);
                        }
                    } else {
                        break 'closing2;
                    }
                }

                if self.done_but_has_links.is_empty() {
                    current_status_level = PathStatus::WaitForNewTiles;
                }

                true
            },
            PathStatus::WaitForNewTiles => {
                false
            },
        } {

        }

        // warn!("path finder pre mult: {}", self.multiplier);
        self.multiplier = (self.multiplier - (delta_time - self.target_delta_time)).max(0.1).min(1.0);
        // warn!("path finder post mult: {}", self.multiplier);
    }
}
