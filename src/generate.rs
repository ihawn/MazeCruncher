use std::vec;
use rand::Rng;

pub fn generate_and_solve(mut size: usize, gen_algo: usize, algo: usize, decimation: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool)
{
    //Make sure maze has odd size
    if size % 2 == 0
    {
        size += 1;
    }

    let n = size;
    let m = size;    
    let mut m:Vec<Vec<u8>> = vec![vec![4; n]; m];
    m = prime_matrix(m, size);

    //Window init
    let buff_size = size*anim_scale;
    let mut window = crate::utils::window_init(0, "Maze!");
    
    if show_animation
    {
        window = crate::utils::window_init(buff_size, "Maze!");
    }
    
    window = crate::utils::update_window(window, show_animation, 0, &m, size, anim_speed_mult, buff_size);
    let t = crate::growing_tree::growing_tree(window, buff_size, m, size, decimation, show_animation, anim_speed_mult, save_maze);
    m = t.0; window = t.1;
    crate::solve::solve_maze(window, buff_size, m, size, algo, show_animation, anim_speed_mult, save_maze);
}

//Initialize the zero matrix in a "cell" format which surrounds even entries with "walls"
fn prime_matrix(mut mtx: Vec<Vec<u8>>, size: usize) -> Vec<Vec<u8>>
{   
    for i in 0..size
    {
        for j in 0..size
        {
            if (i+1) % 2 != 0 || (j+1) % 2 != 0
            {
                mtx[i][j] = u8::MAX;
            }
        }
    }

    mtx
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