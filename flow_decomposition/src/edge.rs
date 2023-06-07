

pub type NodeId = usize;
pub type EdgeId = usize;
pub type Weight = i64;


#[derive(Clone)]

pub struct Edge {
    pub id: EdgeId,
    pub start_node: NodeId,
    pub end_node: NodeId,
    pub weight: Weight,
    pub string: String,
}


pub fn build_edge(id: EdgeId, start_node: NodeId, end_node: NodeId, weight: Weight, string: String) -> Edge {
    Edge {
        id,
        start_node,
        end_node,
        weight,
        string,
    }
}

