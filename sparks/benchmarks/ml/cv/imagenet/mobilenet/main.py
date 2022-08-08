#!/home/xiyuzhai/anaconda3/bin/python
import numpy as np
import tensorflow as tf
import time
from tensorflow import keras
from tensorflow.keras.layers import Dense, Activation
from tensorflow.keras.optimizers import Adam
from tensorflow.keras.metrics import categorical_crossentropy
from tensorflow.keras.preprocessing.image import ImageDataGenerator
from tensorflow.keras.preprocessing import image
from tensorflow.keras.models import Model
from tensorflow.keras.applications import imagenet_utils
from sklearn.metrics import confusion_matrix
import itertools
import os
from pathlib import Path
import shutil
import random
import matplotlib.pyplot as plt

physical_devices = tf.config.experimental.list_physical_devices("GPU")
print("Num GPUs Available: ", len(physical_devices))
tf.config.experimental.set_memory_growth(physical_devices[0], True)

mobile = tf.keras.applications.mobilenet.MobileNet()

file_path = Path(__file__)
mobilenet_dir = file_path.parent.absolute()
data_dir = mobilenet_dir.joinpath("data")


def prepare_image(file):
    img_path = str(data_dir)
    img = image.load_img(img_path + "/" + file, target_size=(224, 224))
    img_array = image.img_to_array(img)
    img_array_expanded_dims = np.expand_dims(img_array, axis=0)
    return tf.keras.applications.mobilenet.preprocess_input(img_array_expanded_dims)


preprocessed_image = prepare_image("1.jpg")
start = time.time()
for _ in range(0, 100):
    predictions = mobile.predict(preprocessed_image)
end = time.time()
print("time elapsed for 100 predictions: %f seconds", end - start)
