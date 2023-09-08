/** @type {import("prettier").Config} */
const config = {
  trailingComma: "all",
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: false,
  bracketSpacing: true,
  printWidth: 80,
  endOfLine: "lf",
  plugins: [require.resolve("prettier-plugin-tailwindcss")],
  tailwindConfig: "./tailwind.config.ts",
};

module.exports = config;
