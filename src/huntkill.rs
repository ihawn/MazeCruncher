use minifb::Window;
use rand::Rng;
use std::collections::BinaryHeap;
use std::{cmp::Ordering};


pub fn hunt_and_kill(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_speed_mult: usize) -> (Vec<Vec<u8>>, Window)
{
    //Maze generation init
    let cell_size = (size - 1)/2;
    let mut x = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut y = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut x_list = Vec::new();
    let mut y_list = Vec::new();
    let mut visited_list: BinaryHeap<HuntNode> = BinaryHeap::new();
    visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: x, y: y});
    x_list.push(x); //push the starting point onto the list
    y_list.push(y);
    mtx[x][y] = 0;

    let s: u128 = size as u128;
    let itt: u128 = (s - 3)*(s + 1)/4;
    for k in 0..itt
    {
        crate::utils::update_gen_counter(itt, k);

        window = crate::utils::update_window(window, show_animation, k, &mtx, size, anim_speed_mult, buff_size);

        x = *x_list.last().unwrap();
        y = *y_list.last().unwrap();


        //Check if current tile can go farther. If not, scan the maze and search for the first unvisited cell with visited adjacent cell
        let mut travel = travel_to(&mtx, size, &x, &y);
        if travel.is_empty() && !x_list.is_empty()
        {
            let mut tried_nodes: Vec<HuntNode> = vec![];

            while travel.is_empty()
            {
                let node = visited_list.pop().unwrap();
                x = node.x;
                y = node.y;
                travel = travel_to(&mtx, size, &x, &y);
                tried_nodes.push(node);
            }

            //Add the tried nodes back to the binary heap
            for t in tried_nodes
            {
                visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: t.x, y: t.y});
            }
        }

        let n = rand::thread_rng().gen_range(0..travel.len());

        if !x_list.is_empty()
        {
            let t = travel_rand(visited_list, mtx, x, y, n, travel, x_list, y_list);
            visited_list = t.0; mtx = t.1; x_list = t.2; y_list = t.3;
        }
    }


    (mtx, window)
}

//Function to determine if current cell has any adjacent, non-visited cells
fn travel_to(mtx: &[Vec<u8>], size: usize, x: &usize, y: &usize) -> Vec<i32>
{
    let mut dirs = vec![1, 2, 3, 4];

    if *y == 1 || mtx[*x][*y-2] == 0 { dirs = remove_value(dirs, 1); } //look up   
    if *x == size - 2 || mtx[*x+2][*y] == 0 { dirs = remove_value(dirs, 2); } //look right
    if *y == size - 2 || mtx[*x][*y+2] == 0 { dirs = remove_value(dirs, 3); } //look down    
    if *x == 1 || mtx[*x-2][*y] == 0 { dirs = remove_value(dirs, 4); } //look left

    dirs
}

//Function to remove value from vector
fn remove_value(mut values: Vec<i32>, value: i32) -> Vec<i32>
{
    for i in 0..values.len()
    {
        if values[i] == value
        {
            values.remove(i);
            break;
        }
    }

    values
}

fn travel_rand(mut visited_list: BinaryHeap<HuntNode>, mut mtx: Vec<Vec<u8>>, x: usize, y: usize, n: usize, travel: Vec<i32>, mut x_list: Vec<usize>, mut y_list: Vec<usize>) -> (BinaryHeap<HuntNode>, Vec<Vec<u8>>, Vec<usize>, Vec<usize>)
{
    //movement up, right, down, or left
    match travel[n]
    {
        1 => {mtx[x][y-2] = 0;
              mtx[x][y-1] = 0;
              x_list.push(x);
              y_list.push(y-2);
              visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: x, y: y-2})},

        2 => {mtx[x+2][y] = 0;
              mtx[x+1][y] = 0;
              x_list.push(x+2);
              y_list.push(y);
              visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: x+2, y: y})},

        3 => {mtx[x][y+2] = 0;
              mtx[x][y+1] = 0;
              x_list.push(x);
              y_list.push(y+2);
              visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: x, y: y+2})},

        4 => {mtx[x-2][y] = 0;
              mtx[x-1][y] = 0;
              x_list.push(x-2);
              y_list.push(y);
              visited_list.push(HuntNode {id: rand::thread_rng().gen_range(0..u64::MAX), x: x-2, y: y})},

        _ => println!("Something terrible has happened"),
    }

    (visited_list, mtx, x_list, y_list)
}

struct HuntNode
{
    id: u64,
    x: usize,
    y: usize,
}

impl PartialEq for HuntNode
{
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for HuntNode {}

impl PartialOrd for HuntNode
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl Ord for HuntNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.id.cmp(&other.id)
    }
}