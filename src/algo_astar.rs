extern crate minifb;
use std::{cmp::Ordering, collections::BinaryHeap};


pub fn astar(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0, "A*");

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size, "A*");
    }

    //Algo init
    let mut maze = graph_init(&mtx, size, end_x, end_y);

    let mut open_heap: BinaryHeap<MazeNode> = BinaryHeap::new();
    maze[start_x][start_y].open = true;
    open_heap.push(maze[start_x][start_y]);

    //let mut current = open_list[0];
    let mut current: MazeNode = maze[start_x][start_y];
    let end_node = maze[end_x][end_y];

    let mut max = 0; //For the counter

    let mut counter: u128 = 0;
    while !open_heap.is_empty()
    {
        //update window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }

        current = open_heap.pop().unwrap(); //Get node with lowest f
        current.closed = true;
        
        maze[current.x][current.y].open = false;


        mtx[current.x][current.y] = 2;
        maze[current.x][current.y] = current;

        //Stopping condition
        if current == end_node 
        {   
            //retreive path
            while current.x != start_x || current.y != start_y
            {
                mtx[current.x][current.y] = 1;
                current = maze[current.parent_x][current.parent_y];

                //update window
                if show_animation && counter % anim_speed_mult as u128 == 0
                {
                    buffer = crate::utils::update_buffer(&mtx, size, buffer);
                    window
                    .update_with_buffer(&buffer, size, size)
                    .unwrap();
                }

                counter += 1;
            }
            mtx[start_x][start_y] = 1;
            break 
        }

        //Get children and update lists
        let mut children = get_children(&maze, current); 

        'inner: for i in 0..children.len()
        {
            if  maze[children[i].x][children[i].y].open || children[i].closed { continue 'inner; }
            
            children[i].g = current.g + 1;
            children[i].f = children[i].g + children[i].h;

            maze[children[i].x][children[i].y].open = true;
            open_heap.push(children[i]);
        }

        counter += 1;
        max = crate::utils::update_counter(max, current.x, current.y, size, "A*");
    }

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_astar.png".to_string());
    }
}

//Initialize graph from the maze matrix
fn graph_init(mtx: &[Vec<u8>], size: usize, end_x: usize, end_y: usize) -> Vec<Vec<MazeNode>>
{
    let mut maze_graph: Vec<Vec<MazeNode>> = vec!();
    for i in 0..size
    {
        let mut maze_row: Vec<MazeNode> = vec!();
        for j in 0..size
        {
            maze_row.push(MazeNode{
                x: i,
                y: j,
                parent_x: usize::MAX,
                parent_y: usize::MAX,
                is_wall: mtx[i][j] == u8::MAX,
                open: false,
                closed: false,
                h: manhatten(i, end_x, j, end_y) as usize,
                g: usize::MAX,
                f: 0
            });
        }
        maze_graph.push(maze_row);
    }

    maze_graph
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

pub fn manhatten(x: usize, end_x: usize, y: usize, end_y: usize) -> u32
{
    (end_x-x + end_y-y) as u32
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
    f: usize
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