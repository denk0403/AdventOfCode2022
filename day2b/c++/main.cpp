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

Result key2result(char key)
{
    switch (key)
    {
    case 'X':
        return Result::Lose;
    case 'Y':
        return Result::Draw;
    case 'Z':
        return Result::Win;
    }

    throw std::invalid_argument("received invalid key");
}

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

Shape get_shape_for_result(Shape op_shape, Result result)
{
    switch (result)
    {
    case Result::Draw:
        return op_shape;
    case Result::Lose:
    {
        switch (op_shape)
        {
        case Shape::Rock:
            return Shape::Scissors;
        case Shape::Paper:
            return Shape::Rock;
        case Shape::Scissors:
            return Shape::Paper;
        }
    }
    case Result::Win:
    {
        switch (op_shape)
        {
        case Shape::Rock:
            return Shape::Paper;
        case Shape::Paper:
            return Shape::Scissors;
        case Shape::Scissors:
            return Shape::Rock;
        }
    }
    }
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
                auto result_key = line_chars[2];

                auto op_shape = key2shape(op_key);
                auto result = key2result(result_key);

                auto me_shape = get_shape_for_result(op_shape, result);

                my_score += result2score(result) + shape2score(me_shape);
            }
        }
        input_file.close();

        printf("%d\n", my_score);
    }
}