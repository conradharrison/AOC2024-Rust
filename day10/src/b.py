import sys
import re

### boolean solve(Node n) {
###     put node n on the stack;
###     while the stack is not empty {
###         if the node at the top of the stack is a leaf {
###             if it is a goal node, return true
###             else pop it off the stack
###         }
###         else {
###             if the node at the top of the stack has untried children
###                 push the next untried child onto the stack
###             else pop the node off the stack
### 
###     }
###     return false
### }
class Node:
    def __init__ (self, x, y, value):
        self.x = x
        self.y = y
        self.value = value

    def __str__(self):
        return f"{self.x} {self.y} {self.value}"

class Board:
    def __init__ (self, width, height):
        self.width = width
        self.height = height
        self.grid = {}
        self.clear()

    def __str__(self):
        s = ""
        for y in range(self.height):
            for x in range(self.width):
                s += f"{self.grid[(x,y)]:2} " 
            s += "\n"
        return s

    def clear(self):
        for x in range(self.width):
            for y in range(self.height):
                self.grid[(x,y)] = 0

    def clear_visited(self):
        for x in range(self.width):
            for y in range(self.height):
                if self.grid[(x,y)] == 1:
                    self.grid[(x,y)] = 0

def is_goal(n):
    return n.value == 9

def is_visited(loc, board):
    return board.grid[loc] > 0 

def is_wall(n, loc):
    global initial
    return initial[loc].value - n.value != 1

def is_deadend(n, board):

    if n.x>0:
        if not is_visited((n.x-1, n.y), board) and not is_wall(n, (n.x-1,n.y)): return False
    if n.y>0:
        if not is_visited((n.x,n.y-1), board) and not is_wall(n, (n.x,n.y-1)): return False
    if n.x<board.width-1:
        if not is_visited((n.x+1,n.y), board) and not is_wall(n, (n.x+1,n.y)): return False
    if n.y<board.height-1:
        if not is_visited((n.x,n.y+1), board) and not is_wall(n, (n.x,n.y+1)): return False

    return True

def first_untried_child(n, board):
    
    #print(f"Getting children of {n}")
    
    if n.x>0:
        if board.grid[(n.x-1, n.y)]==0 and not is_wall(n, (n.x-1, n.y)): return initial[(n.x-1, n.y)]
    if n.y>0:
        if board.grid[(n.x,n.y-1)]==0 and not is_wall(n, (n.x,n.y-1)): return initial[(n.x,n.y-1)]
    if n.x<board.width-1:
        if board.grid[(n.x+1,n.y)]==0 and not is_wall(n, (n.x+1,n.y)): return initial[(n.x+1,n.y)]
    if n.y<board.height-1:
        if board.grid[(n.x,n.y+1)]==0 and not is_wall(n, (n.x,n.y+1)): return initial[(n.x,n.y+1)]

    return None

def solve (n):
    global W
    global H
    ends = []
    stack = []
    board = Board(W, H)
    print(f"pushing {n}")
    stack.append(n)
    board.grid[(n.x,n.y)] = 1
    while len(stack) != 0:
        t = stack[-1]
        print(f"Considering {t}")
        if is_goal(t):
            print("GOAL!")
            print(board)
            # Do not return, but record and continue
            ends.append(t)

            ## explore from start
            ### stack = []
            ### #print(f"pushing {n}")
            ### stack.append(n)
            ### board.grid[(n.x,n.y)] = 1

            ## explore from current
            stack.pop()
            board.grid[(t.x,t.y)] += 1
            stack[-1] += 1

            #board.clear_visited()
            #board.grid[(t.x,t.y)] = -1
            print(board)
            
            continue
            #return t
        if is_deadend(t, board):
            print("deadend, popping")
            stack.pop()
            board.grid[(t.x,t.y)] = 1
        else:
            c = first_untried_child(t, board)
            if c != None:
                print(f"pushing {c}")
                stack.append(c)
                board.grid[(c.x,c.y)] = 1
            else:
                print("popping {t}")
                stack.pop()
                board.grid[(t.x,t.y)] = 0
    return ends

# read in maze into initital
initial = {}
#initial[(0,0)] = Node(0,0,0)
#initial[(0,1)] = Node(0,1,1)
#initial[(0,2)] = Node(0,2,0)
#
#initial[(1,0)] = Node(1,0,0)
#initial[(1,1)] = Node(1,1,4)
#initial[(1,2)] = Node(1,2,0)
#
#initial[(2,0)] = Node(2,0,6)
#initial[(2,1)] = Node(2,1,7)
#initial[(2,2)] = Node(2,2,3)

with open(sys.argv[1]) as file:
    lines = [line.rstrip() for line in file]

y = 0
for l in lines:
    x = 0
    for c in l:
        initial[(x,y)] = Node(x, y, int(c))
        x = x + 1
    y = y + 1

W = len(lines[0])
H = len(lines)

s = 0
for y in range(H):
    for x in range(W):
        if initial[(x,y)].value==0 and x==6 and y==4:
            ends = solve(initial[(x,y)])
            s = s + len(ends)
            #print(f"{x},{y}: {len(ends)}")
            #for e in ends:
            #    print(e)

print(s)
