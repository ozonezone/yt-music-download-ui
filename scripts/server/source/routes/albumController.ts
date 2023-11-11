import { Body, Controller, Post, Route } from "tsoa";
import { AlbumResult, get_album, get_album_browse_id } from "libmuse";

export type AlbumGetParams = {
  browseId: string;
};
export type AlbumGetByPlaylistIdParams = {
  playlistId: string;
};

@Route("album")
export class AlbumController extends Controller {
  @Post()
  public async getAlbum(
    @Body() body: AlbumGetParams,
  ): Promise<AlbumResult> {
    return await get_album(body.browseId);
  }
}

@Route("albumByPlaylistId")
export class AlbumByPlaylistIdController extends Controller {
  @Post()
  public async getAlbumByPlaylistId(
    @Body() body: AlbumGetByPlaylistIdParams,
  ): Promise<AlbumResult> {
    const browseId = await get_album_browse_id(body.playlistId);
    if (!browseId) {
      throw new Error("No album found for playlist");
    }
    const album = await get_album(browseId);
    return album;
  }
}
