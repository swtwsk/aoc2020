#include <fstream>
#include <iostream>
#include <string>
#include <array>

unsigned int binary_search(const std::string& line, const char lower_half_marker)
{
    const auto count = line.size();
    unsigned int l = 0u;
    unsigned int r = (2 << (count - 1)) - 1;
    for (auto i = 0u; i < count; i++)
    {
        const auto center = (l + r) / 2;
        if (line[i] == lower_half_marker)
            r = center;
        else
            l = center + 1;
    }

    if (l != r)
        std::cout << "SOMETHING WRONG: " << line << "; l = " << l << ", r = " << r << std::endl;

    return l;
}

int main()
{
    using std::string;

    std::ifstream file;
    file.open("input_5.txt");
    if (!file.is_open())
    {
        std::cout << "Reading went wrong" << std::endl;
        file.close();
        return 1;
    }

    auto seats = std::array<bool, 1024>();
    seats.fill(false);

    for (string line; std::getline(file, line);)
    {
        const auto row = binary_search(line.substr(0, 7), 'F');
        const auto column = binary_search(line.substr(7), 'L');

        const auto seat_id = row * 8 + column;
        seats[seat_id] = true;
    }

    for (auto i = 1u; i < seats.size() - 1; i++)
    {
        if (seats[i - 1] && seats[i + 1] && !seats[i])
            std::cout << i << std::endl;
    }
}
