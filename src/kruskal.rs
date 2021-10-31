use std::{cmp::Ordering};
use rand::Rng;
use minifb::Window;
use std::collections::BinaryHeap;
use std::collections::HashSet;


pub fn kruskal(mut window: Window, buff_size: usize, mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_speed_mult: usize) -> (Vec<Vec<u8>>, Window)
{
    let cell_count = usize::pow(size-1, 2)/4;
    let mut cell_sets: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); cell_count];
    let mut mirror: Vec<Vec<u64>> = vec![vec![0; size]; size];

    let mut id: u64 = 0;
    //Initialize mirror matrix to store set ids
    for i in (1..size-1).step_by(2)
    {
        for j in (1..size-1).step_by(2)
        {
            let mut hs = HashSet::new();
            hs.insert((i,j));
            mirror[i][j] = id;
            cell_sets[id as usize] = hs;          
            id+=1;
        }
    }


    let mut edge_heap = get_edges(size);
    let mut counter: u128 = 0;
    let itt: u128 = edge_heap.len() as u128;

    while !edge_heap.is_empty()
    {
        if counter%(itt/100) == 0
        {
            let m = 100*counter/itt + 1;
            println!("Generating Maze: {}%", m);
        }


        let edge = edge_heap.pop().unwrap();

              
        //Vertical edges
        if edge.x % 2 == 0
        {
            if mirror[edge.x-1][edge.y] != mirror[edge.x+1][edge.y]
            {
                let s = set_merge(cell_sets, mtx, mirror, (edge.x-1, edge.y), (edge.x+1, edge.y), (edge.x, edge.y));
                cell_sets = s.0; mtx = s.1; mirror = s.2;
            }
        }

        //Horizontal edges
        else
        {
            if mirror[edge.x][edge.y-1] != mirror[edge.x][edge.y+1]
            {
                let s = set_merge(cell_sets, mtx, mirror, (edge.x, edge.y-1), (edge.x, edge.y+1), (edge.x, edge.y));
                cell_sets = s.0; mtx = s.1; mirror = s.2;
            }
        }

        

        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);
        counter += 1;
    }

    
    //Remove odd edge
    for i in 1..size-1
    {
        for j in 1..size-1
        {
            let mut sum = 0;
            let perm_x: Vec<i8> = vec![1, 1, -1, -1];
            let perm_y: Vec<i8> = vec![1, -1, 1, -1];
            for k in 0..4
            {
                if mtx[i+perm_x[k] as usize][j+perm_y[k] as usize] == u8::MAX { sum += 1; }
            }
            if sum == 4 { mtx[i][j] = 0; }
        }
    }

    (mtx, window)
}



fn set_merge(mut cell_sets: Vec<HashSet<(usize, usize)>>, mut mtx: Vec<Vec<u8>>, mut mirror: Vec<Vec<u64>>, pt1: (usize, usize), pt2: (usize, usize), edge_pt: (usize, usize)) -> (Vec<HashSet<(usize, usize)>>, Vec<Vec<u8>>, Vec<Vec<u64>>)
{
    let a = cell_sets[mirror[pt1.0][pt1.1] as usize].clone();
    let b = cell_sets[mirror[pt2.0][pt2.1] as usize].clone();
    let c: HashSet<(usize, usize)>;
    let id;

    //Pick the longer of the two sets as the dominant one so changing the other's id takes less time
    if b.len() < a.len()
    {
        cell_sets[mirror[pt1.0][pt1.1] as usize].extend(&b);
        cell_sets[mirror[pt1.0][pt1.1] as usize].insert((edge_pt.0, edge_pt.1));
        c = b;
        id = mirror[pt1.0][pt1.1];
    }
    else
    {
        cell_sets[mirror[pt2.0][pt2.1] as usize].extend(&a);
        cell_sets[mirror[pt2.0][pt2.1] as usize].insert((edge_pt.0, edge_pt.1));
        c = a;
        id = mirror[pt2.0][pt2.1];
    }

    //Change the maze and miror matrices accordingly    
    for t in c
    { 
        mtx[t.0][t.1] = 0;
        mirror[t.0][t.1] = id;
    }
    mtx[edge_pt.0][edge_pt.1] = 0;     
    mirror[edge_pt.0][edge_pt.1] = 0; 

    (cell_sets, mtx, mirror)
}

//Gets the maze edges and randomizes them. Returns all in a binary stack
fn get_edges(size: usize) -> BinaryHeap<GridEdge>
{
    let mut edge_heap: BinaryHeap<GridEdge> = BinaryHeap::new();

    let mut p = 1;
    for i in p..size-1
    {
        for j in (p%2+1..size-1).step_by(2)
        {
            edge_heap.push(GridEdge {
                id: rand::thread_rng().gen_range(0..u64::MAX),
                x: i,
                y: j,
            });
        }

        p = p%2 + 1;
    }
    edge_heap
}

struct GridEdge
{
    id: u64,
    x: usize,
    y: usize,
}

impl PartialEq for GridEdge
{
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}

impl Eq for GridEdge {}

impl PartialOrd for GridEdge
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl Ord for GridEdge
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.id.cmp(&other.id)
    }
}