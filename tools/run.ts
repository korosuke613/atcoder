import { getContestProperty } from "./lib.ts";

if (Deno.args.length !== 1) {
  console.info("INFO: invalid arguments, URL is required.");
  Deno.exit(1);
}

const url = new URL(Deno.args[0]);
const contestProperty = getContestProperty(url);
console.log(`${contestProperty.contestName}_${contestProperty.taskName}`);
