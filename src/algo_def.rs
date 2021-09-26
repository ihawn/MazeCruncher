//Dead end filling
extern crate minifb;

pub fn def(mut mtx: Vec<Vec<u8>>, size: usize, start_x: usize, start_y: usize, end_x: usize, end_y: usize, save_maze: bool, show_animation: bool, anim_scale: usize, anim_speed_mult: usize)
{
    //Graphics init
    let buff_size = size*anim_scale;
    let mut buffer: Vec<u32> = vec![0;  1];

    let mut window = crate::utils::window_init(0, "Dead End Filling");

    if show_animation
    {
        buffer = vec![0;  buff_size*buff_size];
        window = crate::utils::window_init(buff_size, "Dead End Filling");
    }

    //Algo init
    let mut dead_list: Vec<(usize, usize)> = Vec::new(); 
    
    //Find initial dead ends
    for i in 1..size-1
    {
        for j in 1..size-1
        {
            if !(i == start_x && j == start_y) && !(i == end_x && j == end_y) && mtx[i][j] != u8::MAX && get_children(&mtx, (i,j)).len() <= 1 //is dead end
            {
                dead_list.push((i,j));
            }
        }
    }
    
    let mut counter: u128 = 0;
    while !dead_list.is_empty()
    {
        //thread::sleep(time::Duration::from_secs(1));
        //update window
        if show_animation && counter % anim_speed_mult as u128 == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }

        let mut i = 0;
        let mut s = dead_list.len();
        while i < s
        {
            let children = get_children(&mtx, dead_list[i]);
            if children.len() == 1 && !(children[0].0 == start_x && children[0].1 == start_y) && !(children[0].0 == end_x && children[0].1 == end_y)
            {        
                mtx[dead_list[i].0][dead_list[i].1] = 2;
                dead_list[i] = children[0];
            }
            else
            {
                dead_list.remove(i);
                s-=1;  
            }
            i+=1;
        }

        counter += 1;
    }

    
    //Fill in the path(s)
    for i in 1..mtx.len()-1
    {
        for j in 1..mtx.len()-1
        {
            if mtx[i][j] == 0 { mtx[i][j] = 1; }
        }

        if show_animation && i % anim_speed_mult == 0
        {
            buffer = crate::utils::update_buffer(&mtx, size, buffer);
            window
            .update_with_buffer(&buffer, size, size)
            .unwrap();
        }
    }

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_def.png".to_string());
    }
}



//Get adjacent nodes
fn get_children(maze: &[Vec<u8>], node: (usize, usize)) -> Vec<(usize, usize)>
{
    let x = node.0;
    let y = node.1;
    let mut children: Vec<(usize, usize)> = vec!();

    if x < maze[0].len()-1 && maze[x+1][y]!=u8::MAX && maze[x+1][y]!=2 { children.push((x+1,y)); }
    if x > 0 && maze[x-1][y]!=u8::MAX && maze[x-1][y]!=2 { children.push((x-1,y)); }
    if y < maze[0].len()-1 && maze[x][y+1]!=u8::MAX && maze[x][y+1]!=2 { children.push((x,y+1)); }
    if y > 0 && maze[x][y-1]!=u8::MAX && maze[x][y-1]!=2 { children.push((x,y-1)); }


    children
}
