extern crate minifb;

#[derive(Copy, Clone)]
pub enum Direction
{
    Up,
    Right,
    Down,
    Left
}


pub fn solve_maze(mtx: Vec<Vec<u8>>, size: usize, algo: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool) 
{
    //Solver init
    let start_x = 1;
    let start_y = 1;
    let end_x = size - 2;
    let end_y = size - 2;

    match algo
    {
        1 => crate::algo_dfs::dfs(mtx, size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult),
        2 => crate::algo_astar::astar(mtx, size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult),
        3 => crate::algo_tremaux::tremaux(mtx, size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult),
        4 => {
            crate::algo_dfs::dfs(mtx.clone(), size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult);
            crate::algo_astar::astar(mtx.clone(), size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult);
            crate::algo_tremaux::tremaux(mtx, size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult);
        }
        _ => crate::algo_astar::astar(mtx, size, start_x, start_y, end_x, end_y, save_maze, show_animation, anim_scale, anim_speed_mult)
    }


}