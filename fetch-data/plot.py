import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy.stats import norm
import seaborn as sns

sns.set_theme()

transactions = ["new_order", "payment", "order_status", "delivery", "stock_level", "total_time"]

file = 'out.csv'
df = pd.read_csv(file)

data = df.loc[:,transactions[:-1]]
transpose_data = pd.melt(data)

for transaction in transactions:
    print("===============================")
    print(f"Pandas Describe {transaction}: ", df[transaction].describe())
    print("===============================")

sns.boxplot(x="variable", y="value", data=transpose_data)
plt.show()

fig, axes = plt.subplots(6, 2, figsize=(20, 30), gridspec_kw={'wspace': 0.5, 'hspace': 1})
for idx, transaction in enumerate(transactions):
    plt.title(transaction)
    sns.boxplot(x=df[transaction], ax=axes[idx, 0])

    axes[idx, 1].hist(df[transaction], density=True)
    arr = df[transaction]
    mean = arr.mean()
    sigma = arr.std()
    x = np.linspace(min(arr), max(arr), 100)
    plt.title(transaction)
    axes[idx, 1].plot(x, norm.pdf(x, mean, sigma))
plt.show()
