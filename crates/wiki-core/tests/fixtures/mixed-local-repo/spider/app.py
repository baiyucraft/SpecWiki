from flask import Flask, jsonify
from modules.gc_sql import GCSql

app = Flask(__name__)


@app.route("/api/news")
def api_news():
    GCSql()
    return jsonify({"ok": True})


if __name__ == "__main__":
    app.run(port=4096)
