# Target Locator using Min-heap Priority Queue
Creates an array-based min heap. Three key functionalities are implemented:
- Enqueue 
- Dequeue
- Peek (Retrieving the root element without removing it, view of the highest-priority item)

## Target Locator
Integrates the priority queue into a practical application: a target locator for identifying optimal combat targets in a simulated battle scenario. 

### Distance Calculation
A function to compute the orthogonal distance between two points, essential for determining proximity in a grid-based layout.
### Target Locator Algorithm
takes the locations of allies and enemies and identifies the nearest enemy for each ally. It ensures each ally targets the closest enemy that is not already targeted by a closer ally
