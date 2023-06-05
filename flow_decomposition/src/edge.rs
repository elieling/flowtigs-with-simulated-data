#[derive(Clone)]


pub struct Edge {
    pub id: i64,
    pub start_node: usize,
    pub end_node: usize,
    pub weight: i64,
    pub string: String,
}