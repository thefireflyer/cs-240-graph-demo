///////////////////////////////////////////////////////////////////////////////

use std::{f32::consts::PI, path::PathBuf};

use cs_240_library::{
    algorithms::graphs::dfs::depth_first_search,
    data_structures::graphs::{
        weighted_graph::WeightedGraph, IDefiniteGraph, IGraph, IGraphEdgeWeightedMut, IGraphMut,
        IWeightedGraph,
    },
};
use egui::{
    ahash::{HashMap, HashMapExt},
    emath::TSTransform,
    Pos2, Vec2,
};

///////////////////////////////////////////////////////////////////////////////

fn typing(x: usize) -> f32 {
    f32::from(i16::try_from(x).unwrap())
}

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Page {
    Blank,
    Project(Project),
}

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub path: Option<PathBuf>,
    pub view: PanZoom,
    pub graph: WeightedGraph<String, i32>,
    pub text: String,
    pub graphic: HashMap<String, Pos2>,
}

//---------------------------------------------------------------------------//

impl Project {
    //---------------------------------------------------------------------------//

    pub fn new() -> Self {
        let mut graph = WeightedGraph::new();

        graph.insert_node("node 1".to_owned());
        graph.insert_node("node 2".to_owned());
        graph.insert_node("node 3".to_owned());
        graph.insert_node("node 4".to_owned());

        graph.insert_edge_weighted("node 1".to_owned(), "node 2".to_owned(), 1);
        graph.insert_edge_weighted("node 1".to_owned(), "node 3".to_owned(), 2);
        graph.insert_edge_weighted("node 2".to_owned(), "node 4".to_owned(), 1);

        let text = serde_yaml::to_string(&graph).unwrap_or_default();

        let mut res = Self {
            path: None,
            view: Default::default(),
            graph,
            text,
            graphic: Default::default(),
        };

        res.update_graphic();

        res
    }

    //---------------------------------------------------------------------------//

    pub fn update_graph(&mut self) {
        match serde_yaml::from_str(&self.text) {
            Ok(val) => {
                if self.check(&val) {
                    self.graph = val;
                    self.update_graphic();
                }
            }
            Err(_) => {}
        }
    }

    //---------------------------------------------------------------------------//

    pub fn check(&mut self, graph: &WeightedGraph<String, i32>) -> bool {
        for node in graph.get_all() {
            for adj in graph.get_adj(&node) {
                if !graph.contains(&adj) {
                    return false;
                }
            }
        }
        true
    }

    //---------------------------------------------------------------------------//

    fn update_graphic(&mut self) {
        self.graphic.clear();
        let (roots, mut order, cyclic) = depth_first_search(self.graph.clone());

        if cyclic {
            println!("Cyclic graph");
            let nodes = self.graph.get_all();
            let len = typing(nodes.len());

            for (i, node) in nodes.into_iter().enumerate() {
                self.graphic.insert(
                    node,
                    Pos2 {
                        x: (2.0 * PI * typing(i) / len).cos() * 50.0 * len + 300.0,
                        y: (2.0 * PI * typing(i) / len).sin() * 50.0 * len + 300.0,
                    },
                );
            }
            self.simulate();
            // self.smacof();
        } else {
            println!("Acyclic graph");

            let mut x = 0.0;
            let mut y = 0.0;

            for root in roots {
                y = 0.0;
                x = self.update_graphic_rec(root, x + 100.0, y, 1);
            }
        }
    }

    //---------------------------------------------------------------------------//

    fn update_graphic_rec(&mut self, root: String, mut x: f32, mut y: f32, weight: i32) -> f32 {
        let mut adj = self.graph.get_adj_weighted(&root).into_iter();

        let offset = f32::from(weight as u16) * 100.0;
        println!("{}", offset);
        y += offset;

        self.graphic.insert(root.clone(), Pos2 { x, y });

        if let Some((first, weight)) = adj.next() {
            x = self.update_graphic_rec(first, x, y, weight);
        }

        for (node, weight) in adj {
            // self.graphic.insert(node.clone(), Pos2 { x, y });
            x = self.update_graphic_rec(node, x + 100.0, y, weight);
        }

        x
    }

    //---------------------------------------------------------------------------//

    pub fn smacof(&mut self) {}

    //---------------------------------------------------------------------------//

    pub fn stress(&mut self) {}

    //---------------------------------------------------------------------------//

