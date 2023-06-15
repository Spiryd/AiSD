import random

import sys
sys.setrecursionlimit(16000)

import math
def floor_log(x):
    return math.frexp(x)[1] - 1

cmps = 0

class BinomialTree:
    def __init__(self, key):
        self.key = key
        self.children = []
        self.order = 0
 
    def add_at_end(self, t):
        self.children.append(t)
        self.order = self.order + 1
 
 
class BinomialHeap:
    def __init__(self):
        self.trees = []
 
    def extract_min(self):
        global cmps
        if self.trees == []:
            return None
        smallest_node = self.trees[0]
        for tree in self.trees:
            cmps += 1
            if tree.key < smallest_node.key:
                smallest_node = tree
        self.trees.remove(smallest_node)
        h = BinomialHeap()
        h.trees = smallest_node.children
        self.merge(h)
 
        return smallest_node.key
 
    def get_min(self):
        global cmps
        if self.trees == []:
            return None
        least = self.trees[0].key
        for tree in self.trees:
            cmps += 1
            if tree.key < least:
                least = tree.key
        return least
 
    def combine_roots(self, h):
        self.trees.extend(h.trees)
        self.trees.sort(key=lambda tree: tree.order)
 
    def merge(self, h):
        global cmps
        self.combine_roots(h)
        if self.trees == []:
            return
        i = 0
        while i < len(self.trees) - 1:
            current = self.trees[i]
            after = self.trees[i + 1]
            if current.order == after.order:
                if (i + 1 < len(self.trees) - 1
                    and self.trees[i + 2].order == after.order):
                    after_after = self.trees[i + 2]
                    cmps += 1
                    if after.key < after_after.key:
                        after.add_at_end(after_after)
                        del self.trees[i + 2]
                    else:
                        after_after.add_at_end(after)
                        del self.trees[i + 1]
                else:
                    cmps += 1
                    if current.key < after.key:
                        current.add_at_end(after)
                        del self.trees[i + 1]
                    else:
                        after.add_at_end(current)
                        del self.trees[i]
            i = i + 1
 
    def insert(self, key):
        g = BinomialHeap()
        g.trees.append(BinomialTree(key))
        self.merge(g)
    
# Creating fibonacci tree
class FibonacciTree:
    def __init__(self, value):
        self.value = value
        self.child = []
        self.order = 0

    # Adding tree at the end of the tree
    def add_at_end(self, t):
        self.child.append(t)
        self.order = self.order + 1


class FibonacciHeap:
    def __init__(self):
        self.trees = []
        self.least = None
        self.count = 0

    # Insert a node
    def insert(self, value):
        global cmps
        new_tree = FibonacciTree(value)
        self.trees.append(new_tree)
        cmps += 1
        if self.least is None or value < self.least.value:
            self.least = new_tree
        self.count += 1

    # Get minimum value
    def get_min(self):
        if self.least is None:
            return None
        return self.least.value

    # Extract the minimum value
    def extract_min(self):
        smallest = self.least
        if smallest is not None:
            for child in smallest.child:
                self.trees.append(child)
            self.trees.remove(smallest)
            if self.trees == []:
                self.least = None
            else:
                self.least = self.trees[0]
                self.consolidate()
            self.count -= 1
            return smallest.value

    # Consolidate the tree
    def consolidate(self):
        global cmps
        aux = (floor_log(self.count) + 1) * [None]

        while self.trees != []:
            x = self.trees[0]
            order = x.order
            self.trees.remove(x)
            while aux[order] is not None:
                y = aux[order]
                cmps += 1
                if x.value > y.value:
                    x, y = y, x
                x.add_at_end(y)
                aux[order] = None
                order += 1
            aux[order] = x

        self.least = None
        for k in aux:
            if k is not None:
                self.trees.append(k)
                cmps += 1
                if self.least is None or k.value < self.least.value:
                    self.least = k

    # Union two Fibonacci heaps
    def merge(self, other_heap):
        global cmps
        self.trees.extend(other_heap.trees)
        cmps += 1
        if self.least is None or (other_heap.least is not None and other_heap.least.value < self.least.value):
            self.least = other_heap.least
        self.count += other_heap.count

