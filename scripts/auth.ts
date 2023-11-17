import { dirname } from "https://deno.land/std@0.181.0/path/mod.ts";
import { DenoFileStore, get_option, setup } from "libmuse";

const storePath = Deno.args[0];

if (!storePath) {
  throw new Error("store pass is not passed");
}

try {
  await Deno.stat(storePath);
} catch {
  await Deno.mkdir(dirname(storePath), { recursive: true });
}

setup({
  store: new DenoFileStore(storePath),
  debug: true,
});

const auth = get_option("auth");

if (auth.has_token()) {
  console.log("Already logged in!");
} else {
  console.log("Getting login code...");

  const loginCode = await auth.get_login_code();

  console.log(
    `Go to ${loginCode.verification_url} and enter the code: ${loginCode.user_code}`,
  );

  confirm("Press enter when you have logged in");

  console.log("Loading token...");

  await auth.load_token_with_code(loginCode);

  console.log("Logged in!", auth._token);
}
