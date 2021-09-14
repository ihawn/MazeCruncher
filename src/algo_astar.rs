extern crate minifb;

#[derive(Copy, Clone)]
pub struct MazeNode
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

pub fn astar(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0);

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size);
    }

    //Algo init
    let mut maze = graph_init(&mtx, size, end_x, end_y);

    let mut open_list = Vec::new();
    let mut closed_list= Vec::new();
    maze[start_x][start_y].open = true;
    open_list.push(maze[start_x][start_y]);

    let mut current = open_list[0];
    let end_node = maze[end_x][end_y];
    let mut lowest_loc: usize = 0;

    let mut max = 0;

    let mut counter: u128 = 0;
    while !open_list.is_empty()
    {
        //update window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }

        lowest_loc = find_lowest(&open_list);
        let mut min_loc: usize = 0;
    
        current = open_list[lowest_loc]; //Find node in open list with lowest f
        
        current.closed = true;
        maze[current.x][current.y].open = false;
        maze[current.x][current.y].closed = true;
        open_list.remove(lowest_loc); //Remove current from open list and move to closed
        closed_list.push(current);


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
            open_list.push(children[i]);
        }



        counter += 1;
        max = crate::utils::update_counter(max, current.x, current.y, size);
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
                g: 0,
                f: 0
            });
        }
        maze_graph.push(maze_row);
    }

    maze_graph
}

fn find_lowest(lst: &[MazeNode]) -> usize
{
    let mut min_loc: usize = 0;
    let mut min_val = lst[0].f;
    

    for i in 1..lst.len()
    {
        if lst[i].f < min_val
        {
            min_val = lst[i].f;
            min_loc = i;
        }
    }

    min_loc
}

fn get_children(maze: &[Vec<MazeNode>], node: MazeNode) -> Vec<MazeNode>
{
    let x = node.x;
    let y = node.y;
    let mut children: Vec<MazeNode> = vec!();
    for _i in 0..4
    {
        if x < maze[0].len()-1 && !maze[x+1][y].is_wall { children.push(maze[x+1][y]); }
        if x > 0 && !maze[x-1][y].is_wall { children.push(maze[x-1][y]); }
        if y < maze[0].len()-1 && !maze[x][y+1].is_wall { children.push(maze[x][y+1]); }
        if y > 0 && !maze[x][y-1].is_wall { children.push(maze[x][y-1]); }
    }

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
