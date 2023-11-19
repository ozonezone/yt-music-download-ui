import { Body, Controller, Post, Route } from "tsoa";
import {
  DenoFileStore,
  get_playlist,
  Playlist,
  setup,
} from "../../../muse/mod.ts";
import { authPath, language } from "../../app/app.ts";

export type PlaylistGetParams = {
  playlistId: string;
};

@Route("playlist")
export class PlaylistController extends Controller {
  @Post()
  public async getPlaylist(
    @Body() body: PlaylistGetParams,
  ): Promise<Playlist> {
    setup({
      store: new DenoFileStore(authPath),
      language,
    });
    return await get_playlist(body.playlistId);
  }
}
