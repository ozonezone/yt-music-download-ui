import { Body, Controller, Post, Route } from "tsoa";
import { DenoFileStore, get_song, setup, Song } from "libmuse";

export type SongGetParams = {
  videoId: string;
  authPath: string;
  language?: string;
};

@Route("song")
export class SongController extends Controller {
  @Post()
  public async getSong(
    @Body() body: SongGetParams,
  ): Promise<Song> {
    setup({
      store: new DenoFileStore(body.authPath),
      language: body.language,
    });
    return await get_song(body.videoId);
  }
}
