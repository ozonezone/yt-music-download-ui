import { DenoFileStore, get_queue, setup } from "muse/mod.ts";

const videoId = Deno.args[0];
const storePath = Deno.args[1];
const radio = Deno.args[2] === "true";

if (!storePath) {
  throw new Error("store pass is not passed");
}
if (!videoId) {
  throw new Error("videoId is not passed");
}

setup({
  store: new DenoFileStore(storePath),
});

console.log(JSON.stringify(await get_queue(videoId, null, { radio })));
