# Greedy Algorithms
The idea of a greedy is to not consider all choices, we just pick the best. Then we of course need to prove that picking this locally best choice will result in the globally optimal solution.

**Greedy-choice property**
- The challenge is to choose the right interpretation of "the best choice". Need to brainstorm what is a good greedy choice. Fx activity that starts first is a problem because if can last all day. Also cant pick shortest activity. Because it can overlap badly. Another choice could be the number of activities they an activity overlaps.
---
## Activity-Selection Problem
- Input:
A set of n activities, each with start and end(Finish) times A[i].s and A[i].f

- Output:
The largest subset of mutually compatible activities

## How to solve? (Using dynamic programming)
The straightforward solution is to pick an activity and then check the compatability of activities before and after the activity. This will result in a dynamic programming solution.

### How to prove that they are correct
 - We have to prove the optimal substructure
   -  If an optimal solution $A$ to $S_ij$ includes $A[k]$, then it also includes optimal solutions to $S_ik$ and $S_kj$
   - For example, if you pick the optimal local choice, then the sub-structure will have to include the optimal solution. The problems are independent.

### Time and Space complexity
Solving this problem with dynamic programming results requires $n^2$ space becaues the table i*j which are both n length.
The runtime is at least $\Omega(n^2)$ because it has to fill out the whole table. But it is actually $\Theta(n^3)$ because you first fill the table, and then check each element in the table.

## How to solve? (The greedy approach)
Idea: Choose the activity that **finishes first**. This leaves as much time as possible for other activities. Then, solve only one sub-problem for the remaining compatible activities.

### Time and Space complexity
- Space = 1
- Complexity = n
---
## Huffman code
First build a frequency table. How many times does each character occur? The idea is to use a small amount of bits for the most frequent characters.

**Prefix code**: no codeword is a prefix of another. It can be shown that optimal coding can be done with prefix code. We want to use prefix code because then we can decode the codes without ambiguity.

We can store all codewords in a *binary trie* ( a binary tree but when used with strings its called trie). Easy to decode because you just go down the left or right node every time you read a 1 or 0 untill you have no leaf nodes and then you have the decoded the character from the bit string.