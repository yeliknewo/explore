pub struct System {

}

impl System {
    pub fn new() -> System {
        System {

        }
    }
}

impl ::specs::System<::utils::Delta> for System {
    fn run(&mut self, arg: ::specs::RunArg, _: ::utils::Delta) {
        use specs::Join;

        let (tile_map, transform, clickable, mut dwarf, tiles) = arg.fetch(|w|
            (w.read::<::comps::TileMap>(), w.read::<::comps::Transform>(), w.read::<::comps::Clickable>(), w.write::<::comps::Dwarf>(), w.read::<::comps::Tile>())
        );

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

        for (mut d, t) in (&mut dwarf, &transform).iter() {
            let target_i: ::math::Point2I = target.clone().into();

            if d.get_mut_point_path().len() > 0 || target_i == t.get_pos().into() {
                continue;
            }

            // warn!("dwarf position: {}", t.get_pos());
            // warn!("target position: {}", target);

            let mut nodes = vec!();

            let mut open = vec!();
            let mut open_next = vec!();

            if let Some(starting_tile) = tm.get_tile(&t.get_pos().into()) {
                open.push(nodes.len());
                nodes.push(PathFindingNode::Start(*starting_tile, 0, 0));
            } else {
                warn!("dwarf is not on a tile");
                continue;
            }

            let mut closed = vec!();

            let mut end = None;

            while end.is_none() && open.len() > 0 {
                open.sort_by(|first: &usize, second: &usize| {
                    match nodes[*first] {
                        PathFindingNode::Start(_, _, distance) |
                        PathFindingNode::Node(_, _, distance) => {
                            distance
                        }
                    }.cmp(&match nodes[*second] {
                        PathFindingNode::Start(_, _, distance) |
                        PathFindingNode::Node(_, _, distance) => {
                            distance
                        }
                    })
                });

                // warn!("starting open drain");
                let node_index = open.remove(0);
                let node = nodes[node_index].clone();

                // warn!("node distance: {}", match node {
                //     PathFindingNode::Start(_, _, distance) |
                //     PathFindingNode::Node(_, _, distance) => {
                //         distance
                //     },
                // });
                match node {
                    PathFindingNode::Start(entity, _, distance) |
                    PathFindingNode::Node(entity, _, distance) => {
                        if let Some(tile) = tiles.get(entity) {
                            for link in tile.get_fast_links() {
                                open_next.push(nodes.len());
                                nodes.push(PathFindingNode::Node(link.0, node_index, distance + link.1));
                            }
                            for link in tile.get_links() {
                                if let Some(real_link) = tm.get_tile(&link.0) {
                                    open_next.push(nodes.len());
                                    nodes.push(PathFindingNode::Node(*real_link, node_index, distance + link.1));
                                }
                            }
                        }
                        if let Some(transform) = transform.get(entity) {
                            if transform.get_pos() == target {
                                if let Some(end_index) = end {
                                    let end_node = &nodes[end_index];
                                    match end_node {
                                        &PathFindingNode::Node(_, _, end_distance) |
                                        &PathFindingNode::Start(_, _, end_distance) => {
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

                open.append(&mut open_next);
            }

            let mut entity_path = vec!();

            if let Some(end) = end {
                let mut current = end;

                while match nodes[current] {
                    PathFindingNode::Node(entity, from_index, _) => {
                        current = from_index;
                        entity_path.push(entity);
                        true
                    },
                    PathFindingNode::Start(entity, _, _) => {
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

            d.get_mut_point_path().append(&mut point_path);
        }
            //
            // for node_index in opens.remove(last_open).drain(..) {
            //     let node = nodes[node_index].clone();
            //     match node {
            //         PathFindingNode::Start(entity, _, distance) => {
            //             if let Some(tile) = tiles.get(entity) {
            //                 for link in tile.get_links() {
            //                     nodes.push(PathFindingNode::Node(entity, node_index, distance + link.1));
            //                     opens[next_open].push(nodes.len());
            //                 }
            //             }
            //             if let Some(transform) = transform.get(entity) {
            //                 if transform.get_pos() == target {
            //                     if let Some(in_end) = end.clone() {
            //                         match in_end {
            //                             PathFindingNode::Node(_, _, end_distance) => {
            //                                 if distance < end_distance {
            //                                     end = Some(node);
            //                                 }
            //                             },
            //                             PathFindingNode::Start(_, _, end_distance) => {
            //                                 if distance < end_distance {
            //                                     end = Some(node);
            //                                 }
            //                             },
            //                         }
            //                     } else {
            //                         end = Some(node);
            //                     }
            //                 }
            //             }
            //         },
            //         PathFindingNode::Node(entity, _, distance) => {
            //             if let Some(tile) = tiles.get(entity) {
            //                 for link in tile.get_links() {
            //                     nodes.push(PathFindingNode::Node(entity, node_index, distance + link.1));
            //                     opens[next_open].push(nodes.len());
            //                 }
            //             }
            //             if let Some(transform) = transform.get(entity) {
            //                 if transform.get_pos() == target {
            //                     if let Some(in_end) = end.clone() {
            //                         match in_end {
            //                             PathFindingNode::Node(_, _, end_distance) => {
            //                                 if distance < end_distance {
            //                                     end = Some(node);
            //                                 }
            //                             },
            //                             PathFindingNode::Start(_, _, end_distance) => {
            //                                 if distance < end_distance {
            //                                     end = Some(node);
            //                                 }
            //                             },
            //                         }
            //                     } else {
            //                         end = Some(node);
            //                     }
            //                 }
            //             }
            //         }
            //     }
            //
            //     closed.push(node_index);
            // }

            // if let Some(target) = target_opt.as_ref() {
            //     let offset_from_target = target.clone() - t.get_pos();
            //
            //     if offset_from_target.length() < p.get_speed_break().length() {
            //         l.idle();
            //     } else if offset_from_target.length() < d.get_speed() * time {
            //         l.walk_to(target.clone());
            //     } else {
            //         l.walk(offset_from_target.normalized() * d.get_speed());
            //     }
            // } else {
            //     l.idle();
            // }
    }
}

#[derive(Debug, Clone)]
enum PathFindingNode {
    Start(::specs::Entity, usize, u32),
    Node(::specs::Entity, usize, u32),
}
