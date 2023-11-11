import { Body, Controller, Post, Route } from "tsoa";
import { DenoFileStore, get_playlist, Playlist, setup } from "libmuse";

export type PlaylistGetParams = {
  playlistId: string;
  authPath: string;
  language?: string;
};

@Route("playlist")
export class PlaylistController extends Controller {
  @Post()
  public async getPlaylist(
    @Body() body: PlaylistGetParams,
  ): Promise<Playlist> {
    setup({
      store: new DenoFileStore(body.authPath),
      language: body.language,
    });
    return await get_playlist(body.playlistId);
  }
}
