import tensorflow as tf
import numpy as np
from PIL import Image
from json import dump as json_dump

def preprocess_image(image_path, target_size=(224, 224)):
    img = Image.open(image_path).convert("RGB")
    img = img.resize(target_size)
    img_array = np.array(img) / 255.0
    img_array = np.expand_dims(img_array, axis=0)
    return img_array

image_path = "sample_image.jpg"
image_data_array = preprocess_image(image_path)

output_file = "image_data.json"
with open(output_file, "w") as f:
    json_dump({"signature_name": "serving_default", "instances": image_data_array.tolist()}, f)

print(f"Image data saved to {output_file}")
