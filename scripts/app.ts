import express, { json, urlencoded } from "express";
import { RegisterRoutes } from "./build/routes.ts";
import { port } from "./config.ts";

export const app = express();

app.use(
  urlencoded({
    extended: true,
  }),
);
app.use(json());

RegisterRoutes(app);

app.listen(
  port,
  () => console.log(`Deno server listening at http://localhost:${port}`),
);
