import { readFileSync } from "node:fs";

const boxArea = (length, width, height) =>
  2 * length * width + 2 * width * height + 2 * height * length;

const boxVolume = (length, width, height) => length * width * height;

// returns area of smallest side of box
const smallestArea = (length, width, height) =>
  Math.min(length * width, width * height, height * length);

// returns perimeter of smallest side of box
const smallestPerimeter = (length, width, height) =>
  Math.min(
    2 * length + 2 * width,
    2 * width + 2 * height,
    2 * height + 2 * length
  );

let wrapperArea = 0;
let ribbonLength = 0;

try {
  const f = readFileSync("input.txt", { encoding: "utf-8" });

  for (let line of f.split("\n")) {
    if (!line.match(/[0-9]+x[0-9]+x[0-9]/)) continue;

    let [length, width, height] = line.split("x").map((i) => Number(i));

    wrapperArea +=
      boxArea(length, width, height) + smallestArea(length, width, height);

    ribbonLength +=
      smallestPerimeter(length, width, height) +
      boxVolume(length, width, height);
  }
} catch (err) {
  console.log(err);
}
console.log("The output for part 1 is: ", wrapperArea, "ft²");
console.log("The output for part 2 is: ", ribbonLength, "ft");
