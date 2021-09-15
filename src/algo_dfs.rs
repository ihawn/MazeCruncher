#[derive(Copy, Clone)]
pub struct LightNode
{
    pub parent_x: usize,
    pub parent_y: usize
}

pub fn dfs(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0, "Depth First Search*");

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size, "Depth First Search");
    }

    //Algo init
    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;
    let mut maze = graph_init(size);

    let mut counter: u128 = 0;
    loop
    {
        //update window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }

        mtx[x][y] += 1;
        let trav: Option<crate::solve::Direction> = check_for_traveled(&mtx, x, y, end_x, end_y, 0); //Get travel direction

        let mut v: Vec<usize> = vec!();
        match trav //Travel based on direction. Set parent 
        {
            Some(crate::solve::Direction::Up) => {v = vec![x, y-1]; maze[x][y-1].parent_x = x; maze[x][y-1].parent_y = y;},
            Some(crate::solve::Direction::Right) => {v = vec![x+1,y]; maze[x+1][y].parent_x = x; maze[x+1][y].parent_y = y;},
            Some(crate::solve::Direction::Down) => {v = vec![x, y+1]; maze[x][y+1].parent_x = x; maze[x][y+1].parent_y = y;},
            Some(crate::solve::Direction::Left) => {v = vec![x-1, y]; maze[x-1][y].parent_x = x; maze[x-1][y].parent_y = y;},
            _ => v = vec!(maze[x][y].parent_x, maze[x][y].parent_y)
        }

        //Add 1 to dead ends (since we only hit them once but they need to have a value of 2)
        if is_dead_end(&mtx, x, y, end_x, end_y, 0)
        {
            mtx[x][y] += 1;
        }
    
        x = v[0];
        y = v[1];

        max = crate::utils::update_counter(max, x, y, size);

        if x == end_x && y == end_y { break; }
 
        counter += 1;
    }
    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_dfs.png".to_string());
    }
}

//Returns direction to travel to. 0 = can't travel, 1 = up, 2 = right, 3 = down, 4 = left
fn check_for_traveled(mtx: &[Vec<u8>], x: usize, y: usize, end_x: usize, end_y: usize, can_travel_num: u8) -> Option<crate::solve::Direction>
{
    let cx = vec!(x, x-1, x, x+1);
    let cy = vec!(y-1, y, y+1, y);
    let v = vec!(mtx[x][y-1], mtx[x-1][y], mtx[x][y+1], mtx[x+1][y]);
    let d = vec!(crate::solve::Direction::Up, crate::solve::Direction::Left, crate::solve::Direction::Down, crate::solve::Direction::Right);

    let mut min_val = u32::MAX;
    let mut min_loc = 4;
    for i in 0..v.len() //Loop through possible directions
    {
        let man = crate::algo_astar::manhatten(cx[i], end_x, cy[i], end_y);
        if v[i] <= can_travel_num && man < min_val //Determine if route can be traveled on and find closest one to the end
        {
            min_val = man;
            min_loc = i;
        }
    }

    if min_loc != 4 { return Some(d[min_loc]) }

    None
}

//Checks if at dead end according to relative_num
fn is_dead_end(mtx: &[Vec<u8>], x: usize, y: usize, end_x: usize, end_y: usize, relative_num: u8) -> bool
{
    check_for_traveled(mtx, x, y, end_x, end_y, relative_num).is_none()
}

//Copy the maze matrix into graph form so we can save each cell's parent
pub fn graph_init(size: usize) -> Vec<Vec<LightNode>> 
{
    let mut maze_graph: Vec<Vec<LightNode>> = vec!();
    for _i in 0..size
    {
        let mut maze_row: Vec<LightNode> = vec!();
        for _j in 0..size
        {
            maze_row.push(LightNode{parent_x: 0, parent_y: 0});
        }
        maze_graph.push(maze_row);
    }

    maze_graph
}


