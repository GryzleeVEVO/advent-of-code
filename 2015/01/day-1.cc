// Part 1:
//  - Start is floor 0
//  - ( means go up 1 floor
//  - ) means go down 1 floor
//  - Objective: get the floor number
//
// Part 2: Get first character that leads to basement (-1)
//  - Index starts at 1

#include <fstream>
#include <iostream>

using namespace std;

int main() {
    ifstream f("input.txt");

    if (!f.is_open()) {
        cerr << "Couldn't open input" << endl;
        return EXIT_FAILURE;
    }

    int dest_floor = 0;
    unsigned int floor_counter = 0;
    unsigned int basement_char = 0;
    char c = {};

    while (!f.eof()) {
        f.get(c);

        switch (c) {
        case '(':
            dest_floor++;
            break;
        case ')':
            dest_floor--;
            break;
        default:
            continue;
        }

        floor_counter++;

        // NOTE: stream ptr will already be at next char but since it
        // starts indexing with 0 it can do
        if (!basement_char && dest_floor <= -1) basement_char = floor_counter;

        // Should be 232
        cout << "Output for part 1: " << dest_floor << endl;

        // Should be 1783
        cout << "Output for part 2: " << basement_char << endl;
    }

    f.close();

    return EXIT_SUCCESS;
}