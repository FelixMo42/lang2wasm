import { exists } from "https://deno.land/std/fs/mod.ts"
import { lookup } from "https://deno.land/x/mime_types@1.0.0/mod.ts"

const server = Deno.listen({ port: 8000 })

async function fileResponse(path) {
    if ( await exists(path) ) {
        let body = await Deno.readFile(path)

        let headers = new Headers()
        headers.append('Content-Type', lookup(path))

        return new Response(body, {
            status: 200,
            headers
        })
    } else {
        return new Response("file not found", { status: 404 })
    }
}

for await (const conn of server) {
    (async () => {
        const httpConn = Deno.serveHttp(conn)
        for await (const request of httpConn) {
            let path = new URL(request.request.url).pathname

            if (path == "/main.wasm") {
                await Deno.run({
                    cmd: ["cargo", "run"]
                }).status()
            }

            // send the responce
            await request.respondWith( fileResponse("./res" + path) )
        }
    })()
}