
# Solver for Logic Puzzle

This little program solves a class of logic puzzles in which each letter represents a number.
There is a list of words given including the sum of the values of each number assigned to a letter.
Each number is only exactly assigned to one letter.

The goal is to find the word of which only the values of its letters are known.

## Usage

To solve a puzzle put it in a text file and solve it like this: 

    cryptopuzzle < input.txt

The solution is printed to standard output.

## Input format

The first line is the numbers of the solution separated by spaces.

The following lines are a word and the corresponding sum of its letters separated by a space.

Don't forget to add an empty line at the end to signal the program that all information was provided.

