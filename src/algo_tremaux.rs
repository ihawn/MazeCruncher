enum Direction
{
    Up,
    Right,
    Down,
    Left
}

pub fn tremaux(mut mtx: Vec<Vec<u8>>, mut x: usize, mut y: usize) -> (Vec<Vec<u8>>, usize, usize)
{
    mtx[x][y] += 1;
            
    let coordinate = traverse_maze(&mtx, x, y);

    if is_dead_end(&mtx, x, y, 0)
    {
        mtx[x][y] += 1;
    }

    x = coordinate[0];
    y = coordinate[1];

    (mtx, x, y)
}

fn traverse_maze(mtx: &[Vec<u8>], x: usize, y: usize) -> Vec<usize>
{
    let mut coord = vec![0, 0];

    //First check if any untravelled paths remain
    let mut trav: Option<Direction> = check_for_traveled(mtx, x, y, 0);
    let mut k = 0;

    while trav.is_none()
    {
        k+=1;
        trav = check_for_traveled(mtx, x, y, k); 
    }


    match trav
    {
        Some(Direction::Up) => coord = vec![x, y-1],
        Some(Direction::Right) => coord = vec![x+1,y],
        Some(Direction::Down) => coord = vec![x, y+1],
        Some(Direction::Left) => coord = vec![x-1, y],

        _ => println!("Something terrible has happened"),
    }

    coord
}


//Returns direction to travel to. 0 = can't travel, 1 = up, 2 = right, 3 = down, 4 = left
fn check_for_traveled(mtx: &[Vec<u8>], x: usize, y: usize, can_travel_num: u8) -> Option<Direction>
{
    //look down
    if mtx[x][y+1] <= can_travel_num
    {
        return Some(Direction::Down);
    }
    //look right
    if mtx[x+1][y] <= can_travel_num
    {
        return Some(Direction::Right);
    }
    //look up
    if mtx[x][y-1] <= can_travel_num
    {
        return Some(Direction::Up);
    }
    //look left
    if mtx[x-1][y] <= can_travel_num
    {
        return Some(Direction::Left);
    }  
    None
}

//Checks if at dead end according to relative_num
fn is_dead_end(mtx: &[Vec<u8>], x: usize, y: usize, relative_num: u8) -> bool
{
    check_for_traveled(mtx, x, y, relative_num).is_none()
}
