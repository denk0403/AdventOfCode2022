// @ts-check
import { readFileSync } from "fs";

const FILE_PATH = "../input.txt";

const ROCK = "rock",
	PAPER = "paper",
	SCISSORS = "scissors";

const KEY_2_SHAPE = {
	A: ROCK,
	B: PAPER,
	C: SCISSORS,
};

const SHAPE_2_SCORE = {
	[ROCK]: 1,
	[PAPER]: 2,
	[SCISSORS]: 3,
};

const WIN = "win",
	LOSE = "lose",
	DRAW = "draw";

const RESULT_2_SCORE = {
	[WIN]: 6,
	[LOSE]: 0,
	[DRAW]: 3,
};

const KEY_2_RESULT = {
	X: LOSE,
	Y: DRAW,
	Z: WIN,
};

/**
 * @param {ROCK | PAPER | SCISSORS} opShape
 * @param {WIN | LOSE | DRAW} result
 */
function getShapeForResult(opShape, result) {
	if (result === DRAW) {
		return opShape;
	}

	if (result === LOSE) {
		switch (opShape) {
			case ROCK:
				return SCISSORS;
			case PAPER:
				return ROCK;
			case SCISSORS:
				return PAPER;
		}
	}

	switch (opShape) {
		case ROCK:
			return PAPER;
		case PAPER:
			return SCISSORS;
		case SCISSORS:
			return ROCK;
	}
}

const input = readFileSync(FILE_PATH, { encoding: "utf8" });
const lines = input.split(/\r?\n/);

let my_score = 0;
for (const line of lines) {
	if (line.trim()) {
		const [opKey, _, resultKey] = line;

		const opShape = KEY_2_SHAPE[opKey],
			result = KEY_2_RESULT[resultKey];

		const meShape = getShapeForResult(opShape, result);

		my_score += RESULT_2_SCORE[result] + SHAPE_2_SCORE[meShape];
	}
}

console.log(my_score);
