#include <fstream>
#include <iostream>
#include <utility>
#include <string>
#include <vector>

using grid_t = std::vector<std::vector<bool>>;

unsigned long long int count_trees(grid_t& grid, const size_t right_jump, const size_t down_jump)
{
    auto y = 0u;
    unsigned long long int counter = 0u;

    for (auto i = 0u; i < grid.size(); i += down_jump)
    {
        if (grid[i][y])
            counter++;
        y = (y + right_jump) % grid[i].size();
    }

    return counter;
}

int main()
{
    using std::string;

    std::ifstream file;
    file.open("input_3_1.txt");
    if (!file.is_open())
    {
        std::cout << "Reading went wrong" << std::endl;
        file.close();
        return 1;
    }

    size_t length = 0;
    auto grid = grid_t();

    size_t x = 0;
    for (string line; std::getline(file, line); )
    {
        if (length <= 0)
            length = line.size();

        grid.emplace_back(length, false);

        for (auto j = 0u; j < length; j++)
            grid[x][j] = line[j] == '#';

        x++;
    }

    size_t y = 0;

    auto counter = count_trees(grid, 1, 1);
    counter *= count_trees(grid, 3, 1);
    counter *= count_trees(grid, 5, 1);
    counter *= count_trees(grid, 7, 1);
    counter *= count_trees(grid, 1, 2);

    std::cout << counter << std::endl;
}
