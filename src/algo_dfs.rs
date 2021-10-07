extern crate minifb;
use minifb::Window;


pub fn dfs(mut window: Window, params: crate::solve::MazeParams) -> Window
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
    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;
    let mut maze = graph_init(size);

    let mut counter: u128 = 0;
    loop
    {
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);


        mtx[x][y] += 1;
        let trav: Option<crate::solve::Direction> = check_for_traveled(&mtx, x, y, end_x, end_y, 0); //Get travel direction

        let v: Vec<usize>;
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

        max = crate::utils::update_counter(max, x, y, size, "Depth First Search");

        if x == end_x && y == end_y { break; }
 
        counter += 1;
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_dfs.png".to_string());
    }

    window
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
        let man = crate::utils::euclidean(cx[i], end_x, cy[i], end_y) as u32;
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


#[derive(Copy, Clone)]
pub struct LightNode
{
    pub parent_x: usize,
    pub parent_y: usize
}