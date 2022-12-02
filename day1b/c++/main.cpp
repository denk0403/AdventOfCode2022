#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

const auto FILE_NAME = "../input.txt";

int main()
{
    std::ifstream input_file(FILE_NAME);

    if (input_file.is_open())
    {
        auto running_total = 0;
        std::vector<int> totals = {};

        std::string line;
        while (std::getline(input_file, line))
        {
            if (line.empty())
            {
                totals.push_back(running_total);
                running_total = 0;
                continue;
            }

            auto number = std::stoi(line);
            running_total += number;
        }
        input_file.close();

        std::sort(totals.begin(), totals.end(), std::greater<int>());

        printf("%d\n", totals.at(0) + totals.at(1) + totals.at(2));
    }
}