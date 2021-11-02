use std::vec;
use rand::Rng;
use minifb::Window;


pub fn generate_and_solve(mut size: usize, gen_algo: usize, algo: usize, decimation: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool)
{
    println!("Generating Maze");

    //Make sure maze has odd size
    if size % 2 == 0
    {
        size += 1;
    }

    let n = size;
    let m = size;    
    let mut m: Vec<Vec<u8>> = vec![vec![u8::MAX; n]; m];

    //Window init
    let buff_size = size*anim_scale;
    let mut window = crate::utils::window_init(0, "Maze!");
    
    if show_animation
    {
        window = crate::utils::window_init(buff_size, "Maze!");
    }
    
    let t: (Vec<Vec<u8>>, Window);
    match gen_algo
    {
        1 => t = crate::growing_tree::growing_tree(window, buff_size, m, size, show_animation, anim_speed_mult),
        2 => t = crate::kruskal::kruskal(window, buff_size, m, size, show_animation, anim_speed_mult),
        3 => t = crate::prim::prim(window, buff_size, m, size, show_animation, anim_speed_mult),
        4 => t = crate::huntkill::hunt_and_kill(window, buff_size, m, size, show_animation, anim_speed_mult),
        _ => t = crate::growing_tree::growing_tree(window, buff_size, m, size, show_animation, anim_speed_mult)
    }

    m = t.0; window = t.1;
    window = crate::utils::update_window(window, show_animation, 0, &m, size, anim_speed_mult, buff_size);
    
    //Prevent a perfect maze for decimation factor > 0
    if decimation > 0 { m = crate::generate::decimate_maze(m, size, decimation); }

    //print_matrix(&mtx, size);
    if save_maze
    {
        crate::toimage::mtx_to_img(&m, size, "unsolved.png".to_string());
    }
    
    println!("Maze Generation Complete");
    
    crate::solve::solve_maze(window, buff_size, m, size, algo, show_animation, anim_speed_mult, save_maze);
}

//Remove walls to allow for multiple solutions
pub fn decimate_maze(mut mtx: Vec<Vec<u8>>, size: usize, factor: usize) -> Vec<Vec<u8>>
{
    println!("Decimating Maze");

    for i in 1..size-1
    {
        for j in 1..size-1
        {
            //If either side is a wall and the perpendicular neighborhood is a cooridoor (this avoids strange looking corners)
            if (mtx[i-1][j] == u8::MAX && mtx[i+1][j] == u8::MAX && mtx[i][j-1] != u8::MAX && mtx[i][j+1] != u8::MAX)  || 
            (mtx[i][j-1] == u8::MAX && mtx[i][j+1] == u8::MAX && mtx[i-1][j] != u8::MAX && mtx[i+1][j] != u8::MAX)
            {
                if rand::thread_rng().gen_range(0..100) < factor && rand::thread_rng().gen_range(0..100) < factor //Probability of making a wall a coridor
                {
                    mtx[i][j] = 0;
                }
            }
        }
    }

    mtx
}