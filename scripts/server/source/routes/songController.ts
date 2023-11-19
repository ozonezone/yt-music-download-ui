import { Body, Controller, Post, Route } from "tsoa";
import { DenoFileStore, get_song, setup, Song } from "../../../muse/mod.ts";
import { authPath, language } from "../../app/app.ts";

export type SongGetParams = {
  videoId: string;
};

@Route("song")
export class SongController extends Controller {
  @Post()
  public async getSong(
    @Body() body: SongGetParams,
  ): Promise<Song> {
    setup({
      store: new DenoFileStore(authPath),
      language,
    });
    return await get_song(body.videoId);
  }
}
