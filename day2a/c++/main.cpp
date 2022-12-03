#include <iostream>
#include <fstream>
#include <stdexcept>

const auto FILE_NAME = "../input.txt";

enum Shape
{
    Rock,
    Paper,
    Scissors,
};

Shape key2shape(char key)
{
    switch (key)
    {
    case 'A':
        return Shape::Rock;
    case 'B':
        return Shape::Paper;
    case 'C':
        return Shape::Scissors;
    case 'X':
        return Shape::Rock;
    case 'Y':
        return Shape::Paper;
    case 'Z':
        return Shape::Scissors;
    }

    throw std::invalid_argument("received invalid key");
}

int shape2score(Shape shape)
{
    switch (shape)
    {
    case Shape::Rock:
        return 1;
    case Shape::Paper:
        return 2;
    case Shape::Scissors:
        return 3;
    }

    throw std::invalid_argument("received invalid shape");
}

enum Result
{
    Win,
    Lose,
    Draw,
};

int result2score(Result result)

{
    switch (result)
    {
    case Result::Win:
        return 6;
    case Result::Lose:
        return 0;
    case Result::Draw:
        return 3;
    }

    throw std::invalid_argument("received invalid result");
}

Result get_round_result(Shape op_shape, Shape me_shape)
{
    if (op_shape == me_shape)
    {
        return Result::Draw;
    }

    if ((op_shape == Shape::Rock && me_shape == Shape::Scissors) || (op_shape == Shape::Paper && me_shape == Shape::Rock) || (op_shape == Shape::Scissors && me_shape == Shape::Paper))
    {
        return Result::Lose;
    }

    return Result::Win;
}

int main()
{
    std::ifstream input_file(FILE_NAME);

    if (input_file.is_open())
    {
        auto my_score = 0;

        std::string line;
        while (std::getline(input_file, line))
        {
            if (!line.empty())
            {
                auto line_chars = line.c_str();
                auto op_key = line_chars[0];
                auto me_key = line_chars[2];

                auto op_shape = key2shape(op_key);
                auto me_shape = key2shape(me_key);

                auto result = get_round_result(op_shape, me_shape);

                my_score += result2score(result) + shape2score(me_shape);
            }
        }
        input_file.close();

        printf("%d\n", my_score);
    }
}