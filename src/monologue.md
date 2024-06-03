Hello everyone,

I've been learning Rust recently and wanted to write an application to practice more. One of the good exercises is to build a game. I chose a relatively simple game I quite enjoy: Minesweeper. If you're unfamiliar with the game, I'll leave some links in the description, so please check it out.

After some research for a good and simple game library, I found one called Macroquad, which is quite easy to work with. If we check their examples page, they have a lot of different examples, and it looks like a good fit.

Let's copy their example code and try it out. I set up an empty Rust project here, so I'll paste the code, add the library with Cargo, and then run the project. It runs well with a very red background. Let's change it to white and remove the other parts of the example.

Alright, looks good. Let's start by creating modules for Minesweeper and add them to another file. Then let's create a struct for the game that will hold the number of rows, columns, the list of tiles, and the state of the game. Let's also create a module for the tile and add the struct there. We declare the game state enum with three states: Playing, Won, or Lost.

Now, let's start with the implementation by adding a new function to create a new instance. It receives the number of rows, columns, and the number of mines on the board.

We need to create the game tiles vector, so let's use a function that we'll call "create tiles." The definition will take the same params and return a vector of tiles.

We create the vector with a size equal to the rows times the columns since that's the total number of tiles in the game. Then, let's define the "new" function in the tile struct so that we can use it. It needs to derive the "Clone" trait, so let's do it, along with "Debug." Now, we need to randomly generate these mines on the board. Let's use the "rand" package, add it with Cargo, and then loop over the mines count to get a random number in the tiles. If that tile already has a mine, we choose another mine. We need to add the "has_mine" property to the tile struct. Finally, we set this tile to have a mine and return the tiles vector.

Now, let's go back to the main file and create a new game, say with 10x10 tiles with 10 mines.

For the visual part, let's draw the game. We can start with the draw function, but first, let's define the tile state: hidden, revealed, or flagged.

In the Minesweeper struct, we add the draw function, which takes a reference to self. Let's define a tile size of 50 to start with, then loop over the columns and rows of the game. We need to find the index of these i and j indices in the tiles vector, so we create a function called "get_index." The index will be the number of cols times how many columns we have, which is j, plus i. Finally, we fix the mismatching types. We create a ref to the tile in hand and call a "draw" function on it. This function will take the x and y coordinates of where to draw and the tile size. In the tile struct, we define the draw function and use a simple "draw_rectangle" from Macroquad. It takes the x and y positions, then w and h for size, and the color. We'll use sky blue.

Now let's run it. All is blue because they are stuck together. Let's add some padding... much better. Let's also add some margins at the top and left so that it's not too stuck to the side. Another Cargo run, and we have our margins.

