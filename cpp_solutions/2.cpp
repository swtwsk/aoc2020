#include <fstream>
#include <iostream>
#include <utility>
#include <string>
#include <cctype>
#include <numeric>

unsigned int getUInt(std::string& line)
{
    size_t i = 0;
    for (; i < line.length(); i++)
    {
        if (!std::isdigit(line[i]))
            break;
    }
    auto left_str = line.substr(0, i);
    auto left = std::stoul(left_str);
    line = line.substr(i, line.size() - i);
    return left;
}

int main()
{
    using std::string;

    std::ifstream file;
    file.open("input_2_1.txt");
    if (!file.is_open())
    {
        std::cout << "Reading went wrong" << std::endl;
        file.close();
        return 1;
    }

    auto line_counter = 0u;

    string line;
    for (string line; std::getline(file, line); )
    {
        auto left = getUInt(line);
        line = line.substr(1, line.size() - 1); // remove '-'
        auto right = getUInt(line);
        line = line.substr(1, line.size() - 1); // remove ' '
        auto c = line[0];
        line = line.substr(3, line.size() - 3); // remove 'a: '

        //unsigned int char_counter = std::accumulate(line.begin(), line.end(), 0, [c](int acc, auto el) {return acc + (el == c ? 1 : 0); });

        auto leftValid = line[left - 1] == c && line[right - 1] != c;
        auto rightValid = line[left - 1] != c && line[right - 1] == c;

        line_counter += leftValid || rightValid ? 1 : 0;
    }

    std::cout << line_counter << std::endl;
}
