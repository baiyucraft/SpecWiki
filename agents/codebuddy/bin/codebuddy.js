#!/usr/bin/env node

// 发布包的主要能力在运行时 API，而不是这个命令行入口。
// 这里保持极薄，只证明构建产物存在且能被 Node 正常加载。
// 这样可以避免在入口脚本里重复实现任何 Repo Wiki 业务逻辑。
import { tools } from "../dist/index.js";

const availableToolNames = Object.keys(tools).sort();
process.stdout.write(`codebuddy-wiki agent ready: ${availableToolNames.join(", ")}\n`);
