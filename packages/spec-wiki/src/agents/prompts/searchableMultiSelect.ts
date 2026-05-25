/**
 * 这个文件实现 `spec-wiki init` 的交互式多选提示。
 * 它参考 UniSpec 的 searchable multi-select，但只保留当前三宿主场景需要的键盘交互。
 */
import { clearScreenDown, emitKeypressEvents, moveCursor } from "node:readline";

type PromptInput = {
  isTTY?: boolean;
  isRaw?: boolean;
  setRawMode: (mode: boolean) => void;
  resume: () => void;
  pause: () => void;
  on: (eventName: "keypress", listener: (character: string, key: Keypress) => void) => void;
  off: (eventName: "keypress", listener: (character: string, key: Keypress) => void) => void;
};

type PromptOutput = {
  isTTY?: boolean;
  write: (chunk: string) => boolean;
};

type Keypress = {
  name?: string;
  ctrl?: boolean;
  sequence?: string;
};

export type SearchableChoice = {
  /** 展示名称。 */
  name: string;
  /** 稳定值。 */
  value: string;
  /** 是否已在当前项目中检测到。 */
  detected?: boolean;
  /** 是否已被预选中。 */
  preSelected?: boolean;
};

export type SearchableMultiSelectConfig = {
  /** 提示标题。 */
  message: string;
  /** 当前可选项。 */
  choices: SearchableChoice[];
  /** 最多显示多少条。 */
  pageSize?: number;
  /** 结果校验函数。 */
  validate?: (selected: string[]) => boolean | string;
  /** 测试可注入的输入流。 */
  input?: PromptInput;
  /** 测试可注入的输出流。 */
  output?: PromptOutput;
};

const ANSI = {
  dim: "\u001B[2m",
  cyan: "\u001B[36m",
  green: "\u001B[32m",
  yellow: "\u001B[33m",
  bold: "\u001B[1m",
  inverse: "\u001B[7m",
  reset: "\u001B[0m",
  hideCursor: "\u001B[?25l",
  showCursor: "\u001B[?25h",
};

function paint(text: string, color: string): string {
  return `${color}${text}${ANSI.reset}`;
}

function formatChoiceSuffix(choice: SearchableChoice, selected: boolean): string {
  if (selected) {
    return paint(" (selected)", ANSI.dim);
  }

  if (choice.detected) {
    return paint(" (detected)", ANSI.dim);
  }

  return "";
}

function clampCursor(cursor: number, size: number): number {
  if (size <= 0) {
    return 0;
  }

  return Math.max(0, Math.min(cursor, size - 1));
}

function filterChoices(
  choices: SearchableChoice[],
  searchText: string,
): SearchableChoice[] {
  if (!searchText.trim()) {
    return choices;
  }

  const term = searchText.toLowerCase();
  return choices.filter((choice) =>
    choice.name.toLowerCase().includes(term)
    || choice.value.toLowerCase().includes(term),
  );
}

function renderPrompt(
  config: SearchableMultiSelectConfig,
  searchText: string,
  selectedValues: string[],
  cursor: number,
  error: string | null,
): string {
  const { message, choices, pageSize = 10 } = config;
  const filteredChoices = filterChoices(choices, searchText);
  const selectedSet = new Set(selectedValues);
  const normalizedCursor = clampCursor(cursor, filteredChoices.length);
  const lines: string[] = [];

  lines.push(`${paint("?", ANSI.cyan)} ${paint(message, ANSI.bold)}`);

  const selectedLine = selectedValues.length > 0
    ? selectedValues
        .map((value) => choices.find((choice) => choice.value === value)?.name ?? value)
        .join(", ")
    : paint("(none selected)", ANSI.dim);
  lines.push(`  Selected: ${selectedLine}`);
  lines.push(
    `  Search: ${paint("[", ANSI.yellow)}${searchText || paint("type to filter", ANSI.dim)}${paint("]", ANSI.yellow,
    )}`,
  );
  lines.push(
    `  ${paint("↑↓", ANSI.cyan)} navigate  ${paint("Space", ANSI.cyan)} toggle  ${paint("Backspace", ANSI.cyan)} remove  ${paint("Enter", ANSI.cyan)} confirm`,
  );

  if (filteredChoices.length === 0) {
    lines.push(`  ${paint("No matches", ANSI.yellow)}`);
  } else {
    const startIndex = Math.max(
      0,
      Math.min(
        normalizedCursor - Math.floor(pageSize / 2),
        Math.max(0, filteredChoices.length - pageSize),
      ),
    );
    const endIndex = Math.min(startIndex + pageSize, filteredChoices.length);

    for (let index = startIndex; index < endIndex; index += 1) {
      const choice = filteredChoices[index];
      const isActive = index === normalizedCursor;
      const isSelected = selectedSet.has(choice.value);
      const arrow = isActive ? paint(">", ANSI.cyan) : " ";
      const marker = isSelected ? paint("[x]", ANSI.green) : paint("[ ]", ANSI.dim);
      const label = isActive ? paint(choice.name, ANSI.inverse) : choice.name;
      lines.push(`  ${arrow} ${marker} ${label}${formatChoiceSuffix(choice, isSelected)}`);
    }

    if (filteredChoices.length > pageSize) {
      const currentPage = Math.floor(normalizedCursor / pageSize) + 1;
      const totalPages = Math.ceil(filteredChoices.length / pageSize);
      lines.push(paint(`  (${currentPage}/${totalPages})`, ANSI.dim));
    }
  }

  if (error) {
    lines.push(paint(`  ${error}`, ANSI.yellow));
  }

  return lines.join("\n");
}

