const { hello, helloWithCallback } = require("./index.js");

async function main() {
  console.log("=== Calling Rust from JS ===");
  console.log(hello("World"));

  console.log();

  console.log("=== Calling JS from Rust ===");
  await helloWithCallback((arg) =>
    arg
      .replace("🦀", "👻")
      .replace("{{template}}", "THIS IS JS~")
      .toUpperCase(),
  );
}

main().catch(console.error);
