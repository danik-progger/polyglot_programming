import fs from "fs";

// ----- ITERATOR -----
const arr: number[] = [1, 2, 3].map((x) => x + 1);
console.log(arr);

// ----- READ FILE ------
fs.readFileSync("src/lines.txt")
    .toString()
    .split("\n")
    .filter((__, i) => i % 2 === 0)
    .filter((__, i) => i > 1 && i < 4)
    .forEach((line) => console.log(line));

// ----- ENUM -----
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

function printColor(color: Color) {
    switch (color) {
        case Color.Red:
            console.log("red");
            break;
        case Color.Green:
            console.log("green");
            break;
        case Color.Blue:
            console.log("blue");
    }
}

printColor(Color.Red);

type Man = {
    age: number;
    name: string;
};

type Item = number | string | Man;

function append(items: Item[]) {
    items.push("hello fem!");
}

const items: Item[] = [];
console.log(items);
append(items);
console.log(items);

const numbers: number[] = [1, 2, 3];
console.log(numbers);
append(numbers);
console.log(numbers);

// ----- OPTIONS -----
function foo(n: number | undefined): number | undefined {
    return n === undefined ? undefined : n * 5;
}

console.log(foo(5));
console.log(foo(undefined));
console.log(foo(0));

function practice(vec: number[], index: number): number {
    return (vec[index] ?? index) * 5;
}