def collect_data1():
    global cmps
    with open("./data/heap_data1.csv", "w") as file:
        file.write("k;n;i;type;cmp\n")
        for n in [500, 1000]:
            for k in range(5):
                i = 0
                cmps = 0
                heap1 = BinomialHeap()
                file.write(f"{k};{n};{i};Bin;{cmps}\n")
                cmps = 0
                heap2 = BinomialHeap()
                i += 1
                file.write(f"{k};{n};{i};Bin;{cmps}\n")
                for _ in range(n):
                    cmps = 0
                    heap1.insert(random.randint(0, (2*n - 1)))
                    i += 1
                    file.write(f"{k};{n};{i};Bin;{cmps}\n")
                for _ in range(n):
                    cmps = 0
                    heap2.insert(random.randint(0, (2*n - 1)))
                    i += 1
                    file.write(f"{k};{n};{i};Bin;{cmps}\n")
                cmps = 0
                heap1.merge(heap2)
                i += 1
                file.write(f"{k};{n};{i};Bin;{cmps}\n")
                for _ in range(2*n):
                    cmps = 0
                    heap1.extract_min()
                    i += 1
                    file.write(f"{k};{n};{i};Bin;{cmps}\n")
                
                i = 0
                cmps = 0
                heap1 = FibonacciHeap()
                file.write(f"{k};{n};{i};Fib;{cmps}\n")
                cmps = 0
                heap2 = FibonacciHeap()
                i += 1
                file.write(f"{k};{n};{i};Fib;{cmps}\n")
                for _ in range(n):
                    cmps = 0
                    heap1.insert(random.randint(0, (2*n - 1)))
                    i += 1
                    file.write(f"{k};{n};{i};Fib;{cmps}\n")
                for _ in range(n):
                    cmps = 0
                    heap2.insert(random.randint(0, (2*n - 1)))
                    i += 1
                    file.write(f"{k};{n};{i};Fib;{cmps}\n")
                cmps = 0
                heap1.merge(heap2)
                i += 1
                file.write(f"{k};{n};{i};Fib;{cmps}\n")
                for _ in range(2*n):
                    cmps = 0
                    heap1.extract_min()
                    i += 1
                    file.write(f"{k};{n};{i};Fib;{cmps}\n")    

def collect_data2():
    global cmps   
    with open("./data/heap_data2.csv", "w") as file:
        file.write("n;type;cmp\n")
        for n in range(100, 10001, 100):
            heap1 = BinomialHeap()
            heap2 = BinomialHeap()
            for _ in range(n):
                heap1.insert(random.randint(0, (2*n - 1)))
                heap2.insert(random.randint(0, (2*n - 1)))
            heap1.merge(heap2)
            for _ in range(2*n):
                heap1.extract_min()
            file.write(f"{n};Bin;{cmps}\n")

            cmps = 0
            heap1 = FibonacciHeap()
            heap2 = FibonacciHeap()
            for _ in range(n):
                heap1.insert(random.randint(0, (2*n - 1)))
                heap2.insert(random.randint(0, (2*n - 1)))
            heap1.merge(heap2)
            for _ in range(2*n):
                heap1.extract_min()
            file.write(f"{n};Fib;{cmps}\n")


if __name__ == "__main__":
    collect_data1()
    collect_data2()
    heap1 = BinomialHeap()
    print("BinomialHeap1 created")
    heap1.insert(5)
    print("Inserted 5")
    heap1.insert(8)
    print("Inserted 8")

    heap2 = BinomialHeap()
    print("BinomialHeap2 created")
    heap2.insert(3)
    print("Inserted 3")
    heap2.insert(12)
    print("Inserted 12")

    heap1.merge(heap2)
    print("Heaps Merged")

    print("extracting mins...")
    min_value = heap1.extract_min()
    print(min_value)  # Output: 3
    min_value = heap1.extract_min()
    print(min_value)  # Output: 5
    min_value = heap1.extract_min()
    print(min_value)  # Output: 8
    min_value = heap1.extract_min()
    print(min_value)  # Output: 12

    heap1 = FibonacciHeap()
    print("FibonacciHeap1 created")
    heap1.insert(5)
    print("Inserted 5")
    heap1.insert(8)
    print("Inserted 8")

    heap2 = FibonacciHeap()
    print("FibonacciHeap2 created")
    heap2.insert(3)
    print("Inserted 3")
    heap2.insert(12)
    print("Inserted 12")

    heap1.merge(heap2)
    print("Heaps Merged")

    print("extracting mins...")
    min_value = heap1.extract_min()
    print(min_value)  # Output: 3
    min_value = heap1.extract_min()
    print(min_value)  # Output: 5
    min_value = heap1.extract_min()
    print(min_value)  # Output: 8
    min_value = heap1.extract_min()
    print(min_value)  # Output: 12
