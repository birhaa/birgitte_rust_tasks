use crate::Node;

pub fn distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64{
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

pub fn distance_total(nodes: &Vec<Node>) -> f64{
    return nodes
    .into_iter()
    .enumerate()
    .fold(0.0,  |s, (i, node)| {
        if i == nodes.len() -1 {
            let start_node = &nodes[0];
            return  s + distance(node.x, start_node.x, node.y, start_node.y);
        }else{
            let next = &nodes[i+1];
            s + distance(next.x,node.x, next.y, node.y)
        }
    });
}



mod tests {
    use crate::distance;
    use crate::Node;
    use crate::distance_total;

    #[test]
    fn test_distance(){
       assert_eq!(format!("{:.5}", distance(1.0, 4.0, 2.0, 5.0)), "4.24264");
    }

    #[test]
    fn test_distance_total(){
        let nodes = {
            let mut nodes : Vec<Node> = Vec::new();
            nodes.push(Node{
                id: 0,
                x: 4.0,
                y: 5.0
            });
            nodes.push(Node{
                id: 1,
                x: 1.0,
                y: 2.0
            });
            nodes.push(Node{
                id: 3,
                x: 6.0,
                y: 7.0
            });
            nodes
        };
        assert_eq!(format!("{:.5}", distance_total(&nodes)), "14.14214");
     }

}