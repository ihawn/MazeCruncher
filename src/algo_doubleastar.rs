extern crate minifb;
use minifb::Window;
use std::{cmp::Ordering, collections::BinaryHeap};


pub fn double_astar(mut window: Window, params: crate::solve::MazeParams) -> Window
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

    let mut open_heap1: BinaryHeap<DoubleMazeNode> = BinaryHeap::new();
    let mut open_heap2: BinaryHeap<DoubleMazeNode> = BinaryHeap::new();
    maze[start_x][start_y].open = true;
    maze[end_x][end_y].open = true;
    open_heap1.push(maze[start_x][start_y]);
    open_heap2.push(maze[end_x][end_y]);

    //let mut current = open_list[0];
    let mut current1: DoubleMazeNode;
    let mut current2: DoubleMazeNode = maze[end_x][end_y];

    let mut reached_end = false;
    let mut top_first = false;

    let mut max = 0; //For the counter

    let mut counter: u128 = 0;
    while !open_heap1.is_empty() && !open_heap2.is_empty()
    {
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);

    
        //Top left
        let mut res = astar_tendril(reached_end, mtx, maze, open_heap1, current2, 2, 3);
        mtx = res.0; maze = res.1; open_heap1 = res.2; current1 = res.3; current2 = res.4; reached_end = res.5; 

        //Bottom right

        if !reached_end
        {
            res = astar_tendril(reached_end, mtx, maze, open_heap2, current1, 3, 2);
            mtx = res.0; maze = res.1; open_heap2 = res.2; current2 = res.3; current1 = res.4; reached_end = res.5;
        }
        else { top_first = true; }


        //Stopping condition
        if reached_end
        {   
            //Ensure that parent backtracking is correct for imperfect mazes
            let node: DoubleMazeNode;
            if top_first { node = current2; }
            else { node = current1; }

            let children = get_children(&maze, node);
            for c in children
            {
                if c.allegiance == node.allegiance
                {
                    if top_first { current2 = maze[c.parent_x][c.parent_y]; }
                    else { current1 = maze[c.parent_x][c.parent_y]; }
                    break;
                }
            }


            //retreive path
            while (current1.x != start_x || current1.y != start_y) || (current2.x != end_x || current2.y != end_y)
            {
                mtx[current1.x][current1.y] = 1;
                if current1.parent_x != usize::MAX && current1.parent_y != usize::MAX { current1 = maze[current1.parent_x][current1.parent_y]; }

                mtx[current2.x][current2.y] = 1;
                if current2.parent_x != usize::MAX && current2.parent_y != usize::MAX { current2 = maze[current2.parent_x][current2.parent_y]; }

                window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);

                counter += 1;
            }
            mtx[start_x][start_y] = 1;
            break 
        }

        counter += 1;
        max = crate::utils::update_counter(max, size - (current1.x as i32 - current2.x as i32).abs() as usize, size - (current1.y as i32 - current2.y as i32).abs() as usize, size, "A*");
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_doubleastar.png".to_string());
    }
    
    window
}

fn astar_tendril(mut reached_end: bool, mut mtx: Vec<Vec<u8>>, mut maze: Vec<Vec<DoubleMazeNode>>, mut open_heap: BinaryHeap<DoubleMazeNode>, mut other_current: DoubleMazeNode, allegiance: u8, other_allegiance: u8) -> (Vec<Vec<u8>>, Vec<Vec<DoubleMazeNode>>, BinaryHeap<DoubleMazeNode>, DoubleMazeNode, DoubleMazeNode, bool)
{
    let mut current = open_heap.pop().unwrap(); //Get node with lowest f
    current.closed = true;

            
    maze[current.x][current.y].open = false;


    mtx[current.x][current.y] = allegiance;
    maze[current.x][current.y] = current;


    //Get children and update lists
    let mut children = get_children(&maze, current); 

    for i in 0..children.len()
    {
        if !reached_end && children[i].allegiance == other_allegiance { reached_end = true; other_current = children[i] }
        if maze[children[i].x][children[i].y].open || children[i].closed || reached_end { continue; }
        
        children[i].g = current.g + 1;
        children[i].f = children[i].g + crate::utils::euclidean(children[i].x, other_current.x, children[i].y, other_current.y) as usize;
        children[i].allegiance = allegiance;
 
        maze[children[i].x][children[i].y].open = true;
        open_heap.push(children[i]);
    }

    (mtx, maze, open_heap, current, other_current, reached_end)
}


//Initialize graph from the maze matrix
fn graph_init(mtx: &[Vec<u8>], size: usize) -> Vec<Vec<DoubleMazeNode>>
{
    let mut maze_graph: Vec<Vec<DoubleMazeNode>> = vec!();
    for i in 0..size
    {
        let mut maze_row: Vec<DoubleMazeNode> = vec!();
        for j in 0..size
        {
             //heuristic needs to be computed dynamically which is why h isn't included here
            maze_row.push(DoubleMazeNode{
                x: i,
                y: j,
                parent_x: usize::MAX,
                parent_y: usize::MAX,
                is_wall: mtx[i][j] == u8::MAX,
                open: false,
                closed: false,
                allegiance: 0,
                g: usize::MAX,
                f: 0
            });
        }
        maze_graph.push(maze_row);
    }

    maze_graph
}

fn get_children(maze: &[Vec<DoubleMazeNode>], node: DoubleMazeNode) -> Vec<DoubleMazeNode>
{
    let x = node.x;
    let y = node.y;
    let mut children: Vec<DoubleMazeNode> = vec!();

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


//Struct to store maze node
#[derive(Copy, Clone)]
struct DoubleMazeNode
{
    x: usize,
    y: usize,
    parent_x: usize,
    parent_y: usize,
    is_wall: bool,
    open: bool,
    closed: bool,
    allegiance: u8,
    g: usize,
    f: usize
}

impl PartialEq for DoubleMazeNode
{
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}

impl PartialOrd for DoubleMazeNode
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Eq for DoubleMazeNode {}

impl Ord for DoubleMazeNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.f.cmp(&other.f).reverse()
    }
}