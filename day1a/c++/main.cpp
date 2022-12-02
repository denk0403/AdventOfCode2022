#include <iostream>
#include <fstream>

const auto FILE_NAME = "../input.txt";

int main()
{
    std::ifstream input_file(FILE_NAME);

    if (input_file.is_open())
    {
        auto running_total = 0;
        auto max_so_far = 0;

        std::string line;
        while (std::getline(input_file, line))
        {
            if (line.empty())
            {
                if (running_total > max_so_far)
                {
                    max_so_far = running_total;
                }
                running_total = 0;
                continue;
            }

            auto number = std::stoi(line);
            running_total += number;
        }
        input_file.close();

        printf("%d\n", max_so_far);
    }
}