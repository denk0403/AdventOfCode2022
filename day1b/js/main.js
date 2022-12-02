import { readFileSync } from "fs";

const FILE_PATH = "../input.txt";

const input = readFileSync(FILE_PATH, { encoding: "utf8" });
const lines = input.split(/\r?\n/);

let runningTotal = 0;
const totals = [];
for (const line of lines) {
	if (line === "") {
		totals.push(runningTotal);
		runningTotal = 0;
		continue;
	}

	const number = Number.parseInt(line);
	runningTotal += number;
}

totals.sort((a, b) => b - a);

console.log(totals[0] + totals[1] + totals[2]);
