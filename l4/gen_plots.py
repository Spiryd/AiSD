import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.read_csv("./data/trees.csv", sep=";")
print(df.head())
avg_df = df[df["type"] == "avg"]
max_df = df[df["type"] == "max"]

sns.set_style("darkgrid")

plt.title("max_cmp")
sns.lineplot(data=max_df , x="n", y="cmp", hue="tree")
plt.savefig("./plots/max_cmp.png")
plt.clf()

plt.title("max_rns")
sns.lineplot(data=max_df , x="n", y="rns", hue="tree")
plt.savefig("./plots/max_rns.png")
plt.clf()

plt.title("max_h")
sns.lineplot(data=max_df , x="n", y="h", hue="tree")
plt.savefig("./plots/max_h.png")
plt.clf()

plt.title("avg_cmp")
sns.lineplot(data=avg_df , x="n", y="cmp", hue="tree")
plt.savefig("./plots/avg_cmp.png")
plt.clf()

plt.title("avg_rns")
sns.lineplot(data=avg_df , x="n", y="rns", hue="tree")
plt.savefig("./plots/avg_rns.png")
plt.clf()

plt.title("avg_h")
sns.lineplot(data=avg_df , x="n", y="h", hue="tree")
plt.savefig("./plots/avg_h.png")
plt.clf()
