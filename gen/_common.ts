import { mkdirSync, writeFileSync } from "fs";
import { join } from "path";

export const RUST_OUT_DIR = join(__dirname, "..", "rust", "common", "gen");

try {
  mkdirSync(RUST_OUT_DIR);
} catch (err) {
  if (err.code !== "EEXIST") {
    throw err;
  }
}
writeFileSync(
  join(RUST_OUT_DIR, "mod.rs"),
  `
pub mod attrs;
pub mod codepoints;
pub mod entities;
`
);

export const DATA_DIR = join(__dirname, "data");

export const leftPad = (str: string, n: number) =>
  "0".repeat(n - str.length) + str;

export const prettyJson = (v: any) => JSON.stringify(v, null, 2);

export const byteStringLiteral = (bytes: number[]): string =>
  [
    'b"',
    ...bytes.map((c) => {
      if (!Number.isSafeInteger(c) || c < 0 || c > 255) {
        throw new Error("Not a byte");
      }
      // 0x20 == ' '.
      // 0x7E == '~'.
      // 0x5C == '\\'.
      // 0x22 == '"'.
      if (c >= 0x20 && c <= 0x7e && c != 0x5c && c != 0x22) {
        return String.fromCharCode(c);
      } else {
        return `\\x${leftPad(c.toString(16), 2)}`;
      }
    }),
    '"',
  ].join("");
