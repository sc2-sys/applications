FROM docker.io/tensorflow/serving:2.18.0

# Set environment variables
ENV MODEL_BASE_PATH=/models
ENV MODEL_NAME=mobilenet

# Create model directory
RUN mkdir -p $MODEL_BASE_PATH/$MODEL_NAME/1

# Download a pre-trained model during the build
RUN apt-get update \
    && apt-get install -y wget \
    && wget -O /tmp/mobilenet.tar.gz https://tfhub.dev/google/imagenet/mobilenet_v2_100_224/classification/5?tf-hub-format=compressed \
    && tar -xzvf /tmp/mobilenet.tar.gz -C $MODEL_BASE_PATH/$MODEL_NAME/1

# Expose the TensorFlow Serving port
EXPOSE 8501

# Start TensorFlow Serving
CMD [\"/usr/bin/tensorflow_model_server\", \
     \"--rest_api_port=8501\", \
     \"--model_name=mobilenet\", \
     \"--model_base_path=/models/mobilenet\"]
