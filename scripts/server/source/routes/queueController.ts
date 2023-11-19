import { Body, Controller, Post, Route } from "tsoa";
import { DenoFileStore, get_queue, Queue, setup } from "../../../muse/mod.ts";
import { authPath, language } from "../../app/app.ts";

export type QueueGetParams = {
  videoId: string;
  radio: boolean;
};

@Route("queue")
export class QueueController extends Controller {
  @Post()
  public async getQueue(
    @Body() body: QueueGetParams,
  ): Promise<Queue> {
    setup({
      store: new DenoFileStore(authPath),
      language,
    });
    return await get_queue(body.videoId, null, { radio: body.radio });
  }
}
