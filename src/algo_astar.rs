extern crate minifb;
use minifb::Window;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem::size_of_val;



pub fn astar(mut window: Window, params: crate::solve::MazeParams) -> Window
{
    let mut mtx = params.mtx;
    let size = params.size;
    let start_x = params.start_x;
    let start_y = params.start_y;
    let end_x = params.end_x;
    let end_y = params.end_y;
    let save_maze = params.save_maze;
    let show_animation = params.show_animation;
    let anim_speed_mult = params.anim_speed_mult;
    let buff_size = params.buff_size;

    //Algo init
    let mut maze = graph_init(&mtx, size, start_x, start_y, end_x, end_y);

    let mut open_heap: BinaryHeap<Node>/*BinaryHeap<MazeNode>*/ = BinaryHeap::new();
    maze[0].node.open = true;
    open_heap.push(maze[0].clone());

    //let mut current = open_list[0];
    let mut current: Node;

    let mut max = 0; //For the counter

    let mut counter: u128 = 0;
    while !open_heap.is_empty()
    {      
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);

        current = open_heap.pop().unwrap(); //Get node with lowest f
        current.node.closed = true; 
        current.node.open = false;
        maze[current.node.index].node.open = false;
        maze[current.node.index].node.closed = true;

        
        let mut o = current.node;
        let mut p = maze[current.node.parent_index].node;
        mtx = crate::utils::fill_between_nodes(mtx, o.x, o.y, p.x, p.y, 2);

        //Stopping condition
        if current.node.x == end_x && current.node.y == end_y
        {             
            //retreive path
            while current.node.x != start_x || current.node.y != start_y
            {
                o = current.node;
                mtx = crate::utils::fill_between_nodes(mtx, o.x, o.y, p.x, p.y, 1);
                p = current.node;

                window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);
                current = maze[current.node.parent_index].clone();

                counter += 1;
            }
            
            mtx = crate::utils::fill_between_nodes(mtx, o.x, o.y, maze[o.parent_index].node.x, maze[o.parent_index].node.y, 1);
            mtx[start_x][start_y] = 1;
            mtx[end_x][end_y] = 1;

            
            println!("Solved");
            break 
        }


        'inner: for i in 0..current.connected.len()
        {
            if maze[current.connected[i]].node.open || maze[current.connected[i]].node.closed/*current.connected[i].closed*/ { continue 'inner; }
            
            maze[current.connected[i]].node.g = maze[current.node.index].node.g + maze[current.node.index].edge_weights[i] as usize;
            maze[current.connected[i]].node.f = maze[current.connected[i]].node.g + crate::utils::euclidean(maze[current.connected[i]].node.x, end_x, maze[current.connected[i]].node.y, end_y);
            maze[current.connected[i]].node.open = true;
            maze[current.connected[i]].node.closed = false;          
            maze[current.connected[i]].node.parent_index = current.node.index;            
            open_heap.push(maze[current.connected[i]].clone());
        }



        counter += 1;
        max = crate::utils::update_counter(max, current.clone().node.x, current.clone().node.y, size, "A*");
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_astar.png".to_string());
    }
    
    window
}

