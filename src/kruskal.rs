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
    let mut counter = 0;

    while !edge_heap.is_empty()
    {
        let edge = edge_heap.pop().unwrap();
              
        //Vertical edges
        if edge.x % 2 == 0
        {
            let id = mirror[edge.x-1][edge.y];
            if cell_sets[mirror[edge.x-1][edge.y] as usize] != cell_sets[mirror[edge.x+1][edge.y] as usize]//id != mirror[edge.x+1][edge.y]
            {
                let b = cell_sets[mirror[edge.x+1][edge.y] as usize].clone();
                cell_sets[mirror[edge.x-1][edge.y] as usize].extend(&b);
                cell_sets[mirror[edge.x-1][edge.y] as usize].insert((edge.x, edge.y));

                //Change the maze and miror matrices accordingly
                let last = cell_sets[mirror[edge.x+1][edge.y] as usize].clone();
                for c in last
                { 
                    mtx[c.0][c.1] = 0;
                    mirror[c.0][c.1] = id;
                }
                mtx[edge.x][edge.y] = 0;     
                mirror[edge.x][edge.y] = 0;
            }            
        }

        //Horizontal edges
        else
        {
            let id = mirror[edge.x][edge.y-1];
            if cell_sets[mirror[edge.x][edge.y-1] as usize] != cell_sets[mirror[edge.x][edge.y+1] as usize] //id != mirror[edge.x][edge.y+1]
            {
                let b = cell_sets[mirror[edge.x][edge.y+1] as usize].clone();
                cell_sets[mirror[edge.x][edge.y-1] as usize].extend(&b);
                cell_sets[mirror[edge.x][edge.y-1] as usize].insert((edge.x, edge.y));

                
                //Change the maze and miror matrices accordingly
                let last = cell_sets[mirror[edge.x][edge.y+1] as usize].clone();
                for c in last
                { 
                    mtx[c.0][c.1] = 0;
                    mirror[c.0][c.1] = id;
                } 
                mtx[edge.x][edge.y] = 0;     
                mirror[edge.x][edge.y] = 0;
            }
        }

        window = crate::utils::update_window(window, show_animation, counter, &mtx, size, anim_speed_mult, buff_size);
        counter += 1;
    }

    (mtx, window)
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