extern crate minifb;
extern crate queues;
use queues::*;


pub fn bfs(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0, "Breadth First Search");

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size, "Breadth First Search");
    }

    //Algo init
    let mut maze = graph_init(&mtx, size);
    let mut node_queue: Queue<BNode> = queue![];
    node_queue.add(maze[start_x][start_y]);

    
    let end_node = maze[end_x][end_y];
    let mut max = 0; //For the counter


    let mut counter: u128 = 0;
    while node_queue.size() > 0
    {
        //update window
        if show_animation && counter % (anim_speed_mult*5) as u128 == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }

        let mut current: BNode = node_queue.remove().unwrap();
        mtx[current.x][current.y] = 2;
        
        //Get adjacent cells
        let children = get_children(&maze, current);

        //Loop through adjacent cells and update parents and distances
        for mut c in children
        {
            if mtx[c.x][c.y] != 2
            {
                maze[c.x][c.y].parent_x = current.x;
                maze[c.x][c.y].parent_y = current.y;
                mtx[c.x][c.y] = 2;

                c.parent_x = current.x;
                c.parent_y = current.y;

                node_queue.add(c);
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

        counter += 1;
        max = crate::utils::update_counter(max, current.x, current.y, size, "Breadth First Search");
    }

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_bfs.png".to_string());
    }
}

//Get adjacent nodes
fn get_children(maze: &[Vec<BNode>], node: BNode) -> Vec<BNode>
{
    let x = node.x;
    let y = node.y;
    let mut children: Vec<BNode> = vec!();

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
fn graph_init(mtx: &[Vec<u8>], size: usize) -> Vec<Vec<BNode>>
{
    let mut maze_graph: Vec<Vec<BNode>> = vec!();
    for i in 0..size
    {
        let mut maze_row: Vec<BNode> = vec!();
        for j in 0..size
        {
            let node = BNode{
                x: i,
                y: j,
                parent_x: usize::MAX,
                parent_y: usize::MAX,
                is_wall: mtx[i][j] == u8::MAX
            };
            maze_row.push(node);
        }
        maze_graph.push(maze_row);
    }
    maze_graph
}

//Struct to store maze node
#[derive(Copy, Clone)]
struct BNode
{
    x: usize,
    y: usize,
    parent_x: usize,
    parent_y: usize,
    is_wall: bool,
}

impl PartialEq for BNode
{
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y }
}
