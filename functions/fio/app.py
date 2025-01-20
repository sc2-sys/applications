from flask import Flask
from os import environ
from subprocess import run

app = Flask(__name__)


@app.route("/")
def fio_benchmark():
    run("fio /fio_read_jobfile.fio", shell=True, check=True)
    return "Fio read benchmark done!\n"


if __name__ == "__main__":
    app.run(debug=True, host="0.0.0.0", port=8080)
