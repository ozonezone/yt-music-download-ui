import {
  DefaultRouteGenerator,
  ExtendedRoutesConfig,
  generateRoutes,
  generateSpec,
} from "tsoa";

class CustomRouterGenerator extends DefaultRouteGenerator {
  protected getRelativeImportPath(fileLocation: string) {
    return super.getRelativeImportPath(fileLocation) + ".ts";
  }
}

const routeOptions: ExtendedRoutesConfig = {
  entryFile: "../app/app.ts",
  noImplicitAdditionalProperties: "throw-on-extras",
  routesDir: "../app/build",
  controllerPathGlobs: ["**/*Controller.ts"],
  routeGenerator: CustomRouterGenerator,
};

const compilerOptions = {
  allowImportingTsExtensions: true,
};

console.log("Generating routes...");
await generateRoutes(routeOptions, compilerOptions);

console.log("Generating OpenAPI spec...");
await generateSpec({
  ...routeOptions,
  specVersion: 3,
  outputDirectory: "../app/openapi",
  version: "0.1.0",
  host: "localhost",
  schemes: ["http"],
  basePath: "/",
}, compilerOptions);

console.log("Patching OpenAPI spec...");
const spec = JSON.parse(await Deno.readTextFile("../app/openapi/swagger.json"));
// spec.components.schemas["VideoType"].enum.push(
//   "MUSIC_VIDEO_TYPE_PRIVATELY_OWNED_TRACK",
// );
// spec.components.schemas.Playlist.properties.trackCount = {};
// spec.components.schemas.Queue.properties.current.required = spec.components
//   .schemas.Queue.properties.current.required.filter((item: string) => {
//     return !(item == "playlistId" || item == "index");
//   });
// If language is not endlish, this string is localized
// delete spec.components.schemas.AlbumType.enum;
// spec.components.schemas.PlaylistItem.properties.thumbnails.nullable = true;
// spec.components.schemas.AlbumResult.properties.artists = {
//   "items": {
//     "$ref": "#/components/schemas/ArtistRun",
//   },
//   "type": "array",
// };
// For private album, videotype can be null.
// spec.components.schemas.PlaylistItem.properties.videoType = {
//   "allOf": [
//     {
//       "$ref": "#/components/schemas/VideoType",
//     },
//   ],
//   "nullable": true,
// };
await Deno.writeTextFile(
  "../app/openapi/swagger.json",
  JSON.stringify(spec, null, 2),
);

console.log("Generating Rust client...");
const out = await new Deno.Command("deno", {
  // deno-fmt-ignore
  args: [
    "run", "-A", "npm:@openapitools/openapi-generator-cli",
    "generate",
    "-i", "../app/openapi/swagger.json",
    "-g", "rust",
    "-o", "../../../api",
  ],
}).output();
console.log(new TextDecoder().decode(out.stderr));
