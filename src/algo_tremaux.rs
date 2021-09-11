pub fn tremaux(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize) -> Vec<Vec<u8>>
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
    let mut x = start_x;
    let mut y = start_y;

    let mut max = 0;

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
            
        let coordinate = traverse_maze(&mtx, x, y);
    
        if is_dead_end(&mtx, x, y, 0)
        {
            mtx[x][y] += 1;
        }
    
        x = coordinate[0];
        y = coordinate[1];

        max = crate::utils::update_counter(max, x, y, size);

        if x == end_x && y == end_y { break; }

        
        counter += 1;
    }


    mtx
}

fn traverse_maze(mtx: &[Vec<u8>], x: usize, y: usize) -> Vec<usize>
{
    //First check if any untravelled paths remain
    let mut trav: Option<crate::solve::Direction> = check_for_traveled(mtx, x, y, 0);
    let mut k = 0;

    while trav.is_none()
    {
        k+=1;
        trav = check_for_traveled(mtx, x, y, k); 
    }

    travel(trav, x, y)
}


//Returns direction to travel to. 0 = can't travel, 1 = up, 2 = right, 3 = down, 4 = left
fn check_for_traveled(mtx: &[Vec<u8>], x: usize, y: usize, can_travel_num: u8) -> Option<crate::solve::Direction>
{
    //look down
    if mtx[x][y+1] <= can_travel_num
    {
        return Some(crate::solve::Direction::Down);
    }
    //look right
    if mtx[x+1][y] <= can_travel_num
    {
        return Some(crate::solve::Direction::Right);
    }
    //look up
    if mtx[x][y-1] <= can_travel_num
    {
        return Some(crate::solve::Direction::Up);
    }
    //look left
    if mtx[x-1][y] <= can_travel_num
    {
        return Some(crate::solve::Direction::Left);
    }  
    None
}

//Checks if at dead end according to relative_num
fn is_dead_end(mtx: &[Vec<u8>], x: usize, y: usize, relative_num: u8) -> bool
{
    check_for_traveled(mtx, x, y, relative_num).is_none()
}

fn travel(dir: Option<crate::solve::Direction>, x: usize, y: usize) -> Vec<usize>
{
    match dir
    {
        Some(crate::solve::Direction::Up) => vec![x, y-1],
        Some(crate::solve::Direction::Right) => vec![x+1,y],
        Some(crate::solve::Direction::Down) => vec![x, y+1],
        Some(crate::solve::Direction::Left) => vec![x-1, y],
        _ => [0,0].to_vec()
    }
}
