import * as path from "https://deno.land/std/path/mod.ts";
import { exists } from "https://deno.land/std/fs/mod.ts";

const BASE_CONTESTS_DIR = "contests";
const TEMPLATE_FILE_NAME = "problem_template.rs";
const CARGO_TOML_PATH = `${BASE_CONTESTS_DIR}/Cargo.toml`;

export type ContestProperty = {
  contestName: string;
  taskName: string;
};

export const getContestProperty = (url: URL): ContestProperty => {
  if (url.host !== "atcoder.jp") {
    throw new URIError(`invalid host, correct "atcoder.jp": ${url.host}`);
  }

  const re = new RegExp(`\/contests\/(\\S+)\/tasks\/(\\S+)`);

  if (!re.test(url.pathname)) {
    throw new URIError(
      `invalid path, correct "${re.source}": ${url.pathname}`,
    );
  }

  const matchGroups = url.pathname.match(re);
  if (matchGroups === null) {
    throw new URIError("unknown error");
  }

  const contestProperty = {
    contestName: matchGroups[1].toString(),
    taskName: matchGroups[2].toString(),
  };
  // console.debug(`DEBUG: ${JSON.stringify({contestProperty}, null, 2)}`)

  return contestProperty;
};

export const createTemplateFile = async (
  url: URL,
  contestProperty: ContestProperty,
) => {
  const workingDir = path.join(
    Deno.cwd(),
    `${BASE_CONTESTS_DIR}/${contestProperty.contestName}`,
  );
  // console.debug(`DEBUG: ${JSON.stringify({workingDir}, null, 2)}`)

  try {
    await Deno.mkdir(workingDir);
    console.info(`INFO: create "${workingDir}"`);
  } catch (err) {
    if (err.name === "AlreadyExists") {
      console.info(`INFO: "${workingDir}" is already exists.`);
    } else {
      throw err;
    }
  }

  const fromPath = path.join(Deno.cwd(), TEMPLATE_FILE_NAME);
  const toPath = path.join(workingDir, `${contestProperty.taskName}.rs`);

  if (await exists(toPath)) {
    console.info(`INFO: "${toPath}" is already exists.`);
    return;
  }
  await Deno.copyFile(fromPath, toPath);

  const encoder = new TextEncoder();
  const encodedComment = encoder.encode(`
// ${url.toString()}
`);

  await Deno.writeFile(
    toPath,
    encodedComment,
    { append: true },
  );

  console.info(`INFO: create "${toPath}"`);
};

export const settingProblemBin = async (contestProperty: ContestProperty) => {
  const tomlPath = path.join(Deno.cwd(), CARGO_TOML_PATH);
  const settingString = `
[[bin]]
name="${contestProperty.contestName}_${contestProperty.taskName}"
path = "${contestProperty.contestName}/${contestProperty.taskName}.rs"
`;

  const tomlFile = await Deno.readTextFile(tomlPath);
  if (
    tomlFile.match(
      `${contestProperty.contestName}_${contestProperty.taskName}`,
    ) !== null
  ) {
    console.info(`INFO: already settings to Cargo.toml.`);
    return;
  }

  const encoder = new TextEncoder();
  const encodedSettingString = encoder.encode(settingString);

  await Deno.writeFile(
    tomlPath,
    encodedSettingString,
    { append: true },
  );

  console.info(`INFO: setting to Cargo.toml.`);
};
