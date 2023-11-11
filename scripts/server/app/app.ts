import express, { json, urlencoded } from "express";
import { RegisterRoutes } from "./build/routes.ts";

const port = Deno.args[0];
if (!port) {
  throw new Error("Please pass port");
}

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
