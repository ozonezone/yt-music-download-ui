import { Body, Controller, Post, Route } from "tsoa";
import { get_playlist, Playlist } from "libmuse";

export type PlaylistGetParams = {
  playlistId: string;
};

@Route("playlist")
export class PlaylistController extends Controller {
  @Post()
  public async getPlaylist(
    @Body() body: PlaylistGetParams,
  ): Promise<Playlist> {
    return await get_playlist(body.playlistId);
  }
}
