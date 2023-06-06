#[derive(Clone)]


pub struct Edge {
    pub id: usize,
    pub start_node: usize,
    pub end_node: usize,
    pub weight: i64,
    pub string: String,
}