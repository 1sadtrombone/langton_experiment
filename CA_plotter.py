import numpy as np
import matplotlib.pyplot as plt

series = np.loadtxt("target/series.csv", delimiter=',')

plt.imshow(series, cmap='binary', interpolation='none')
plt.show()