    pub fn simulate(&mut self) {
        let mut velocities = HashMap::new();
        let mut accelerations = HashMap::new();

        for node in self.graph.get_all() {
            velocities.insert(node.clone(), Vec2 { x: 0.0, y: 0.0 });
            accelerations.insert(node, Vec2 { x: 0.0, y: 0.0 });
        }

        let steps = 300;
        for step in 1..steps + 1 {
            // println!("--- {}/{}", step, steps);

            for node in self.graph.get_all() {
                let node_pos = self.graphic.get(&node).unwrap();
                // println!("> {} @ {:?}", node, node_pos);

                let adj_nodes = self.graph.get_adj_weighted(&node);

                for other in self.graph.get_all() {
                    if other != node {
                        let adj_pos = self.graphic.get(&other).unwrap();

                        let diff = adj_pos.to_vec2() - node_pos.to_vec2();
                        let dist = (diff).length();

                        let spring_char;
                        let relaxed_dist;
                        let node_mass;

                        let mut num_connections = 0;
                        let mut total_weight = 0;

                        for (_, weight) in adj_nodes.iter().filter(|(x, _)| *x == other) {
                            num_connections += 1;
                            total_weight += weight;
                        }

                        for (_, weight) in self
                            .graph
                            .get_adj_weighted(&other)
                            .iter()
                            .filter(|(x, _)| *x == node)
                        {
                            num_connections += 1;
                            total_weight += weight;
                        }

                        if num_connections > 0 {
                            let mean_weight = total_weight / num_connections;

                            spring_char = 0.7;
                            relaxed_dist = f32::from(mean_weight as u16) * 100.0;
                            node_mass = 100.0;
                        } else {
                            spring_char = 0.5;
                            relaxed_dist = 300.0;
                            node_mass = 500.0;
                        }

                        // if adj_nodes.iter().any(|(x, _)| *x == other)
                        //     || self.graph.get_adj(&other).contains(&node)
                        // {
                        //     spring_char = 0.7;
                        //     relaxed_dist = 100.0;
                        //     node_mass = 100.0;
                        // } else {
                        //     spring_char = 0.5;
                        //     relaxed_dist = 150.0;
                        //     node_mass = 500.0;
                        // }

                        let restoring_force = -spring_char * (dist - relaxed_dist);
                        let acc = restoring_force / node_mass;

                        // println!(
                        //     "> {} @ {:?} <<< {:?} | {:?} | {:?} >>> {} @ {:?}",
                        //     node, node_pos, dist, restoring_force, acc, other, adj_pos
                        // );

                        let acc_vec = acc * diff.normalized() + diff.normalized();

                        let node_acc = accelerations.get_mut(&node).unwrap();
                        *node_acc = *node_acc - acc_vec;

                        let other_acc = accelerations.get_mut(&other).unwrap();
                        *other_acc = *other_acc + acc_vec;
                    }
                }

                // for adj in self.graph.get_adj(&node) {
                //     let adj_pos = self.graphic.get(&adj).unwrap();

                //     let diff = adj_pos.to_vec2() - node_pos.to_vec2();
                //     let dist = (diff).length();

                //     let spring_char = 0.7;
                //     let relaxed_dist = 100.0;
                //     let node_mass = 50.0;

                //     let restoring_force = -spring_char * (dist - relaxed_dist);
                //     let acc = restoring_force / node_mass;

                //     let acc_vec = acc * diff.normalized() + diff.normalized();

                //     let node_acc = accelerations.get_mut(&node).unwrap();
                //     *node_acc = *node_acc - acc_vec;

                //     let adj_acc = accelerations.get_mut(&adj).unwrap();
                //     *adj_acc = *adj_acc + acc_vec;
                // }
            }

            for node in self.graph.get_all() {
                let node_pos = self.graphic.get_mut(&node).unwrap();
                let node_vel = velocities.get_mut(&node).unwrap();
                let node_acc = accelerations.get_mut(&node).unwrap();

                *node_vel += *node_acc;
                *node_vel -= node_vel.normalized();
                *node_pos += *node_vel;

                // println!(
                //     "> {} @ {:?} v {:?} a {:?}",
                //     node, node_pos, node_vel, node_acc
                // );
                *node_acc = Vec2 { x: 0.0, y: 0.0 };
            }
        }
    }

    //---------------------------------------------------------------------------//
}

//---------------------------------------------------------------------------//

impl Default for Project {
    fn default() -> Self {
        Self {
            path: None,
            view: Default::default(),
            graph: WeightedGraph::new(),
            text: Default::default(),
            graphic: Default::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PanZoom {
    pub transform: TSTransform,
    pub drag_value: f32,
}

///////////////////////////////////////////////////////////////////////////////
