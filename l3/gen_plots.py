import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

sns.set_style("darkgrid")

df1 = pd.read_csv("data/select_data.csv", header=0, delimiter=";")
plt.title("comeripsons")
sns.lineplot(data=df1, x='n', y = 'cmps', hue="k", style="type")
plt.savefig('plots/cmps.png')
plt.clf()

plt.title("swaps")
sns.lineplot(data=df1, x='n', y = 'swaps', hue="k", style="type")
plt.savefig('plots/swaps.png')
plt.clf()

df2 = pd.read_csv("data/container_size_data.csv", header=0, delimiter=";")
plt.title("comeripsons for container size")
sns.lineplot(data=df2, x='n', y = 'cmps', hue="k", style="size")
plt.savefig('plots/cmps_container_sizes.png')
plt.clf()

plt.title("swaps for container size")
sns.lineplot(data=df2, x='n', y = 'swaps', hue="k", style="size")
plt.savefig('plots/swaps_container_sizes.png')
plt.clf()

df3 = pd.read_csv("data/bin_search_data1.csv", header=0, delimiter=";")
plt.title("comeripsons for bin search by target")
sns.lineplot(data=df3, x='n', y = 'cmps', hue="target")
plt.savefig('plots/cmps_bin_by_target.png')
plt.clf()

plt.title("time for bin search by target")
sns.lineplot(data=df3, x='n', y = 'time', hue="target")
plt.savefig('plots/time_bin_by_target.png')
plt.clf()

df4 = pd.read_csv("data/bin_search_data2.csv", header=0, delimiter=";")
plt.title("comeripsons for bin search")
sns.lineplot(data=df4, x='n', y = 'cmps')
plt.savefig('plots/cmps_bin.png')
plt.clf()

plt.title("time for bin search")
sns.lineplot(data=df4, x='n', y = 'time')
plt.savefig('plots/time_bin.png')
plt.clf()

df5 = pd.read_csv("data/quicksorts.csv", header=0, delimiter=";")
plt.title("comeripsons for quicksorts")
sns.lineplot(data=df5, x='n', y = 'cmps', hue="type")
plt.savefig('plots/quicksorts_cmps.png')
plt.clf()

plt.title("comeripsons for quicksorts")
sns.lineplot(data=df5, x='n', y = 'time', hue="type")
plt.savefig('plots/quicksorts_time.png')
plt.clf()

df6 = pd.read_csv("data/dp_quicksorts.csv", header=0, delimiter=";")
plt.title("comeripsons for dp_quicksorts")
sns.lineplot(data=df6, x='n', y = 'cmps', hue="type")
plt.savefig('plots/dp_quicksorts_cmps.png')
plt.clf()

plt.title("comeripsons for dp_quicksorts")
sns.lineplot(data=df6, x='n', y = 'time', hue="type")
plt.savefig('plots/dp_quicksorts_time.png')
plt.clf()