import { Body, Controller, Post, Route } from "tsoa";
import { get_queue, Queue } from "libmuse";

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
    return await get_queue(body.videoId, null, { radio: body.radio });
  }
}
