// Rc<T> - Reference Counted Smart Pointer
// ========================================
// Rc<T> enables multiple owners of the same data.
// Only for single-threaded scenarios.

use std::rc::Rc;

fn main() {
    // Basic Rc usage
    let data = Rc::new(String::from("Hello, Rc!"));
    
    let data1 = Rc::clone(&data); // Increment reference count
    let data2 = Rc::clone(&data); // Increment reference count
    
    println!("Reference count: {}", Rc::strong_count(&data));
    println!("data: {}", data);
    println!("data1: {}", data1);
    println!("data2: {}", data2);
    
    // Rc with data structures
    rc_with_lists();
    
    // Rc cycles and memory leaks
    // rc_cycles(); // Commented out to avoid memory leak
    
    // Weak references
    weak_references_demo();
    
    drop(data1);
    println!("After dropping data1, count: {}", Rc::strong_count(&data));
    
    drop(data2);
    println!("After dropping data2, count: {}", Rc::strong_count(&data));
}

// Using Rc with linked lists for multiple ownership
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn rc_with_lists() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("Lists: a={:?}, b={:?}, c={:?}", a, b, c);
    }
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Potential memory leak with Rc cycles
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn rc_cycles() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    // This creates a cycle! branch -> leaf -> branch
    // Without weak references, this would leak memory
}

// Weak references prevent cycles
fn weak_references_demo() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Rc with trait objects
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn name(&self) -> &str {
        "Rectangle"
    }
}

fn rc_with_trait_objects() {
    let shapes: Vec<Rc<dyn Shape>> = vec![
        Rc::new(Circle { radius: 5.0 }),
        Rc::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    // Multiple owners of the same shapes
    let shapes1 = shapes.clone(); // Clone the Vec, Rc counts increase
    let shapes2 = shapes.clone();
    
    for shape in &shapes {
        println!("{}: {}", shape.name(), shape.area());
        println!("Strong count: {}", Rc::strong_count(shape));
    }
}

// Custom data structure with Rc
struct Graph {
    nodes: Vec<Rc<GraphNode>>,
}

struct GraphNode {
    id: usize,
    data: String,
    edges: RefCell<Vec<Weak<GraphNode>>>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
    
    fn add_node(&mut self, data: String) -> Rc<GraphNode> {
        let id = self.nodes.len();
        let node = Rc::new(GraphNode {
            id,
            data,
            edges: RefCell::new(Vec::new()),
        });
        self.nodes.push(Rc::clone(&node));
        node
    }
    
    fn add_edge(&self, from: &Rc<GraphNode>, to: &Rc<GraphNode>) {
        from.edges.borrow_mut().push(Rc::downgrade(to));
    }
}

fn graph_example() {
    let mut graph = Graph::new();
    
    let node1 = graph.add_node("Node 1".to_string());
    let node2 = graph.add_node("Node 2".to_string());
    let node3 = graph.add_node("Node 3".to_string());
    
    graph.add_edge(&node1, &node2);
    graph.add_edge(&node2, &node3);
    graph.add_edge(&node3, &node1);
    
    println!("Graph with {} nodes created", graph.nodes.len());
    
    // Print connections
    for node in &graph.nodes {
        print!("{} -> ", node.data);
        for edge in node.edges.borrow().iter() {
            if let Some(target) = edge.upgrade() {
                print!("{} ", target.data);
            }
        }
        println!();
    }
}

// Performance considerations
fn performance_notes() {
    println!("=== Rc Performance Notes ===");
    println!("1. Reference counting has runtime cost");
    println!("2. Clone is cheap (just increments counter)");
    println!("3. Not thread-safe (use Arc for multi-threading)");
    println!("4. Can create memory leaks with cycles");
    println!("5. Use Weak references to break cycles");
}

// Common patterns with Rc
fn common_patterns() {
    // Pattern 1: Shared immutable data
    let config = Rc::new(AppConfig {
        database_url: "postgres://localhost".to_string(),
        api_key: "secret".to_string(),
    });
    
    let service1 = Service::new(Rc::clone(&config));
    let service2 = Service::new(Rc::clone(&config));
    
    // Pattern 2: Tree with parent references
    // (Already shown in weak_references_demo)
    
    // Pattern 3: Observer pattern
    let subject = Rc::new(RefCell::new(Subject::new()));
    let observer1 = Observer::new(Rc::downgrade(&subject));
    let observer2 = Observer::new(Rc::downgrade(&subject));
    
    subject.borrow_mut().add_observer(observer1);
    subject.borrow_mut().add_observer(observer2);
}

struct AppConfig {
    database_url: String,
    api_key: String,
}

struct Service {
    config: Rc<AppConfig>,
}

impl Service {
    fn new(config: Rc<AppConfig>) -> Self {
        Service { config }
    }
}

struct Subject {
    observers: Vec<Observer>,
    state: i32,
}

struct Observer {
    subject: Weak<RefCell<Subject>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
            state: 0,
        }
    }
    
    fn add_observer(&mut self, observer: Observer) {
        self.observers.push(observer);
    }
}

impl Observer {
    fn new(subject: Weak<RefCell<Subject>>) -> Self {
        Observer { subject }
    }
}
