extern crate minifb;
use minifb::{Window, WindowOptions};

pub enum Direction
{
    Up,
    Right,
    Down,
    Left
}


pub fn solve_maze(mut mtx: Vec<Vec<u8>>, size: usize, algo: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool) 
{
    //Solver init
    let start_x = 1;
    let start_y = 1;
    let end_x = size - 2;
    let end_y = size - 2;

    match algo
    {
        1 => mtx = crate::algo_tremaux::tremaux(mtx, size, start_x, start_y, end_x, end_y, show_animation, anim_scale, anim_speed_mult),
        2 => mtx = crate::algo_astar::astar(mtx, size, start_x, start_y, end_x, end_y, show_animation, anim_scale, anim_speed_mult),

        _ => mtx = crate::algo_astar::astar(mtx, size, start_x, start_y, end_x, end_y, show_animation, anim_scale, anim_speed_mult)
    }

    println!("Solved");
    if save_maze
    {
        crate::toimage::mtx_to_img(&mtx, size, "solved.png".to_string());
    }
}