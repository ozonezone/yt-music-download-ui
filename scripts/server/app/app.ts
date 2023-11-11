import express, { json, urlencoded } from "express";
import { RegisterRoutes } from "./build/routes.ts";
import { DenoFileStore, setup } from "libmuse";

const port = Deno.args[0];
const authPath = Deno.args[1];
const language = Deno.args[2];

setup({
  store: new DenoFileStore(authPath),
  language,
});

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
  () => {
    console.log("Deno server started");
    console.log(`|   listening at http://localhost:${port}`);
    console.log(`|   authPath: ${authPath}, language: ${language}`);
  },
);
