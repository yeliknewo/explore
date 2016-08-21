pub struct System {
    multiplier: ::utils::Delta,
    target_delta_time: ::utils::Delta,
    count_time: ::utils::Delta,
    dwarf_count: usize,
}

impl System {
    pub fn new(target_delta_time: ::utils::Delta) -> System {
        System {
            multiplier: 0.0,
            target_delta_time: target_delta_time,
            count_time: 0.0,
            dwarf_count: 0,
        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, time: ::utils::Delta) {
        use specs::Join;

        let new_time = ::time::precise_time_s();

        let (tile_map, transform, clickable, dwarf, tiles, mut path_finding_data) = arg.fetch(|w|
            (w.read::<::comps::TileMap>(), w.read::<::comps::Transform>(), w.read::<::comps::Clickable>(), w.read::<::comps::Dwarf>(), w.read::<::comps::Tile>(), w.write::<::comps::PathFindingData>())
        );

        self.count_time += time;
        if self.count_time > 5.0 {
            self.dwarf_count = dwarf.iter().count();
            self.count_time = 0.0;
        }

        let target = {
            let mut target_opt = None;

            for (t, c) in (&transform, &clickable).iter() {
                if c.clicked {
                    target_opt = Some(t.get_pos());
                }
            }

            if target_opt.is_none() {
                return;
            }

            target_opt.unwrap()
        };

        let tm = {
            let mut tile_map_opt = None;

            for tm in (&tile_map).iter() {
                tile_map_opt = Some(tm);
            }

            if tile_map_opt.is_none() {
                error!("Tile map has not been initalized");
                return;
            }

            tile_map_opt.unwrap()
        };

        'closing: for (d, t, mut pfd) in (&dwarf, &transform, &mut path_finding_data).iter() {
            let target_i: ::math::Point2I = target.clone().into();

            if *pfd.get_mut_priority() > 0 {
                *pfd.get_mut_priority() = pfd.get_mut_priority().saturating_sub(1);
                continue;
            }

            if d.get_point_path().len() > 0 || target_i == t.get_pos().into() {
                continue;
            }

            let (mut nodes, mut open, mut closed) = match pfd.get_mut_path_data_opt().take() {
                Some(::comps::path_finding_data::PathData::Data(nodes, open, closed, old_target)) => {
                    if old_target != target {
                        (vec!(), vec!(), vec!())
                    } else {
                        (nodes, open, closed)
                    }
                },
                None => {
                    (vec!(), vec!(), vec!())
                },
            };

            if let Some(starting_tile) = tm.get_tile(&t.get_pos().into()) {
                open.push(nodes.len());
                nodes.push(::comps::path_finding_data::PathFindingNode::Start(*starting_tile, 0, 0.0));
            } else {
                warn!("dwarf is not on a tile");
                continue;
            }

            let mut end = None;

            while end.is_none() && open.len() > 0 {
                open.sort_by(|first: &usize, second: &usize| {
                    match nodes[*first] {
                        ::comps::path_finding_data::PathFindingNode::Start(entity, _, distance) |
                        ::comps::path_finding_data::PathFindingNode::Node(entity, _, distance) => {
                            distance + (transform.get(entity).expect("Tile had no transform").get_pos() - t.get_pos()).length() * 3.0
                        }
                    }.partial_cmp(&match nodes[*second] {
                        ::comps::path_finding_data::PathFindingNode::Start(entity, _, distance) |
                        ::comps::path_finding_data::PathFindingNode::Node(entity, _, distance) => {
                            distance + (transform.get(entity).expect("Tile had no transform").get_pos() - t.get_pos()).length() * 3.0
                        }
                    }).unwrap_or(::std::cmp::Ordering::Equal)
                });

                let node_index = open.remove(0);
                let node = nodes[node_index].clone();

                match node {
                    ::comps::path_finding_data::PathFindingNode::Start(entity, _, distance) |
                    ::comps::path_finding_data::PathFindingNode::Node(entity, _, distance) => {
                        if let Some(tile) = tiles.get(entity) {
                            for link in tile.get_fast_links() {
                                if closed.binary_search_by(|probe| {
                                    match nodes[*probe] {
                                        ::comps::path_finding_data::PathFindingNode::Node(entity, _, _) |
                                        ::comps::path_finding_data::PathFindingNode::Start(entity, _, _) => {
                                            entity
                                        }
                                    }.cmp(&link.0)
                                }).is_err() {
                                    open.push(nodes.len());
                                    nodes.push(::comps::path_finding_data::PathFindingNode::Node(link.0, node_index, distance + link.1));
                                }
                            }
                            for link in tile.get_links() {
                                if let Some(real_link) = tm.get_tile(&link.0) {
                                    if closed.binary_search_by(|probe| {
                                        match nodes[*probe] {
                                            ::comps::path_finding_data::PathFindingNode::Node(entity, _, _) |
                                            ::comps::path_finding_data::PathFindingNode::Start(entity, _, _) => {
                                                entity
                                            }
                                        }.cmp(&real_link)
                                    }).is_err() {
                                        open.push(nodes.len());
                                        nodes.push(::comps::path_finding_data::PathFindingNode::Node(*real_link, node_index, distance + link.1));
                                    }
                                }
                            }
                        }
                        if let Some(transform) = transform.get(entity) {
                            if transform.get_pos() == target {
                                if let Some(end_index) = end {
                                    let end_node = &nodes[end_index];
                                    match end_node {
                                        &::comps::path_finding_data::PathFindingNode::Node(_, _, end_distance) |
                                        &::comps::path_finding_data::PathFindingNode::Start(_, _, end_distance) => {
                                            if distance < end_distance {
                                                end = Some(node_index);
                                            }
                                        }
                                    }
                                } else {
                                    end = Some(node_index);
                                }
                            }
                        }
                    }
                }

                closed.push(node_index);

                closed.sort_by(|first: &usize, second: &usize| {
                    match nodes[*first] {
                        ::comps::path_finding_data::PathFindingNode::Start(entity, _, _) |
                        ::comps::path_finding_data::PathFindingNode::Node(entity, _, _) => {
                            entity
                        }
                    }.partial_cmp(&match nodes[*second] {
                        ::comps::path_finding_data::PathFindingNode::Start(entity, _, _) |
                        ::comps::path_finding_data::PathFindingNode::Node(entity, _, _) => {
                            entity
                        }
                    }).unwrap_or(::std::cmp::Ordering::Equal)
                });

                if ::time::precise_time_s() > new_time + time * self.multiplier {
                    *pfd.get_mut_path_data_opt() = Some(::comps::path_finding_data::PathData::Data(nodes.clone(), open, closed, target.clone()));
                    *pfd.get_mut_priority() = pfd.get_mut_priority().saturating_add(self.dwarf_count);
                    break 'closing;
                }
            }

            let mut entity_path = vec!();

            if let Some(end) = end {
                let mut current = end;

                while match nodes[current] {
                    ::comps::path_finding_data::PathFindingNode::Node(entity, from_index, _) => {
                        current = from_index;
                        entity_path.push(entity);
                        true
                    },
                    ::comps::path_finding_data::PathFindingNode::Start(entity, _, _) => {
                        entity_path.push(entity);
                        false
                    },
                } {

                }
            }

            let mut point_path = vec!();

            for entity in entity_path.drain(..) {
                point_path.push(transform.get(entity).unwrap().get_pos());
            }

            pfd.get_mut_path().append(&mut point_path);
            *pfd.get_mut_path_ready() = true;
        }

        self.multiplier -= time - self.target_delta_time;
        self.multiplier = self.multiplier.max(0.1).min(1.0);
        // warn!("real, target: {}, {}", time, self.target_delta_time);

        // warn!("multi: {}", self.multiplier);
    }
}
