extern crate minifb;
use minifb::Window;
use std::{cmp::Ordering, collections::BinaryHeap};


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
    let mut current: Node = maze[0].clone();
    let end_node = maze[maze.len()-1].clone();

    let mut max = 0; //For the counter

    let mut counter: u128 = 0;
    while !open_heap.is_empty()
    {
        
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);

        current = open_heap.pop().unwrap(); //Get node with lowest f
        current.node.closed = true; 
        maze[current.clone().node.index].node.open = false;

        
        let mut o = current.node;
        let mut p = maze[current.node.parent_index].node;
        mtx = crate::utils::fill_between_nodes(mtx, o.x, o.y, p.x, p.y, 2);


        // println!("\nCurrent: {},{}", current.node.x, current.node.y);
        // println!("Parent: {},{}", last.node.x, last.node.y);
        // for u in 0..last.connected.len()
        // {
        //     println!("branch: {}, {}",last.connected[u].x, last.connected[u].y);
        //     println!("weight: {}",last.edge_weights[u]);
        // }


        mtx[current.node.x][current.node.y] = 2;
        maze[current.node.index] = current.clone();

        //Stopping condition
        if current.node == end_node.node
        {   
            //retreive path
            while current.node.x != start_x || current.node.y != start_y
            {
                current = maze[current.node.parent_index].clone();

                o = current.node;
                mtx = crate::utils::fill_between_nodes(mtx, o.x, o.y, p.x, p.y, 1);
                p = current.node;

                window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);

                counter += 1;
            }
            mtx[start_x][start_y] = 1;
            break 
        }
        //println!("Current: {},{}", current.node.x, current.node.y);

        //Get children and update lists
        //let mut children = get_children(&maze, current); 

        'inner: for i in 0..current.connected.len()
        {
            if  maze[current.connected[i].index].node.open || current.connected[i].closed { continue 'inner; }
            
            current.connected[i].g = current.node.g + current.edge_weights[i] as usize;
            current.connected[i].f = current.connected[i].g + current.connected[i].h;

            maze[current.connected[i].index].node.open = true;
            maze[current.connected[i].index].node.parent_index = current.node.index;

            
            // println!("\nCurrent: {},{}", p.x, p.y);
            // println!("branch: {}, {}",o.x, o.y);


            
            open_heap.push(maze[current.clone().connected[i].index].clone());
        }


        counter += 1;
        max = crate::utils::update_counter(max, current.clone().node.x, current.clone().node.y, size, "A*");
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_astar.png".to_string());
    }
    
    window
}

