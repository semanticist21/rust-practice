use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Vertex {
    pub label: String,
    pub edges_cost: HashMap<String, u32>,
    pub edges_city: HashMap<String, Rc<RefCell<Vertex>>>,
}

impl Vertex {
    pub fn new(label: String) -> Self {
        Vertex {
            label,
            edges_cost: HashMap::new(),
            edges_city: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, city: Rc<RefCell<Vertex>>, price: u32) {
        let borrowed = city.borrow();
        let key = borrowed.label.clone();
        let key_cloned = key.clone();

        self.edges_cost.insert(key, price);

        drop(borrowed);
        self.edges_city.insert(key_cloned, city);
    }
}

pub fn find_shortest_dijkstra(departure: Rc<RefCell<Vertex>>, destination: &str) -> Vec<String> {
    let mut visited_cities = HashMap::<String, bool>::new();
    let mut unvisited_cities = HashMap::<String, Rc<RefCell<Vertex>>>::new();

    let mut cheapest_cost_to = HashMap::<String, u32>::new();
    let mut cheapest_last_stopover = HashMap::<String, String>::new();

    let current = Rc::clone(&departure);
    let departure = current.borrow();
    let departure_name = departure.label.clone();

    unvisited_cities.insert(departure_name.clone(), Rc::clone(&current));
    cheapest_cost_to.insert(departure_name.clone(), 0);
    drop(departure);

    while unvisited_cities.len() != 0 {
        let first = Rc::clone(unvisited_cities.iter().next().unwrap().1);
        let borrowed_current = first.borrow();

        visited_cities.insert(borrowed_current.label.clone(), true);

        if unvisited_cities.contains_key(&borrowed_current.label) {
            unvisited_cities.remove(&borrowed_current.label);
        }

        for (key, value) in borrowed_current.edges_city.iter() {
            if !unvisited_cities.contains_key(key) {
                unvisited_cities.insert(key.clone(), Rc::clone(&value));
            }

            let price_from_current = borrowed_current.edges_cost[key];

            if !cheapest_last_stopover.contains_key(key) {
                let base_cost = cheapest_cost_to[&borrowed_current.label];

                let cost = price_from_current + base_cost;
                cheapest_cost_to.insert(key.clone(), cost);

                cheapest_last_stopover.insert(key.clone(), borrowed_current.label.clone());
            } else {
                let cheapest_price = cheapest_cost_to[key];
                let price_until_current = cheapest_cost_to[&borrowed_current.label];

                if price_until_current + price_from_current < cheapest_price {
                    cheapest_cost_to.insert(key.clone(), price_until_current + price_from_current);
                    cheapest_last_stopover.insert(key.clone(), borrowed_current.label.clone());
                }
            }
        }
    }

    let mut last_stopover = &cheapest_last_stopover[&destination.to_string()];
    let mut route: Vec<String> = vec![];

    loop {
        if last_stopover.eq(&departure_name) {
            route.push(departure_name);
            break;
        }

        route.push(last_stopover.clone());
        last_stopover = &cheapest_last_stopover[last_stopover];
    }

    // println!("{:?}", cheapest_cost_to);
    // println!("{:?}", cheapest_last_stopover);

    route.reverse();
    route
}

#[test]
fn test_find_path() {
    const ATLANTA: &str = "Atlanta";
    const BOSTON: &str = "Boston";
    const CHICAGO: &str = "Chicago";
    const DENVER: &str = "Denver";
    const EL_PASO: &str = "ElPaso";

    let atlanta = Rc::new(RefCell::new(Vertex::new(ATLANTA.to_string())));
    let boston = Rc::new(RefCell::new(Vertex::new(BOSTON.to_string())));
    let chicago = Rc::new(RefCell::new(Vertex::new(CHICAGO.to_string())));
    let denver = Rc::new(RefCell::new(Vertex::new(DENVER.to_string())));
    let el_paso = Rc::new(RefCell::new(Vertex::new(EL_PASO.to_string())));

    atlanta.borrow_mut().add_route(Rc::clone(&boston), 100);
    atlanta.borrow_mut().add_route(Rc::clone(&denver), 160);
    boston.borrow_mut().add_route(Rc::clone(&chicago), 120);
    boston.borrow_mut().add_route(Rc::clone(&denver), 180);
    chicago.borrow_mut().add_route(Rc::clone(&el_paso), 80);
    denver.borrow_mut().add_route(Rc::clone(&chicago), 40);
    denver.borrow_mut().add_route(Rc::clone(&el_paso), 140);

    let result = find_shortest_dijkstra(Rc::clone(&atlanta), EL_PASO);

    println!("{:?}", result);
}
