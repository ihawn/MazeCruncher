//Dead end filling
extern crate minifb;
use minifb::Window;


pub fn def(mut window: Window, params: crate::solve::MazeParams) -> Window
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
    let mut dead_list: Vec<(usize, usize)> = Vec::new(); 
    println!("Solving Maze with Dead End Filling");
    
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
        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);


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
    }

    window = crate::utils::update_window(window, show_animation, 0, &mtx, size, anim_speed_mult, buff_size);

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved_def.png".to_string());
    }

    window
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
