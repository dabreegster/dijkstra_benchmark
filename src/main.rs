use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::time::Instant;

use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};

use self::priority_queue::PriorityQueueItem;

mod priority_queue;

#[derive(Serialize, Deserialize)]
struct Graph {
    // Index is NodeID
    edges_per_node: Vec<Vec<Edge>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct NodeID(u32);
// Seconds
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Cost(u16);

#[derive(Serialize, Deserialize)]
struct Edge {
    to: NodeID,
    cost: Cost,
}

fn main() {
    //convert_input();
    route();
}

fn convert_input() {
    println!("Read file");
    let contents = std::fs::read_to_string("walk_network_full.json").unwrap();
    println!("Parse");
    let input: HashMap<String, Vec<[usize; 5]>> = serde_json::from_str(&contents).unwrap();

    println!("Converting into graph");
    let mut graph = Graph {
        edges_per_node: std::iter::repeat_with(Vec::new).take(9739277).collect(),
    };
    for (from, input_edges) in input {
        let mut edges = Vec::new();
        for array in input_edges {
            edges.push(Edge {
                to: NodeID(array[1] as u32),
                cost: Cost(array[0] as u16),
            });
        }

        let from: usize = from.parse().unwrap();
        graph.edges_per_node[from] = edges;
    }

    println!("Saving the graph");
    let file = BufWriter::new(File::create("graph.bin").unwrap());
    bincode::serialize_into(file, &graph).unwrap();
}

fn route() {
    println!("Loading graph");
    let now = Instant::now();
    let file = BufReader::new(File::open("graph.bin").unwrap());
    let graph: Graph = bincode::deserialize_from(file).unwrap();
    println!("Loading took {:?}", now.elapsed());

    /*let results = floodfill(&graph, NodeID(42));
    println!("{:?}", results);
    println!("Reached {} nodes", results.len());*/

    // Benchmark mode
    let mut rng = WyRand::new();
    let now = Instant::now();
    for _ in 0..1000 {
        let start = NodeID(rng.generate_range(0..graph.edges_per_node.len() as u32));
        let results = floodfill(&graph, start);
        //println!("Reached {} nodes from {:?}", results.len(), start);
    }
    println!("Calculating routes took {:?}", now.elapsed());
}

fn floodfill(graph: &Graph, start: NodeID) -> HashMap<NodeID, Cost> {
    let time_limit = Cost(3600);

    let mut queue: BinaryHeap<PriorityQueueItem<Cost, NodeID>> = BinaryHeap::new();
    queue.push(PriorityQueueItem {
        cost: Cost(0),
        value: start,
    });

    let mut cost_per_node = HashMap::new();

    while let Some(current) = queue.pop() {
        if cost_per_node.contains_key(&current.value) {
            continue;
        }
        if current.cost > time_limit {
            continue;
        }
        cost_per_node.insert(current.value, current.cost);

        for edge in &graph.edges_per_node[current.value.0 as usize] {
            queue.push(PriorityQueueItem {
                cost: Cost(current.cost.0 + edge.cost.0),
                value: edge.to,
            });
        }
    }

    cost_per_node
}
