use rand::Rng;

pub fn tremaux(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0, "Tremaux");

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size, "Tremaux");
    }

    //Algo init
    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;
    let mut maze = crate::algo_dfs::graph_init(size);

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
        let trav: Option<crate::solve::Direction> = trav_rand(&mtx, x, y, 0); //Get travel direction

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
        if is_dead_end(&mtx, x, y, 0)
        {
            mtx[x][y] += 1;
        }
    
        x = v[0];
        y = v[1];

        max = crate::utils::update_counter(max, x, y, size, "Tremaux");

        if x == end_x && y == end_y { break; }
 
        counter += 1;
    }
    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_tremaux.png".to_string());
    }
}

//Returns direction to travel to. 0 = can't travel, 1 = up, 2 = right, 3 = down, 4 = left
fn trav_rand(mtx: &[Vec<u8>], x: usize, y: usize, can_travel_num: u8) -> Option<crate::solve::Direction>
{
    let mut v = vec!(mtx[x][y-1], mtx[x-1][y], mtx[x][y+1], mtx[x+1][y]);
    let mut d = vec!(crate::solve::Direction::Up, crate::solve::Direction::Left, crate::solve::Direction::Down, crate::solve::Direction::Right);

    let mut s = v.len();
    for _i in 0..v.len() //Loop through possible directions
    {
        //Randomize route
        let n = rand::thread_rng().gen_range(0..s);
        if v[n] > can_travel_num //Determine if route can be traveled on and find closest one to the end
        {
            v.remove(n);
            d.remove(n);
            s-=1;
        }
        else 
        {
            return Some(d[n]);
        }
    }
    None
}

//Checks if at dead end according to relative_num
fn is_dead_end(mtx: &[Vec<u8>], x: usize, y: usize, relative_num: u8) -> bool
{
    trav_rand(mtx, x, y, relative_num).is_none()
}