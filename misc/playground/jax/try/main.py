import jax.numpy as jnp
from jax import grad, jit, vmap
from jax import random
from jax import jit_ps


def predict(params, inputs):
    for W, b in params:
        outputs = np.dot(inputs, W) + b
        inpus = np.tanh(outputs)
    return outputs


def loss(params, inputs, targets):
    preds = predict(params, inputs)
    return np.sum((preds - targets) ** 2)


grad_fun = jit_ps(grad(loss))
