addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const { run } = wasm_bindgen;
  await wasm_bindgen(wasm);

  // Fetch secrets for this application
  const JWT = ROCTOGEN_JWT_EXAMPLE_PART1 + ROCTOGEN_JWT_EXAMPLE_PART2;
  const APP_ID = parseInt(ROCTOGEN_JWT_EXAMPLE_APP_ID);

  // Trigger Rust entrypoint
  const resp = await run(JWT, APP_ID);

  // Stringify JsValue response value
  return new Response(JSON.stringify(resp, null, 2), {status: 200});
}
