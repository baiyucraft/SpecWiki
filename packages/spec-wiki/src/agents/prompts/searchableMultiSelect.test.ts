import { EventEmitter } from "node:events";

import { expect, test, vi } from "vitest";

import { searchableMultiSelect } from "./searchableMultiSelect.js";

class FakeInput extends EventEmitter {
  isTTY = true;
  isRaw = false;
  setRawMode = vi.fn((value: boolean) => {
    this.isRaw = value;
  });

  resume = vi.fn();
  pause = vi.fn();
  override on(eventName: "keypress", listener: (...args: any[]) => void) {
    return super.on(eventName, listener);
  }

  override off(eventName: "keypress", listener: (...args: any[]) => void) {
    return super.off(eventName, listener);
  }
}

class FakeOutput {
  isTTY = true;
  writes: string[] = [];

  write = vi.fn((chunk: string) => {
    this.writes.push(chunk);
    return true;
  });
}

test("searchableMultiSelect confirms selection and releases the input stream", async () => {
  const input = new FakeInput();
  const output = new FakeOutput();

  const resultPromise = searchableMultiSelect({
    message: "Select hosts to bootstrap",
    input,
    output,
    choices: [
      { name: "Codex", value: "codex", preSelected: true },
      { name: "Claude Code", value: "claude" },
    ],
    validate: (selected) => selected.length > 0 || "Select at least one host",
  });

  input.emit("keypress", "", { name: "return", sequence: "\r" });

  await expect(resultPromise).resolves.toEqual(["codex"]);
  expect(input.resume).toHaveBeenCalledTimes(1);
  expect(input.setRawMode).toHaveBeenNthCalledWith(1, true);
  expect(input.setRawMode).toHaveBeenNthCalledWith(2, false);
  expect(input.pause).toHaveBeenCalledTimes(1);
});
