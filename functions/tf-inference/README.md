# Image Recognition Inference

This function performs a simple mobilenet inference using TensorFlow Serving.
To be able to directly curl the image, we need to pre-process the image first.
We have scripts for it in the [preprocess-image](./preprocess-image)
directory.

Once you have a pre-processed image and the model server is running, you can
just run `curl.sh`.
