import { Body, Controller, Post, Route } from "tsoa";
import { DenoFileStore, get_queue, Queue, setup } from "libmuse";

export type QueueGetParams = {
  videoId: string;
  authPath: string;
  radio: boolean;
  language?: string;
};

@Route("queue")
export class QueueController extends Controller {
  @Post()
  public async getQueue(
    @Body() body: QueueGetParams,
  ): Promise<Queue> {
    setup({
      store: new DenoFileStore(body.authPath),
      language: body.language,
    });
    return await get_queue(body.videoId, null, { radio: body.radio });
  }
}
