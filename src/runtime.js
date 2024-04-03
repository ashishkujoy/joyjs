((globalThis) => {
  const core = Deno.core;
  const argToMessage = (...args) => args.map(arg => JSON.stringify(arg)).join(" ");
  globalThis.myconsole = {
    log: (...args) => core.print(argToMessage(...args), false),
  }
  globalThis.fs = {
    readFileSync: async (filePath) => await core.ops.op_read_file(filePath)
  }
})(globalThis);
