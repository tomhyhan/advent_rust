import numpy as np


x = np.array([[313743533208081, 328167831228390, 159732064722764], [468183773350185, 269960480220160, 439515864552130], [182013004223519, 391834672709518, 355278372451916]])
y = np.array([[-110,-135,299]])
print("shape", x.shape, y.transpose().shape)
print(np.dot(x,y.transpose()))