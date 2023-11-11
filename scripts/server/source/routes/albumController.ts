import { Body, Controller, Post, Route } from "tsoa";
import {
  AlbumResult,
  DenoFileStore,
  get_album,
  get_album_browse_id,
  setup,
} from "libmuse";

export type AlbumGetParams = {
  browseId: string;
  authPath: string;
  language?: string;
};
export type AlbumGetByPlaylistIdParams = {
  playlistId: string;
  authPath: string;
  language?: string;
};

@Route("album")
export class AlbumController extends Controller {
  @Post()
  public async getAlbum(
    @Body() body: AlbumGetParams,
  ): Promise<AlbumResult> {
    setup({
      store: new DenoFileStore(body.authPath),
      language: body.language,
    });
    return await get_album(body.browseId);
  }
}

@Route("albumByPlaylistId")
export class AlbumByPlaylistIdController extends Controller {
  @Post()
  public async getAlbumByPlaylistId(
    @Body() body: AlbumGetByPlaylistIdParams,
  ): Promise<AlbumResult> {
    setup({
      store: new DenoFileStore(body.authPath),
      language: body.language,
    });
    const browseId = await get_album_browse_id(body.playlistId);
    if (!browseId) {
      throw new Error("No album found for playlist");
    }
    return await get_album(browseId);
  }
}
