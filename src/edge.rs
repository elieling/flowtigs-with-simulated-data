

pub type NodeId = usize;
pub type EdgeId = usize;
pub type Weight = i64;


#[derive(Clone)]
// Struct for an edge of the dbg
pub struct Edge {
    pub id: EdgeId,
    pub start_node: NodeId,
    pub end_node: NodeId,
    pub weight: Weight,
    // pub string: String,
}


// Function to build an Edge
pub fn build_edge(id: EdgeId, start_node: NodeId, end_node: NodeId, weight: Weight) -> Edge {
    Edge {
        id,
        start_node,
        end_node,
        weight,
        // string,
    }
}

