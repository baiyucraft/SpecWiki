import antfu from "@antfu/eslint-config";

export default antfu(
  {
    stylistic: {
      indent: 2,
      quotes: "double",
      semi: true,
    },
    ignores: [
      "**/node_modules/**",
      "**/dist/**",
      "**/target/**",
      ".git/**",
      ".codex/**",
      ".claude/**",
      ".wiki/**",
      "dist/**",
      "openspec/**",
      "参考/**",
    ],
  },
  {
    rules: {
      "import/consistent-type-specifier-style": "off",
      "node/prefer-global/process": "off",
      "perfectionist/sort-imports": "off",
      "perfectionist/sort-named-imports": "off",
      "style/arrow-parens": "off",
      "style/brace-style": "off",
      "style/indent": "off",
      "test/consistent-test-it": "off",
      "ts/consistent-type-definitions": "off",
    },
  },
);
