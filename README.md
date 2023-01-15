# solver-1010

Solver for the [1010!](https://gram.gs/game-detail-1010-color.html) game 🦀.

## Usage

Boards are represented as 128-bit integers (only the left-most 100 bits are used), where 1 means the square is filled and 0 empty.  
The binary representation is listed row by row.

Example:

```text
board: 1101110011110010101110110000110110000111110011000011001110100011101000001101000000110100000011000000
X X   X X X     X X 
X X     X   X   X X 
X   X X         X X 
  X X         X X X 
X X     X X         
X X     X X X   X   
    X X X   X       
    X X   X         
    X X   X         
    X X             
```

You can input the boards either in binary or in decimal representation.

You can input any number of pieces to place on the board, as a space separated list of names:

```text
 d: X    
d2: XX   
d3: XXX  
d4: XXXX 
d5: XXXXX
```

```text
b2: X  b4: X  b5: X
    X      X      X
b3: X      X      X
    X      X      X
    X             X
```

```text
tn: XXX  te: XXX  ts:   X  tw: X  
    X          X        X      X  
    X          X      XXX      XXX
```

```text
ss: XX  sb: XXX
    XX      XXX
            XXX
```

After entering the pieces, the best placement will be computed and every piece placement will be displayed as well as the final board.

### Demonstration

```text
Enter starting board: 1101110011110010101110110000110110000111110011000011001110100011101000001101000000110100000011000000
Starting board:
  0 1 2 3 4 5 6 7 8 9
0 X X   X X X     X X 
1 X X     X   X   X X 
2 X   X X         X X 
3   X X         X X X 
4 X X     X X         
5 X X     X X X   X   
6     X X X   X       
7     X X   X         
8     X X   X         
9     X X             
├╴ score: 1980
╰╴ grid: 1094082328658686045277596631232

Enter pieces: dw ss de
Computing best solution...
Piece number 1:
  0 1 2 3 4 5 6 7 8 9
0 X X   X X X O O X X 
1 X X     X   X O X X 
2 X   X X         X X 
3   X X         X X X 
4 X X     X X         
5 X X     X X X   X   
6     X X X   X       
7     X X   X         
8     X X   X         
9     X X             
├╴ score: 1930
╰╴ grid: 1108942444833389067093084946624
Piece number 2:
  0 1 2 3 4 5 6 7 8 9
0 X X   X X X X X X X 
1 X X     X   X X X X 
2 X   X X         X X 
3   X X         X X X 
4 X X O O X X         
5 X X O O X X X   X   
6     X X X   X       
7     X X   X         
8     X X   X         
9     X X             
├╴ score: 1818
╰╴ grid: 1108942444833605450981431263424
Piece number 3:
  0 1 2 3 4 5 6 7 8 9
0 X X O X X X X X X X 
1 X X O O X   X X X X 
2 X   X X         X X 
3   X X         X X X 
4 X X X X X X         
5 X X X X X X X   X   
6     X X X   X       
7     X X   X         
8     X X   X         
9     X X             
├╴ score: 2757
╰╴ grid: 1267630883619500134970062749888
Result:
  0 1 2 3 4 5 6 7 8 9
0                     
1 X X   X X   X X X X 
2 X     X         X X 
3   X           X X X 
4 X X   X X X         
5 X X   X X X X   X   
6       X X   X       
7       X   X         
8       X   X         
9       X             
├╴ score: 2757
╰╴ grid: 1063329662199780732435513408
```
