import matplotlib.pyplot as plt
import os

plt.figure()
for filename in os.listdir(os.path.join(os.getcwd(), 'out')):
    if filename.endswith('.dat'):
        method = filename.split('.')[0]
        with open(os.path.join(os.getcwd(), 'out', filename)) as f:
            res = f.readlines()
            indices = []
            times = []
            for line in res:
                index,time = tuple(map(int, line.split(',')))
                indices.append(index)
                times.append(time)
        plt.plot(indices, times, label = method)
plt.legend()
plt.show()
