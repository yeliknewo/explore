use time::precise_time_s;

use utils::Delta;

pub struct System {
    target_delta_time: Delta,
    multiplier: Delta,
}

impl System {
    pub fn new(target_delta_time: Delta) -> System {
        System {
            target_delta_time: target_delta_time,
            multiplier: 0.1,
        }
    }
}

impl ::specs::System<Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, delta_time: Delta) {
        use specs::Join;

        let new_time = ::time::precise_time_s();

        let (tile_map, transforms, tiles, mut path_finding_datas, mut paths_storage) = arg.fetch(|w|
            (
                w.read_resource::<::comps::TileMap>(),
                w.read::<::comps::Transform>(),
                w.read::<::comps::Tile>(),
                w.write::<::comps::PathFindingData>(),
                w.write_resource::<::comps::PathsStorage>()
            )
        );

        'closing: for (tile, transform, mut path_finding_data) in (&tiles, &transforms, &mut path_finding_datas).iter() {
            let (mut nodes, mut open, mut closed) = match path_finding_data.get_mut_path_data_opt().take() {
                Some((nodes, open, closed)) => {
                    (nodes, open, closed)
                },
                None => {
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
                                if open_node.1 == node_index {
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
                    path.reverse();
                    if path.len() >= 2 {
                        paths_storage.add_path(*path.first().unwrap(), *path.last().unwrap(), path);
                    }
                }

                if precise_time_s() > new_time + delta_time * self.multiplier {
                    *path_finding_data.get_mut_path_data_opt() = Some((nodes, open, closed));
                    break 'closing;
                }
            }
        }

        self.multiplier = (self.multiplier - (delta_time - self.target_delta_time)).max(0.1).min(1.0);
    }
}
