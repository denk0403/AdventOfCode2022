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
	X: ROCK,
	Y: PAPER,
	Z: SCISSORS,
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

function getRoundResult(opShape, meShape) {
	if (opShape === meShape) {
		return DRAW;
	}

	if (
		(opShape === ROCK && meShape === SCISSORS) ||
		(opShape === PAPER && meShape === ROCK) ||
		(opShape === SCISSORS && meShape === PAPER)
	) {
		return LOSE;
	}

	return WIN;
}

const input = readFileSync(FILE_PATH, { encoding: "utf8" });
const lines = input.split(/\r?\n/);

let myScore = 0;
for (const line of lines) {
	if (line.trim()) {
		const [opKey, _, meKey] = line;

		const opShape = KEY_2_SHAPE[opKey],
			meShape = KEY_2_SHAPE[meKey];

		const result = getRoundResult(opShape, meShape);

		myScore += RESULT_2_SCORE[result] + SHAPE_2_SCORE[meShape];
	}
}

console.log(myScore);
