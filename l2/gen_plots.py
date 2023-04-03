import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

df = pd.read_csv("data.csv", header=0, delimiter=";")
df["c/n"] = df["cmps"]/df["n"]
df["s/n"] = df["swaps"]/df["n"]
sns.set_style("darkgrid")

plt.title("comeripsons")
sns.lineplot(data=df, x='n', y = 'cmps', hue="type")
plt.savefig('plots/cmps.png')
plt.clf()

plt.title("swaps")
sns.lineplot(data=df, x='n', y = 'swaps', hue="type")
plt.savefig('plots/swaps.png')
plt.clf()

plt.title("c/n")
sns.lineplot(data=df, x='n', y = 'c/n', hue="type")
plt.savefig('plots/cn.png')
plt.clf()

plt.title("s/n")
sns.lineplot(data=df, x='n', y = 's/n', hue="type")
plt.savefig('plots/sn.png')
plt.clf()
