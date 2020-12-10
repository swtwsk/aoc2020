#include <fstream>
#include <iostream>
#include <utility>
#include <string>
#include <vector>
#include <map>
#include <algorithm>
#include <cctype>
#include <functional>
#include <array>

bool validate_number(const std::string& number_string, const std::string::size_type number_length)
{
    if (number_string.size() != number_length)
        return false;
    if (!std::all_of(number_string.begin(), number_string.end(), std::isdigit))
        return false;

    return true;
}

bool validate_date(const std::string& date_string, const unsigned int min_year, const unsigned int max_year)
{
    if (!validate_number(date_string, 4))
        return false;

    unsigned int year = 0;
    for (auto digit : date_string)
    {
        year = year * 10 + (digit - '0');
    }

    return min_year <= year && year <= max_year;
}

bool validate_eye_color(const std::string& eye_string)
{
    std::array<std::string, 7> possible_colors = { "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };
    return std::any_of(possible_colors.begin(), possible_colors.end(), [&eye_string](const std::string& color)
    {
        return color == eye_string;
    });
}

bool validate_hair_color(const std::string& color_string)
{
    if (color_string.size() != 7)
        return false;

    if (color_string[0] != '#')
        return false;

    for (auto i = 1u; i < color_string.size(); i++)
    {
        if (std::isdigit(color_string[i]))
            continue;
        if ('a' <= color_string[i] && color_string[i] <= 'f')
            continue;

        return false;
    }

    return true;
}

bool validate_height(const std::string& height_string)
{
    auto height = 0u;
    auto i = 0u;
    for (i = 0u; i < height_string.size(); i++)
    {
        if (!isdigit(height_string[i]))
            break;
        height = height * 10 + (height_string[i] - '0');
    }

    if (i >= height_string.size())
        return false;

    auto suffix = height_string.substr(i);
    if (suffix == "cm")
        return 150 <= height && height <= 193;
    if (suffix == "in")
        return 59 <= height && height <= 76;
    return false;
}

const std::map<std::string, std::pair<unsigned char, std::function<bool(const std::string&)>>> FIELD_MASK_MAP{
    {"byr", std::make_pair(1, [](const std::string& s) { return validate_date(s, 1920, 2002); })},
    {"iyr", std::make_pair(1 << 1, [](const std::string& s) { return validate_date(s, 2010, 2020); })},
    {"eyr", std::make_pair(1 << 2, [](const std::string& s) { return validate_date(s, 2020, 2030); })},
    {"hgt", std::make_pair(1 << 3, [](const std::string& s) { return validate_height(s); })},
    {"hcl", std::make_pair(1 << 4, [](const std::string& s) { return validate_hair_color(s); })},
    {"ecl", std::make_pair(1 << 5, [](const std::string& s) { return validate_eye_color(s); })},
    {"pid", std::make_pair(1 << 6, [](const std::string& s) { return validate_number(s, 9); })},
    {"cid", std::make_pair(1 << 7, [](const std::string& s) { return true; })}
};

bool is_valid(unsigned char fields)
{
    fields |= 128;
    return fields == 255;
}

std::unique_ptr<std::vector<std::string>> split_by_space(std::string& s)
{
    const std::string delim = " ";
    auto vec = std::make_unique<std::vector<std::string>>();

    auto start = 0U;
    auto end = s.find(delim);
    while (end != std::string::npos)
    {
        vec->push_back(s.substr(start, end - start));
        start = end + delim.length();
        end = s.find(delim, start);
    }

    while (std::isspace(s[s.size() - 1]))
        s.erase(s.size() - 1, 1);

    vec->push_back(s.substr(start, end));

    return vec;
}

unsigned char get_field_mask(const std::string& s)
{
    const std::string delim = ":";

    const auto start = 0U;
    const auto end = s.find(delim);
    if (end == std::string::npos)
        throw std::invalid_argument("Input is not in a key:value pattern");

    const auto key = s.substr(start, end - start);
    auto value = s.substr(end + 1);
    while (std::isspace(value[value.size() - 1]))
        value.erase(value.size() - 1, 1);

    const auto mask_it = FIELD_MASK_MAP.find(key);
    if (mask_it == FIELD_MASK_MAP.end())
        throw std::invalid_argument("Invalid key");

    return mask_it->second.second(value) ? mask_it->second.first : static_cast<unsigned char>(0);
}

int main()
{
    using std::string;

    std::ifstream file;
    file.open("input_4.txt");
    if (!file.is_open())
    {
        std::cout << "Reading went wrong" << std::endl;
        file.close();
        return 1;
    }

    unsigned char fields = 0;
    auto counter = 0u;
    for (string line; std::getline(file, line);)
    {
        if (std::all_of(line.begin(), line.end(), std::isspace))
        {
            if (is_valid(fields))
                counter++;
            fields = 0u;
            continue;
        }

        auto split = split_by_space(line);

        for (const auto& l : *split)
        {
            fields |= get_field_mask(l);
        }
    }

    // last line
    if (is_valid(fields))
        counter++;

    std::cout << counter << std::endl;
}
