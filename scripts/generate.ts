import {
  DefaultRouteGenerator,
  ExtendedRoutesConfig,
  generateRoutes,
  generateSpec,
} from "tsoa";
import { port } from "./config.ts";

class CustomRouterGenerator extends DefaultRouteGenerator {
  protected getRelativeImportPath(fileLocation: string) {
    return super.getRelativeImportPath(fileLocation) + ".ts";
  }
}

const routeOptions: ExtendedRoutesConfig = {
  entryFile: "./app.ts",
  noImplicitAdditionalProperties: "throw-on-extras",
  routesDir: "./build",
  controllerPathGlobs: ["**/*Controller.ts"],
  routeGenerator: CustomRouterGenerator,
};

const compilerOptions = {
  allowImportingTsExtensions: true,
};

await generateRoutes(routeOptions, compilerOptions);

await generateSpec({
  ...routeOptions,
  specVersion: 3,
  outputDirectory: "./openapi",
  version: "0.1.0",
  host: "localhost:" + port,
  schemes: ["http"],
  basePath: "/",
}, compilerOptions);
