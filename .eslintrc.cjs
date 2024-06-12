/** @type { import("eslint").Linter.Config } */
module.exports = {
  root: true,
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:svelte/prettier",
    "plugin:readable-tailwind/error",
    "prettier",
  ],
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint", "readable-tailwind", "import"],
  parserOptions: {
    sourceType: "module",
    ecmaVersion: 2020,
    extraFileExtensions: [".svelte"],
  },
  env: {
    browser: true,
    es2017: true,
    node: true,
  },
  rules: {
    "import/order": [
      "error",
      {
        alphabetize: {
          caseInsensitive: false,
          order: "asc",
          orderImportKind: "asc",
        },
        groups: ["builtin", "external", "internal", "parent", "sibling", "index", "object", "type"],
        "newlines-between": "always",
      },
    ],
    "sort-imports": [
      "error",
      {
        ignoreDeclarationSort: true,
      },
    ],
    "svelte/no-extra-reactive-curlies": ["error"],
    "svelte/no-spaces-around-equal-signs-in-attribute": ["error"],
    "svelte/prefer-class-directive": ["error"],
    "svelte/prefer-style-directive": ["error"],
    "svelte/shorthand-attribute": ["error"],
    "svelte/shorthand-directive": ["error"],
    "svelte/sort-attributes": ["error"],
  },
  overrides: [
    {
      files: ["*.svelte"],
      parser: "svelte-eslint-parser",
      parserOptions: {
        parser: "@typescript-eslint/parser",
      },
    },
  ],
};
