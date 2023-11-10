import { DenoFileStore, get_playlist, setup } from "muse/mod.ts";

const playlistId = Deno.args[0];
const storePath = Deno.args[1];

if (!storePath) {
  throw new Error("store pass is not passed");
}
if (!playlistId) {
  throw new Error("playlistId is not passed");
}

setup({
  store: new DenoFileStore(storePath),
  language: "ja",
});

console.log(JSON.stringify(await get_playlist(playlistId)));
