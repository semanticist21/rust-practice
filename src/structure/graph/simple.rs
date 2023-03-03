use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
    time::Instant,
};

type Key = String;

// key
#[derive(Clone)]
struct Vertex {
    label: Key,
    edges: HashMap<String, Rc<RefCell<Vertex>>>,
}

impl Vertex {
    pub fn new(value: String) -> Rc<RefCell<Self>> {
        let new_vertex = Vertex {
            label: value,
            edges: HashMap::new(),
        };

        Rc::new(RefCell::new(new_vertex))
    }
}

#[derive(Clone)]
struct Graph {
    current: Rc<RefCell<Vertex>>,
}

impl Graph {
    pub fn new(genesis: Rc<RefCell<Vertex>>) -> Self {
        Self { current: genesis }
    }

    pub fn insert_neighbor(&mut self, neighbor: Rc<RefCell<Vertex>>) {
        let borrowed = RefCell::borrow(&self.current);
        let neighbor_borrowed = RefCell::borrow(&neighbor);

        if borrowed.edges.contains_key(&neighbor_borrowed.label) {
            return;
        }

        drop(borrowed);

        let neighbor_key = &neighbor_borrowed.label;

        let mut mut_borrowed = RefCell::borrow_mut(&self.current);
        mut_borrowed
            .edges
            .insert(neighbor_key.clone(), Rc::clone(&neighbor));

        drop(neighbor_borrowed);
        drop(mut_borrowed);

        let my_self = Rc::clone(&self.current);
        self.current = Rc::clone(&neighbor);
        self.insert_neighbor(my_self);
    }

    pub fn search_dfs(&mut self, result: &mut Vec<String>) {
        // insert self label
        let borrowed = RefCell::borrow(&self.current);
        let key = borrowed.label.clone();

        if result.contains(&key) {
            return;
        }

        result.push(key);
        drop(borrowed);

        // recursion
        let current_clone = Rc::clone(&self.current);
        let borrowed = RefCell::borrow(&current_clone);

        for (_, value) in borrowed.edges.iter() {
            self.current = Rc::clone(value);
            self.search_dfs(result);
        }
    }

    pub fn search_bfs(&mut self, result: &mut Vec<String>) {
        // insert inital key
        let borrowed = self.current.borrow();
        result.push(borrowed.label.clone());

        let start_node = Rc::clone(&self.current);
        let mut queue = VecDeque::new();

        queue.push_front(start_node);

        while !queue.is_empty() {
            let pop = queue.pop_back().unwrap();

            let pop_borrow = pop.borrow();
            for (key, value) in &pop_borrow.edges {
                if !result.contains(key) {
                    result.push(key.clone());
                    queue.push_back(Rc::clone(value))
                }

                continue;
            }
        }
    }
}

#[test]
fn test_graph() {
    let vertex1 = Vertex::new("Lux".to_string());
    let mut graph = Graph::new(vertex1);

    for i in 0..1000 {
        let str = format!("{}{}", "lux", i);
        let vertex1 = Vertex::new(str);
        graph.insert_neighbor(vertex1);
    }

    let vertex2 = Vertex::new("Tom".to_string());
    let vertex3 = Vertex::new("Jenkins".to_string());
    let vertex5 = Vertex::new("Jamie1".to_string());
    let vertex6 = Vertex::new("Jamie2".to_string());
    let vertex7 = Vertex::new("Jamie3".to_string());
    let vertex8 = Vertex::new("Jamie4".to_string());
    let vertex9 = Vertex::new("Jamie5".to_string());
    let vertex10 = Vertex::new("Jamie6".to_string());
    let vertex11 = Vertex::new("Jamie7".to_string());
    let vertex12 = Vertex::new("Jamie8".to_string());
    let vertex13 = Vertex::new("Jamie9".to_string());
    let vertex14 = Vertex::new("Jamie10".to_string());
    let vertex4 = Vertex::new("Jamie11".to_string());

    graph.insert_neighbor(vertex2);
    graph.insert_neighbor(vertex3);
    graph.insert_neighbor(vertex4);
    graph.insert_neighbor(vertex5);
    graph.insert_neighbor(vertex6);
    graph.insert_neighbor(vertex7);
    graph.insert_neighbor(vertex8);
    graph.insert_neighbor(vertex9);
    graph.insert_neighbor(vertex10);
    graph.insert_neighbor(vertex11);
    graph.insert_neighbor(vertex12);
    graph.insert_neighbor(vertex13);
    graph.insert_neighbor(vertex14);

    let performance_start = Instant::now();

    let mut vec = vec![];
    graph.search_dfs(&mut vec);
    println!("{:?}", vec);

    let performance_end = Instant::now();
    println!("{:?}", performance_start - performance_end);

    let performance_start = Instant::now();

    let mut vec = vec![];
    graph.search_bfs(&mut vec);
    println!("{:?}", vec);

    let performance_end = Instant::now();
    println!("{:?}", performance_start - performance_end);
}