//Initialize graph from the maze matrix
fn graph_init(mtx: &[Vec<u8>], size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Vec<Node>//Vec<Vec<MazeNode>>
{
    let mut maze_graph: Vec<Vec<MazeNode>> = vec!();

    //Vectors for storing the maze matrix via Compressed Sparse Row format
    let mut nodes: Vec<Node> = vec!();
    let mut node_x: Vec<usize> = vec!();
    let mut node_y: Vec<usize> = vec!();
    
    let mut node_count: usize = 0;

    //Add first node
    nodes.push(Node{ node: MazeNode{
            x: start_x,
            y: start_y,
            parent_x: usize::MAX,
            parent_y: usize::MAX,
            is_wall: mtx[start_x][start_y] == u8::MAX,
            open: false,
            closed: false,
            h: crate::utils::euclidean(start_x, end_x, start_y, end_y) as usize,
            g: usize::MAX,
            f: 0,
            index: node_count,
            parent_index: 0
        }, connected: vec!(), edge_weights: vec!() });

    let mut matrix: Vec<Vec<u8>> = mtx.to_vec();

    //Build the nodes
    for i in 0..size
    {
        let mut maze_row: Vec<MazeNode> = vec!();
        for j in 0..size
        {
            
            //If not a wall and should be a node
            if mtx[i][j] != u8::MAX && 
            (((mtx[i+1][j] == u8::MAX && mtx[i-1][j] != u8::MAX) || (mtx[i+1][j] != u8::MAX && mtx[i-1][j] == u8::MAX)) ||
            ((mtx[i][j+1] == u8::MAX && mtx[i][j-1] != u8::MAX) || (mtx[i][j+1] != u8::MAX && mtx[i][j-1] == u8::MAX)))
            {
                node_count += 1;
                
                nodes.push(Node{ node: MazeNode{
                    x: i,
                    y: j,
                    parent_x: usize::MAX,
                    parent_y: usize::MAX,
                    is_wall: mtx[i][j] == u8::MAX,
                    open: false,
                    closed: false,
                    h: crate::utils::euclidean(i, end_x, j, end_y) as usize,
                    g: usize::MAX,
                    f: 0,
                    index: node_count,
                    parent_index: 0
                }, connected: vec!(), edge_weights: vec!(), });
                node_x.push(i);
                node_y.push(j);

                matrix[i][j] = 1;
            }


            // maze_row.push(MazeNode{
            //     x: i,
            //     y: j,
            //     parent_x: usize::MAX,
            //     parent_y: usize::MAX,
            //     is_wall: mtx[i][j] == u8::MAX,
            //     open: false,
            //     closed: false,
            //     h: crate::utils::euclidean(i, end_x, j, end_y) as usize,
            //     g: usize::MAX,
            //     f: 0
            // });
        }
        maze_graph.push(maze_row);
    }

    //Add end node
    nodes.push(Node{ node: MazeNode{
        x: end_x,
        y: end_y,
        parent_x: usize::MAX,
        parent_y: usize::MAX,
        is_wall: mtx[end_x][end_y] == u8::MAX,
        open: false,
        closed: false,
        h: 0,
        g: usize::MAX,
        f: 0,
        index: node_count,
        parent_index: 0
    }, connected: vec!(), edge_weights: vec!() });


    //Connect the nodes
    let l = nodes.len();
    for i in 0..l-1
    {
        let n1 = nodes[i].node;
        //connect to node below if no wall is between
        if nodes[i].node.x == nodes[i+1].node.x &&
        !wall_between_nodes_vert(mtx, &nodes[i+1], &nodes[i])
        {
            let n2 = nodes[i+1].node;
            let diff = (nodes[i].node.y as i32 - nodes[i+1].node.y as i32).abs() as u16;
            nodes[i].connected.push(n2);
            nodes[i+1].connected.push(n1);
            nodes[i].edge_weights.push(diff);
            nodes[i+1].edge_weights.push(diff);
        }

        //connect to node laterally 
        for n in 0..i
        {
            if nodes[i].node.y == nodes[n].node.y && 
            !wall_between_nodes_horz(mtx, &nodes[n], &nodes[i]) &&
            nodes[i].node != nodes[n].node
            {
                let n2 = nodes[n].node;
                let diff = (nodes[i].node.x as i32 - nodes[n].node.x as i32).abs() as u16;
                nodes[i].connected.push(n2);
                nodes[n].connected.push(n1);
                nodes[i].edge_weights.push(diff);
                nodes[n].edge_weights.push(diff);
            }
        }
        
    }

    matrix[nodes[200].node.x][nodes[200].node.y] = 2;
    println!("root: {}, {}",nodes[200].node.x, nodes[200].node.y);
    for u in 0..nodes[200].connected.len()
    {
        println!("branch: {}, {}",nodes[200].connected[u].x, nodes[200].connected[u].y);
        println!("weight: {}",nodes[200].edge_weights[u]);
    }
    crate::toimage::mtx_to_img(&matrix, size, "Node test.png".to_string());

    //maze_graph
    nodes
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

//n1 is left of n2
fn wall_between_nodes_horz(mtx: &[Vec<u8>], n1: &Node, n2: &Node) -> bool
{
    let y = n1.node.y;
    for i in n1.node.x..n2.node.x
    {
        if mtx[i][y] == u8::MAX { return true; }
    }
    false
}

fn get_children(maze: &[Vec<MazeNode>], node: MazeNode) -> Vec<MazeNode>
{
    let x = node.x;
    let y = node.y;
    let mut children: Vec<MazeNode> = vec!();

    if x < maze[0].len()-1 && !maze[x+1][y].is_wall { children.push(maze[x+1][y]); }
    if x > 0 && !maze[x-1][y].is_wall { children.push(maze[x-1][y]); }
    if y < maze[0].len()-1 && !maze[x][y+1].is_wall { children.push(maze[x][y+1]); }
    if y > 0 && !maze[x][y-1].is_wall { children.push(maze[x][y-1]); }

    for i in 0..children.len()
    {
        children[i].parent_x = node.x;
        children[i].parent_y = node.y;
    }

    children
}

#[derive(Clone)]
struct Node
{
    node: MazeNode,
    connected: Vec<MazeNode>,
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

//Struct to store maze node
#[derive(Copy, Clone)]
struct MazeNode
{
    x: usize,
    y: usize,
    parent_x: usize,
    parent_y: usize,
    is_wall: bool,
    open: bool,
    closed: bool,
    h: usize,
    g: usize,
    f: usize,
    index: usize,
    parent_index: usize
}

impl PartialEq for MazeNode
{
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl PartialOrd for MazeNode
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Eq for MazeNode {}

impl Ord for MazeNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.f.cmp(&other.f).reverse()
    }
}