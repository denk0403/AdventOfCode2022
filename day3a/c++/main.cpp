#include <iostream>
#include <fstream>
#include <set>

const auto FILE_NAME = "../input.txt";

int main()
{
    std::ifstream input_file(FILE_NAME);

    if (input_file.is_open())
    {
        auto priority_sum = 0;

        std::string line;
        while (std::getline(input_file, line))
        {
            if (!line.empty())
            {

                auto line_len = line.length();
                auto half_len = line_len / 2;
                auto first_half = line.substr(0, half_len);
                auto second_half = line.substr(half_len);

                std::set<char> seen_chars;
                for (auto &ch : first_half)
                {
                    seen_chars.insert(ch);
                }

                // Traverse the string
                for (auto &ch : second_half)
                {
                    if (seen_chars.contains(ch))
                    {
                        // is uppercase
                        if (ch >= 'a')
                        {
                            priority_sum += ch - 'a' + 1;
                        }
                        else
                        {
                            priority_sum += ch - 'A' + 27;
                        }
                        break;
                    }
                }
            }
        }
        input_file.close();

        printf("%d\n", priority_sum);
    }
}