/**
 * 通过空格切换、回车确认的交互式多选提示。
 *
 * @param config 提示配置。
 * @returns 返回最终选中的 value 列表。
 */
export async function searchableMultiSelect(
  config: SearchableMultiSelectConfig,
): Promise<string[]> {
  const input = config.input ?? process.stdin;
  const output = config.output ?? process.stdout;

  if (!input.isTTY || !output.isTTY || typeof input.setRawMode !== "function") {
    throw new Error("interactive prompt requires a TTY terminal");
  }

  const selectedValues = config.choices
    .filter((choice) => choice.preSelected)
    .map((choice) => choice.value);
  let searchText = "";
  let cursor = 0;
  let error: string | null = null;
  let renderedLineCount = 0;
  const previousRawMode = input.isRaw;

  emitKeypressEvents(input as NodeJS.ReadStream);
  input.setRawMode(true);
  input.resume();
  output.write(ANSI.hideCursor);

  const redraw = () => {
    const view = renderPrompt(config, searchText, selectedValues, cursor, error);

    if (renderedLineCount > 0) {
      moveCursor(output as NodeJS.WriteStream, 0, -renderedLineCount);
      clearScreenDown(output as NodeJS.WriteStream);
    }

    output.write(`${view}\n`);
    renderedLineCount = view.split("\n").length;
  };

  return await new Promise<string[]>((resolve, reject) => {
    const finish = (result?: string[], promptError?: Error) => {
      input.off("keypress", onKeypress);
      if (!previousRawMode) {
        input.setRawMode(false);
      }
      input.pause();
      output.write(ANSI.showCursor);
      clearScreenDown(output as NodeJS.WriteStream);

      if (promptError) {
        reject(promptError);
        return;
      }

      const selectedNames = (result ?? [])
        .map((value) => config.choices.find((choice) => choice.value === value)?.name ?? value)
        .join(", ");
      output.write(
        `${paint("?", ANSI.cyan)} ${paint(config.message, ANSI.bold)} ${paint(selectedNames || "(none)", ANSI.cyan)}\n`,
      );
      resolve(result ?? []);
    };

    function onKeypress(_character: string, key: Keypress) {
      const filteredChoices = filterChoices(config.choices, searchText);
      cursor = clampCursor(cursor, filteredChoices.length);

      if (key.ctrl && key.name === "c") {
        finish(undefined, new Error("interactive prompt cancelled"));
        return;
      }

      if (key.name === "return") {
        if (config.validate) {
          const result = config.validate([...selectedValues]);
          if (result !== true) {
            error = typeof result === "string" ? result : "Invalid selection";
            redraw();
            return;
          }
        }

        finish([...selectedValues]);
        return;
      }

      if (key.name === "space") {
        const activeChoice = filteredChoices[cursor];
        if (activeChoice) {
          const selectedIndex = selectedValues.indexOf(activeChoice.value);
          if (selectedIndex >= 0) {
            selectedValues.splice(selectedIndex, 1);
          } else {
            selectedValues.push(activeChoice.value);
          }
        }
        error = null;
        redraw();
        return;
      }

      if (key.name === "backspace") {
        if (searchText.length > 0) {
          searchText = searchText.slice(0, -1);
        } else if (selectedValues.length > 0) {
          selectedValues.pop();
        }
        cursor = 0;
        error = null;
        redraw();
        return;
      }

      if (key.name === "up") {
        cursor = clampCursor(cursor - 1, filteredChoices.length);
        error = null;
        redraw();
        return;
      }

      if (key.name === "down") {
        cursor = clampCursor(cursor + 1, filteredChoices.length);
        error = null;
        redraw();
        return;
      }

      if (!key.ctrl && key.sequence && key.sequence.length === 1 && key.sequence >= " ") {
        searchText += key.sequence;
        cursor = 0;
        error = null;
        redraw();
      }
    };

    input.on("keypress", onKeypress);
    redraw();
  });
}
