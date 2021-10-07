use std::{thread, time};
use minifb::Window;

extern crate minifb;

#[derive(Copy, Clone)]
pub enum Direction
{
    Up,
    Right,
    Down,
    Left
}


pub fn solve_maze(mut window: Window, buff_size: usize, mtx: Vec<Vec<u8>>, size: usize, algo: usize, show_animation: bool, anim_speed_mult: usize, save_maze: bool) 
{
    //Solver init
    let start_x = 1;
    let start_y = 1;
    let end_x = size - 2;
    let end_y = size - 2;

    let params: MazeParams = MazeParams{
        mtx,
        size,
        start_x,
        start_y,
        end_x,
        end_y,
        save_maze,
        show_animation,
        anim_speed_mult,
        buff_size
    };


    //Control flow for all algo selection
    let mut start = algo;
    let mut stop = algo+1;
    if algo == 7 { start = 1; stop = 7; }
    
    
    for al in start..stop
    {
        match al
        {
            1 => window = crate::algo_dfs::dfs(window, params.clone()),
            2 => window = crate::algo_bfs::bfs(window, params.clone()),
            3 => window = crate::algo_def::def(window, params.clone()),
            4 => window = crate::algo_astar::astar(window, params.clone()),
            5 => window = crate::algo_dijkstra::dijkstra(window, params.clone()),
            6 => window = crate::algo_tremaux::tremaux(window, params.clone()),
            _ => window = crate::algo_astar::astar(window, params.clone())
        }
        thread::sleep(time::Duration::from_secs(2));
    }
}

#[derive(Clone)]
pub struct MazeParams
{
    pub mtx: Vec<Vec<u8>>,
    pub size: usize,
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub save_maze: bool,
    pub show_animation: bool,
    pub anim_speed_mult: usize,
    pub buff_size: usize
}