Next, let's look at the tile size. If we resize the window, we see that the size is the same and can be hidden outside of the visible window. Let's make it dynamic based on the width and height of the window. We can use the "screen_width" and "screen_height" functions from Macroquad. The width will be the screen width minus the margins (let's define the right and bottom margins too), divided by the columns, similarly for the height. Then we take the minimum of them. If we try again... it becomes a lot more responsive.

Now that we have the tiles drawn correctly, we need to change the color depending on the state. So let's match the state of the tile and change the color accordingly. Let's use gray for the revealed ones. For testing, I'll change the state to revealed and cargo run it.

For the mine and flag, we need some textures, so I've prepared two textures here to use. The documentation of Macroquad has some functions to draw textures, and we need to load them first.

We don't want to load the textures every single time for every tile. It's better to create one instance and use it all the time. I'll do that in a separate module called "textures." The "load" function will load the two textures and keep them. The "load_texture" function is asynchronous, so we make the "load" function async too. We create an instance of the textures in the Minesweeper module for the game. Finally, we add proper error messages if we fail to load the texture.

Now, let's go back to the tile module and try them out. We match again on the state and whether the tile has a mine or not. Since we have cases that don't have any texture, we can use an option for the variable, and if it has anything, we draw the texture. Let's give it a try. It's too big; we need to set the size. In the definition, we can use another "load_texture_ex" function that has some params with the size. Looks like a better fit. Much better, we can see all the mines. Let's try out the flags... looks good.


The next part is handling mouse input when the user clicks on a tile. Let's check the Macroquad docs for the mouse functions. We can create another module called "mouse" to check if the mouse is pressed and to get its position. Since we're going to work with the position of the mouse and tiles a lot, let's create a "position" module with x and y values. We can make it a generic struct. If the mouse is pressed, return the mouse position as an option. Otherwise, return None.

Now that the function is ready, let's call "handle_input" on the game and create this function. We have two cases: one for left mouse click and the other for right. In the case of left click, we make a move, and in case of right click, we flag the tile.

Alright, let's start with making a move at the position. The position we got is the pixel position; we need to map it to which tile the user clicked on. A function called "resolve_tile_position" should do that. It will return an option of the tile position because the user might click outside the tiles.

First, let's get the tile size, then remove the padding that we added. We can use a "sub" function on the position to subtract the left and top margins. The "sub" function needs to have a "where" clause with "Sub" to make sure we can perform the subtraction. We can generate the "add" function the same way. After removing the padding, we need to divide by the tile size to scale the position down to tile position. We can create the "div" function in a similar way. Lastly, we check whether this position is within the bounds of the grid or not. The types mismatch here because one is "f32" and the other is "i32," so we can use Rust's "into" trait to handle the conversion. I'll need a quick Google search because I can't remember the syntax for it. Alright, let's check if we have a position. If we don't, we exit early.

Now it's time to get the tile position and the tile. We'll need to update the "get_index" function first to accept a position instead of "i" and "j".
Once we have the tile, we can "reveal" it when the user clicks on it. The tile ref needs to be mutable, so does the game ref. We need to add the "Copy" where clause too. Now, let's give it a try. It works well; clicking outside doesn't break it, or it does, a bit. Ah, we need to check if the bounds are below zero since we subtract the margins. Now it looks better.

For the "flag" part, it will work the same way. Alright, it works.

The next part is handling the game logic when the user clicks or flags a tile. Let's start with clicking a tile.

We need to match on the state of the tile the user clicked on. The first easy case is if the tile is hidden and has a mine. We reveal this tile, and it's game over for the user.

The other case is if it's hidden but doesn't have a mine. In this case, the tile should be revealed along with the neighboring tiles in all directions until reaching squares that contain numbers. We'll come back to this function.

The last case is if the tile is revealed, which is what's called chording. Supposedly, this is when you click both mouse buttons, but for simplicity, let's do it with a left mouse click. This should also reveal the neighboring tiles if they don't have any numbers. Let's call the same function for now, but we'll come back to that.

The "reveal_neighbor_tiles" function will use a breadth-first search algorithm (BFS) to reveal the tiles. For this, we need a queue to store the current tile positions. We start with the tile that the user clicked, then keep iterating while there are items in the queue. The neighboring items will all differ by -1, 0, or 1. Let's define these values in a constant. For each neighbor, we get the new position by adding the current position to the neighbor difference. We do some out-of-bounds checks. We only need to consider the tile if it's still hidden and doesn't have a mine. Finally, we reveal it. If the tile doesn't have any mines around it, we need to do the same process again and reveal more tiles until we hit tiles with numbers. This "number_of_mines_around" variable is missing, so first, we define it and calculate it. The calculation can be done once at the beginning since it won't change during the game. As usual, we loop through the tiles and calculate the number of tiles for each of them using a helper method. The helper method will loop through the neighboring tiles, check that they are inside the bounds and that they have mines, then count them.

We go back to our BFS algorithm, which is actually "done" now. Let's try it out. Clicking one tile opens the neighboring tiles until it hits a number. We haven't drawn the numbers yet, so we need to attend to that, but from the times I played before, I know this tile should be a mine, and this one too.

For writing the number of mines, we can do that in the tile class. We start by checking that the tile has a number and that it's revealed. Then we grab the "draw_text" function from Macroquad. The function takes the text, x, y for position, font size, and color. We have the x and y coordinates and the tile size as the font size. Let's try it. Good, but it needs to be shifted a bit. Let's add half the size and try again. Seems like we need to subtract a little bit from the x and add a little bit to the y. Much better.

Going back to the board, we can click on a tile, open the neighboring tiles, and flag tiles too.

We need to revise the logic when the user clicks on a revealed tile, but let's add a function first to determine if the user has won. We can do that by checking that all the tiles that don't have mines have been revealed. Let's give it a try. We can lower the number of mines to win quickly. We still handle clicks even after the game is over, so let's add a guard for it. Now, let's try a bigger board. This is a bug. Clicking on the "3" shouldn't have opened the neighboring tiles. This takes us back to the case we forgot when the user clicks a revealed tile. We immediately reveal the neighboring tiles, but first, we must check if the user has flagged all the mines. The "can_reveal_neighbor_tiles" function will have similar logic. We get the index and the tile. If this tile has no mines around it, then it's already been revealed. If it has a mine, then the game is lost anyway.

Then, we need to check that the number of tiles the user flagged correctly equals the number of mines around this tile. Let's match over the "has_mine" and "state" fields. If it has a mine and it's flagged, that's a plus one. If it doesn't have a mine and it's flagged incorrectly, we increment by a large number to make it always false. Otherwise, we return 0. Finally, we check that the count equals the number of mines around. Let's give it a try, it works correctly, clicking on a tile that's not cleared, doesn't open the neighboring tiles, and if I un-flag .... seems like unflagging doesn't work, we can fix it by going to the flag function and changing it to "toggle flag" instead.

Here is another bug, you can flag a revealed tile, we need to add a guard for it in the "flag" function. Let's give it another run, all good now.

So far, we've created the game but it's missing a lot of things, there are a few bugs, some missing logic, and we do need to refactor the code a little bit. We also need to add controls to the game, allow the user to restart, give a visual of the time and remaining mines. If you want to see me do this, give this video a like and subscribe
