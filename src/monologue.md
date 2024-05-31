Hello everyone

I've been learning rust recently and I wanted to write an application so that I can practice more, one of the good exercises is to build a game. I choose a relatively simple game which I quite enjoy which is minesweeper, if you don't know the game, I'll leave some links in the description, so please check it out.

I did some research for a good and simple game library and I found one called macroquad which is quite easy to work with, if we check their examples page, they have a lot of different examples and it looks like a good fit.

So let's copy their example code and try it out. I setup an empty rust project here so I'll paste the code and add the library with cargo then run the project, it runs well with a very red background, let's change it to white, and remove the other parts of the example.

Alright, looks good. Let's start by creating a modules for minesweeper and add it in another file, then let's create a struct for the game that will hold the number of rows, columns, the list of tiles of the game and the state of the game.
Let's also create a module for the tile and add the struct there. We declare the game state enum here with 3 states: Playing, Won or Lost.

Now, let's start with the implementation by adding a new function to create a new instance. It receives the number of rows, cols and the number of mines on the board.

We need to create the game tiles vector, so let's use a function that we'll call "create tiles". The definition will take the same params and return a vector of tiles.

We create the vector with a size equal to the the rows times the columns, since that's the total number of tiles in the game, then let's define the "new" function in the tile struct so that we can use it. It needs to derive the "Clone" trait so let's do it, along with "Debug".
Now, we need to randomly generate these mines in the board, so let's use "rand" package, we add it first with cargo. Then we loop over the mines count and get a random number in the tiles, if that tile already has a mine, we choose another mine. We need to add the "has mine" property to the tile struct. Finally, we set this tile to have a mine, and we return the tiles vector.

Now, let's go back to the main file and create a new game, let's say with 10 times 10 with 10 mines.

And now for the visual part, let's draw the game, we can start with the draw function, but first, let's define the tile state which is gonna be: hidden, revealed or flagged.

Then in the minesweeper struct, we add the draw function, which takes a reference to self. Let's define a tile size of 50 to start with, then we loop over the columns and rows of the game, we need to find the index of this i and j indices in the tiles vector, so we create a function we call it "get index". The index is gonna be the number of cols times how many columns we have, which is j, and we add i to it. Finally, we fix the mismatching types we have. We define a ref to the tile in hand and we call a "draw" function on it too, this function will take the x and y of where to draw, and the tile size. In the tile struct, the we define the draw function and we use a simple "draw rectangle" from macroquad. It takes the x and y for position, then w and h for size, and the color, we'll use sky blue.

Now let's run it, all is blue cause they are stuck together, let's add some padding .... much better. Let's also add some margins at the top and left so that it's not too stuck to the side. Another cargo run, and we have our margins.

Next, let's look at the tile size, if we resize the window, we see that the size is the same and can be hidden outside of the visible window, let's make it dynamic based on the width and height of the window. We can use the "screen width" and "screen height" functions from macroquad. The width will be the screen width minus the margins (let's define the right and bottom margins too then) divided by the columns, similar for the height. Then we take the minimum of them, if we try again .... it becomes a lot more responsive.

Now that we have the tiles drawn correctly, we need to change the color depending on the state. So let's match the state of the tile and change the color accordingly, let's use gray for the revealed ones. For testing, I'll change the state to revealed and cargo run it.

For the mine and flag, we need some textures, so I've prepared 2 textures here to use. The documentation of macroquad has some functions to draw textures ..... and we need to load the textures first.

We don't want to load the textures every single time for every tile, it's better to create one instance and use it all the time, I'll do that in a separate module that will be called textures. The "load" function will load the 2 textures and keep them. The "load texture" function is asynchronous so we make the "load" function async too, and we create an instance of the textures in the minesweeper module for the game. Last thing, we add some proper error messages if we fail to load the texture.

Now, let's go back to tile module and try them out. We match again on the state, and whether the tile has a mine or not. Since we have cases that doesn't have any texture, we can use an option for the variable, and if it has anything, we draw the texture. Let's give it a try. It's too big, we need to set the size. In the definition, it uses another "load texture ex" function that has some params with the size, looks like a better fit. Much better, we can see all the mines, let's try out the flags .... looks good.


The next part is handling mouse input, when the user clicks on the tile. Let's check macroquad docs for the mouse functions. We can create another module that we'll call mouse to check if the mouse is pressed and to get the position of the mouse. Since we're gonna work with "position" of the mouse and tiles a lot, let's create a "position" module that has the x and y values. We can make it a generic struct. If the mouse is pressed, return the mouse position as an option. Otherwise, return None.

Now that the function is ready, let's call "handle input" on the game and create this function. We have 2 cases, one for left mouse click, and the other for right. In case of left click, we make a move, and in case of right click, we flag the tile.

Alright, let's start with making a move at the position. The position we got is the pixel position, we need to map it to which tile the user clicked on, a function called "resolve tile position" should do that. It will return an option of the tile position, cause the user might click outside the tiles.

First, let's get the tile size, then remove the padding that we added, we can use a "sub" function on the position to subtract the left and top margins. The "sub" function needs to have a "where" clause with "Sub" to make sure we can perform the subtraction. And we can generate the "add" function the same way. After removing the padding, we need to divide by the tile size to scale the position down to the tile position, we can create the "div" function in a similar way. Last thing is to check whether this position is within the bounds of the grip or not. The types mismatch here cause one is "f 32" and the other is "i 32", so we can use rust "into" trait to handle the conversion, I'll need a google search cause I cannot remember what the syntax for it is. Alright, let's check if we have a position, if we don't, we exit early.

Now it's time to get the tile position and the tile, we'll need to update the "get index" function first to accept a position instead of an "i" and "j".
Once we have the tile, we can "reveal" it when the user clicks on it. the tile ref needs to be mutable, so does the game ref. We need to add the "Copy" where clause too. Now, let's give it a try. It works well, clicking outside doesn't break it, or it does, a bit. Ah, we need to check if the bounds are below zero since we subtract the margins. Now it looks better.

For the "flag" part, it will work the same way. Alright, it works.

The next part is handling the logic of the game when the user clicks or flags a tile. Let's start with clicking a tile.
