"""Serves the repository root for the mock-ups, like `python3 -m http.server`, but tells the
browser never to cache: after a change, a reload always gets every file of the mock-up, never a mix
of old and new ones. From the repository root:

    python3 docs/mockups/serve.py 8765

and open http://127.0.0.1:8765/docs/mockups/<name>.html.
"""

import http.server
import sys


class NoCache(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


port = int(sys.argv[1]) if len(sys.argv) > 1 else 8765
host = sys.argv[2] if len(sys.argv) > 2 else "127.0.0.1"
http.server.ThreadingHTTPServer((host, port), NoCache).serve_forever()
