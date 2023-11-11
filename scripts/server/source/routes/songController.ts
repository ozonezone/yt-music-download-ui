import { Body, Controller, Post, Route } from "tsoa";
import { get_song, Song } from "libmuse";

export type SongGetParams = {
  videoId: string;
};

@Route("song")
export class SongController extends Controller {
  @Post()
  public async getSong(
    @Body() body: SongGetParams,
  ): Promise<Song> {
    return await get_song(body.videoId);
  }
}
