addEventListener('fetch', event => {
  event.respondWith(handle());
})

addEventListener('scheduled', event => {
  event.waitUntil(handle());
})

async function handle() {
  const { run } = wasm_bindgen;
  await wasm_bindgen(wasm);

  // Fetch secrets for this application
  const JWT = ROCTOGEN_JWT_EXAMPLE_PART1 + ROCTOGEN_JWT_EXAMPLE_PART2;
  const APP_ID = parseInt(ROCTOGEN_JWT_EXAMPLE_APP_ID);

  // Trigger Rust entrypoint
  const resp = await run(JWT, APP_ID);

  return new Response(resp, {status: 200});
}
