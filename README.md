# MazeCruncher
Welcome to maze cruncher!

<a href="https://github.com/ihawn/MazeCruncher/releases/tag/MazeCruncher">Download Here</a>

## Usage
* To get started, just run the standalone .exe or compile and run the source code yourself.
* Both the solved and unsolved mazes are saved as a png in the directory that the .exe is run from.
* A solution animation can be displayed too but it will slow the algorithm considerably, though it looks very interesting.
* The algorithm considers the maze solved when it has gone from the upper left corner to the lower right corner. 

## Settings

* **Algorithm:** Current available algorithms are Depth First Search, A*, double A*, Dijkstra, and Tremaux.

* **Maze Decimation Probability:** Setting this to zero will ensure the maze has only one solution. Setting this to anything greater than zero will increase the probability of more paths.

* **Save Maze:** Responding yes will save the solved and unsolved maze to the same directory as the executable.

* **Maze Size:** The size of the maze. Large values (>10,000) will take awhile and really large values (>50,000) can use more than 64 GB of memory. Also keep in mind that the solved and unsolved maze image will be saved with dimension n x n.

* **Show Animation:** Whether or not to show the animation of the algorithm solving the maze. It looks cool but takes way longer than just letting it run. Automatically disabled for mazes with size >2048.

* **Animation Scale:** The scale factor that the maze animation will display at. 1 = oneCancel changes pixel per maze cell.

* **Animation Speed:** The speed scale for the maze animation. 1 = n cell traversals per second where n = your monitor's refresh rate.

## A maze generated and solved using Depth First Search with size = 1024:

![alt text](https://github.com/ihawn/MazeCruncher/blob/main/1024.png)
