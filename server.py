import http.server
import os
import socketserver

def get_port():
    port_env = os.environ.get("PORT")
    return int(port_env) if port_env else 8080

PORT = get_port()

class CustomHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.insert_custom_headers()
        super(CustomHttpRequestHandler, self).end_headers()

    def insert_custom_headers(self):
        # Required if using SharedArrayBuffer for sharing memory with workers
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")

CustomHttpRequestHandler.extensions_map = {
    '.html': 'text/html',
    '.wasm': 'application/wasm',
    '': 'application/octet-stream',
}

with socketserver.TCPServer(("", PORT), CustomHttpRequestHandler) as server:
    print(f"HTTP Server listening at port {PORT} ..")
    server.serve_forever()
