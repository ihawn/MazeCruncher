extern crate minifb;
use minifb::Window;
use std::{cmp::Ordering, collections::BinaryHeap};


pub fn dijkstra(mut window: Window, params: crate::solve::MazeParams) -> Window
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
    let mut maze = graph_init(&mtx, size);
    let mut node_heap: BinaryHeap<DNode> = BinaryHeap::new();
    maze[start_x][start_y].dist = 0;

    node_heap.push(maze[start_x][start_y]);
    
    let end_node = maze[end_x][end_y];
    let mut max = 0; //For the counter


    let mut counter: u128 = 0;
    while !node_heap.is_empty()
    {
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);


        //Get lowest in heap
        let mut current = node_heap.pop().unwrap();
        mtx[current.x][current.y] = 2;
        
        //Get adjacent cells
        let children = get_children(&maze, current);

        //Loop through adjacent cells and update parents and distances
        for mut c in children
        {
            if mtx[c.x][c.y] != 2
            {
                let temp_dist = current.dist + 1;

                if temp_dist < c.dist
                {
                    maze[c.x][c.y].dist = temp_dist;
                    maze[c.x][c.y].parent_x = current.x;
                    maze[c.x][c.y].parent_y = current.y;
                    c.dist = temp_dist;
                    c.parent_x = current.x;
                    c.parent_y = current.y;

                    node_heap.push(c);
                }
            }
        }

        //Stopping condition
        if current == end_node 
        {   
            //retreive path
            while current.x != start_x || current.y != start_y
            {
                mtx[current.x][current.y] = 1;
                current = maze[current.parent_x][current.parent_y];

                window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);


                counter += 1;
            }
            mtx[start_x][start_y] = 1;
            break 
        }

        counter += 1;
        max = crate::utils::update_counter(max, current.x, current.y, size, "Dijkstra");
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_dijkstra.png".to_string());
    }

    window
}

//Get adjacent nodes
fn get_children(maze: &[Vec<DNode>], node: DNode) -> Vec<DNode>
{
    let x = node.x;
    let y = node.y;
    let mut children: Vec<DNode> = vec!();

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

//Initialize graph from the maze matrix
fn graph_init(mtx: &[Vec<u8>], size: usize) -> Vec<Vec<DNode>>
{
    let mut maze_graph: Vec<Vec<DNode>> = vec!();
    for i in 0..size
    {
        let mut maze_row: Vec<DNode> = vec!();
        for j in 0..size
        {
            let node = DNode{
                x: i,
                y: j,
                parent_x: usize::MAX,
                parent_y: usize::MAX,
                is_wall: mtx[i][j] == u8::MAX,
                dist: usize::MAX
            };
            maze_row.push(node);
        }
        maze_graph.push(maze_row);
    }
    maze_graph
}

//Struct to store maze node
#[derive(Copy, Clone)]
struct DNode
{
    x: usize,
    y: usize,
    parent_x: usize,
    parent_y: usize,
    is_wall: bool,
    dist: usize,
}

impl PartialEq for DNode
{
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl PartialOrd for DNode
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Eq for DNode {}

impl Ord for DNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.dist.cmp(&other.dist).reverse()
    }
}