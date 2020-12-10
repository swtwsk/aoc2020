#include <array>
#include <fstream>
#include <iostream>
#include <iterator>
#include <utility>
#include <vector>
#include <optional>

using OptPair = std::optional<std::pair<int, int>>;
using ResultPair = std::pair<OptPair, int>;

OptPair get_pair(const std::vector<int>& numbers, int base)
{
    std::array<bool, 2020> available_numbers;
    available_numbers.fill(false);

    for (auto num : numbers)
    {
        if (2020 - base - num < 0)
            continue;

        if (available_numbers[2020 - base - num])
            return std::make_pair(2020 - base - num, num);

        available_numbers[num] = true;
    }

    return std::nullopt;
}

int main()
{
    std::ifstream file;
    file.open("input_1_1.txt");
    if (!file.is_open())
    {
        std::cout << "Reading went wrong" << std::endl;
        file.close();
        return 1;
    }

    const std::vector<int> numbers{ std::istream_iterator<int>{file}, {} };
    file.close();

    std::vector<ResultPair> optional_pairs(numbers.size());
    std::transform(numbers.begin(), numbers.end(), std::back_inserter(optional_pairs),
        [&numbers](auto num) -> ResultPair { return std::make_pair(get_pair(numbers, num), num); });

    for (auto results : optional_pairs)
    {
        OptPair pair_optional;
        int third_int;
        std::tie(pair_optional, third_int) = results;
        if (!pair_optional.has_value())
            continue;

        const auto pair = pair_optional.value();
        std::cout << pair.first << " " << pair.second << " " << third_int << std::endl;
        std::cout << pair.first * pair.second * third_int << std::endl;
        return 0;
    }

    std::cout << "Couldn't find numbers" << std::endl;
    return 1;
}
