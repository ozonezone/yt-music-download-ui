# Deno server part

```
.
│ app
│ │ build          # generated routes
│ │ openapi        # openapi schema
│ └ app.ts         # Deno server entry
│ source
│ │ routes         # Deno server entity
│ │ deno.jsonc
│ │ generate.ts    # script to generate openapi definition, rust types and routes to app folder.

../../api          # Generated openapi rust client
```
