use std::{cmp::Ordering};
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use minifb::Window;
use std::collections::BinaryHeap;


pub fn prim(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_speed_mult: usize) -> (Vec<Vec<u8>>, Window)
{
    let cell_size = (size - 1)/2;
    let start_x: usize =  rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let start_y: usize =  rand::thread_rng().gen_range(0..cell_size)*2 + 1;

    let mut open: BinaryHeap<PrimNode> = BinaryHeap::new();

    open.push(PrimNode{
        id: rand::thread_rng().gen_range(0..u64::MAX),
        x: start_x,
        y: start_y,
    });
    
    let mut counter: u128 = 0;
    let s: u128 = size as u128;
    let itt: u128 = (s - 3)*(s + 1)/4;

    while !open.is_empty()
    {
        crate::utils::update_gen_counter(itt, counter);

        let current = open.pop().unwrap();
        mtx[current.x][current.y] = 0;

        //Connect current to nearest closed
        mtx = join_closest_closed(mtx, &current, size);

        //Get adjacent and add to open heap
        let mh = add_adj(mtx, open, &current, size);
        mtx = mh.0; open = mh.1;

        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);
        counter += 1;
    }


    (mtx, window)
}

//Get adjacent cells and add them to heap
fn add_adj(mut mtx: Vec<Vec<u8>>, mut heap: BinaryHeap<PrimNode>, node: &PrimNode, size: usize) -> (Vec<Vec<u8>>, BinaryHeap<PrimNode>)
{
    let perm: Vec<(i32, i32)> = vec![(2, 0), (0, 2), (0, -2), (-2, 0)];

    for i in 0..4
    {
        let x = (node.x as i32 + perm[i].0) as usize;
        let y = (node.y as i32 + perm[i].1) as usize;
        if x < size - 1 && x > 0 && y < size - 1 && y > 0 && mtx[x][y] == u8::MAX
        {
            heap.push(PrimNode{
                id: rand::thread_rng().gen_range(0..u64::MAX),
                x: x,
                y: y,
            });

            mtx[x][y] = 2;
        }
    }

    (mtx, heap)
}

fn join_closest_closed(mut mtx: Vec<Vec<u8>>, node: &PrimNode, size: usize) -> Vec<Vec<u8>>
{
    let mut perm: Vec<(i32, i32)> = vec![(2, 0), (0, 2), (0, -2), (-2, 0)];
    perm.shuffle(&mut thread_rng());

    for i in 0..4
    {
        let x = (node.x as i32 + perm[i].0) as usize;
        let y = (node.y as i32 + perm[i].1) as usize;
        if x < size - 1 && x > 0 && y < size - 1 && y > 0 && mtx[x][y] == 0
        {
            mtx[(node.x as i32 + perm[i].0/2) as usize][(node.y as i32 + perm[i].1/2) as usize] = 0;
            break;
        }
    }

    mtx
}

struct PrimNode
{
    id: u64,
    x: usize,
    y: usize,
}

impl PartialEq for PrimNode
{
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for PrimNode {}

impl PartialOrd for PrimNode
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl Ord for PrimNode
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.id.cmp(&other.id)
    }
}