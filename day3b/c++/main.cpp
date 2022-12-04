#include <iostream>
#include <fstream>
#include <set>
#include <vector>
#include <sstream>
#include <algorithm>
#include <iterator>

using namespace std;

const auto FILE_NAME = "../input.txt";

vector<string *> chunk(vector<string> items, int chunk_size)
{
    vector<string *> result;

    auto count = 0;

    for (auto &item : items)
    {
        if (count == chunk_size)
            count = 0;

        if (count == 0)
        {
            auto chunk = new string[chunk_size];
            result.push_back(chunk);
        }

        result.back()[count] = item;
        count += 1;
    }

    return result;
}

int main()
{
    ifstream input_file(FILE_NAME);

    if (!input_file.is_open())
    {
        return 1;
    }

    // read whole file and store lines
    vector<string> lines;
    string line;
    while (getline(input_file, line))
    {
        if (!line.empty())
            lines.push_back(std::move(line));
    }

    auto priority_sum = 0;
    auto chunks = chunk(lines, 3);
    for (auto &chunk : chunks)
    {
        auto in_chunk_1_and_2 = [chunk](char ch)
        { return chunk[1].find(ch) != string::npos &&
                 chunk[2].find(ch) != string::npos; };

        auto ch = find_if(chunk[0].begin(), chunk[0].end(), in_chunk_1_and_2);

        if (*ch >= 'a')
        {
            priority_sum += *ch - 'a' + 1;
        }
        else
        {
            priority_sum += *ch - 'A' + 27;
        }
    }

    input_file.close();

    printf("%d\n", priority_sum);
}
