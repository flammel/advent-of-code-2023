import { assertEquals } from "https://deno.land/std@0.208.0/assert/mod.ts";

async function getInput(): Promise<string> {
  const decoder = new TextDecoder();
  const parts = [];
  for await (const chunk of Deno.stdin.readable) {
    parts.push(decoder.decode(chunk));
  }
  return parts.join();
}

function getResult(input: string): number {
  return input
    .split("\n")
    .map((line) => {
      const winning = new Set(
        line
          .split(" | ")
          .at(0)
          ?.split(":")
          .at(1)
          ?.trim()
          .split(" ")
          .filter((n) => n !== "")
          .map((n) => parseInt(n, 10))
      );
      const have =
        line
          .split(" | ")
          .at(1)
          ?.trim()
          .split(" ")
          .filter((n) => n !== "" && winning.has(parseInt(n, 10))).length ?? 0;

      return have === 0 ? 0 : Math.pow(2, have - 1);
    })
    .reduce((acc, curr) => acc + curr, 0);
}

if (import.meta.main) {
  const input = await getInput();
  console.log(getResult(input));
}

Deno.test("day 04 part 01", () => {
  const testInput = `
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`;
  assertEquals(getResult(testInput), 13);
});
