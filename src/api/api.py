from flask import Flask, request
from flask_cors import CORS
import tensorflow as tf
import os


app = Flask(__name__)
CORS(app)
app.secret_key = os.urandom(20)


model = tf.saved_model.load('translator')


@app.route("/predict", methods=["POST"])
def predict():
    arg = request.form.get("data")

    prediction = model(tf.constant(arg)).numpy().decode()


    return prediction


if __name__ == "__main__":
    app.run(host='127.0.0.1', port=3000)