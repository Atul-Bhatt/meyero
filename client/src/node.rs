pub enum NodeStatus {
    Active,
    Inactive,
    Busy,
}

pub struct Node {
   pub u_hash: String,
   pub ip_addr: String,
   pub port: i32,
   pub pub_key: String,
   pub pvt_key: String,
   pub peers: Vec<Node>,
   pub status: NodeStatus,
}
