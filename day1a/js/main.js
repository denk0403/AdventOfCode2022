import { readFileSync } from "fs";

const FILE_PATH = "../input.txt";

const input = readFileSync(FILE_PATH, { encoding: "utf8" });
const lines = input.split(/\r?\n/);

let runningTotal = 0;
let maxSoFar = 0;
for (const line of lines) {
	if (line === "") {
		if (runningTotal > maxSoFar) {
			maxSoFar = runningTotal;
		}
		runningTotal = 0;
		continue;
	}

	const number = Number.parseInt(line);
	runningTotal += number;
}

console.log(maxSoFar);