//Initialize graph from the maze matrix
fn graph_init(mtx: &[Vec<u8>], size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Vec<Node>//Vec<Vec<MazeNode>>
{
    println!("Building graph...");

    //Vectors for storing the maze matrix via Compressed Sparse Row format
    let mut nodes: Vec<Node> = vec!();  
    let mut node_count: usize = 0;

    let mut cur_node= Node{ node: MazeNode{
        x: start_x,
        y: start_y,
        open: false,
        closed: false,
        g: usize::MAX,
        f: 0,
        index: node_count,
        parent_index: 0
    }, connected: vec!(), edge_weights: vec!() };

    //Add first node
    nodes.push(cur_node);

    //Build the nodes and attach
    let mut left_nodes: Vec<usize> = vec![usize::MAX; size]; //Container to store the last node to the left of current with a given y value
    for i in 1..size-1
    {
        for j in 1..size-1
        {         
            //Conditions to be a node: Either dead end, 3-way intersection, or 4-way intersection
            if mtx[i][j] != u8::MAX && 
            (((mtx[i+1][j] == u8::MAX && mtx[i-1][j] != u8::MAX) || (mtx[i+1][j] != u8::MAX && mtx[i-1][j] == u8::MAX)) ||
            ((mtx[i][j+1] == u8::MAX && mtx[i][j-1] != u8::MAX) || (mtx[i][j+1] != u8::MAX && mtx[i][j-1] == u8::MAX)) ||
            (mtx[i+1][j] != u8::MAX && mtx[i-1][j] != u8::MAX && mtx[i][j+1] != u8::MAX && mtx[i][j-1] != u8::MAX))
            {
                node_count += 1;
                cur_node = Node{ node: MazeNode{
                    x: i,
                    y: j, 
                    open: false,
                    closed: false,
                    g: usize::MAX,
                    f: 0,
                    index: node_count,
                    parent_index: 0
                }, connected: vec!(), edge_weights: vec!(), };
                
                nodes.push(cur_node);

                let res = attach_nodes(mtx, nodes, left_nodes, node_count);
                nodes = res.0; left_nodes = res.1;
            }
        }
    }

    //Add end node
    nodes.push(Node{ node: MazeNode{
        x: end_x,
        y: end_y,
        open: false,
        closed: false,
        g: usize::MAX,
        f: 0,
        index: node_count,
        parent_index: 0
    }, connected: vec!(), edge_weights: vec!() });

    node_count += 1;
    let res = attach_nodes(mtx, nodes, left_nodes, node_count);
    nodes = res.0;

    println!("Node count: {}", node_count);
    println!("Size of graph: {}", size_of_val(&*nodes));
    nodes
}

fn attach_nodes(mtx: &[Vec<u8>], mut nodes: Vec<Node>, mut left_nodes: Vec<usize>, node_count: usize) -> (Vec<Node>, Vec<usize>)
{
    let n1 = nodes[node_count-1].node;
    //connect to node below if no wall is between
    if nodes[node_count-1].node.x == nodes[node_count].node.x &&
    !wall_between_nodes_vert(mtx, &nodes[node_count], &nodes[node_count-1])
    {
        let n2 = nodes[node_count].node;
        let diff = (nodes[node_count-1].node.y as i32 - nodes[node_count].node.y as i32).abs() as u16;
        nodes[node_count-1].connected.push(n2.index);
        nodes[node_count].connected.push(n1.index);
        nodes[node_count-1].edge_weights.push(diff);
        nodes[node_count].edge_weights.push(diff);
    }

    //connect to node laterally 
    if left_nodes[nodes[node_count-1].node.y] != usize::MAX &&
    !wall_between_nodes_horz(mtx, &nodes[left_nodes[nodes[node_count-1].node.y]], &nodes[node_count-1])
    {
        let n2 = nodes[left_nodes[nodes[node_count-1].node.y]].node;
        let diff = (nodes[node_count-1].node.x as i32 - n2.x as i32).abs() as u16;

        nodes[node_count-1].connected.push(n2.index);
        nodes[n2.index].connected.push(n1.index);

        nodes[node_count-1].edge_weights.push(diff);
        nodes[n2.index].edge_weights.push(diff);
    }

    left_nodes[nodes[node_count-1].node.y] = nodes[node_count-1].node.index;  

    (nodes, left_nodes)
}

//n1 is above n2 and shares the same x
fn wall_between_nodes_vert(mtx: &[Vec<u8>], n1: &Node, n2: &Node) -> bool
{
    let x = n1.node.x;
    for i in n2.node.y..n1.node.y
    {
        if mtx[x][i] == u8::MAX { return true; }
    }
    false
}

//n1 is left of n2 and shares the same y
fn wall_between_nodes_horz(mtx: &[Vec<u8>], n1: &Node, n2: &Node) -> bool
{
    let y = n1.node.y;
    for i in n1.node.x..n2.node.x
    {
        if mtx[i][y] == u8::MAX { return true; }
    }
    false
}

//Struct to store maze node
#[derive(Copy, Clone)]
struct MazeNode
{
    x: usize,
    y: usize,
    open: bool,
    closed: bool,
    g: usize,
    f: usize,
    index: usize,
    parent_index: usize
}

#[derive(Clone)]
pub struct Node
{
    node: MazeNode,
    connected: Vec<usize>,
    edge_weights: Vec<u16>
}

impl PartialEq for Node
{
    fn eq(&self, other: &Self) -> bool { self.node.x == other.node.x && self.node.y == other.node.y }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.node.f.cmp(&other.node.f).reverse()
    }
}
