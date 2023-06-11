class BinomialHeapNode:
    def __init__(self, value):
        self.value = value
        self.degree = 0
        self.parent = None
        self.child = None
        self.sibling = None

class BinomialHeap:
    def __init__(self):
        self.head = None

    def merge(self, heap):
        # Metoda łącząca dwa kopce dwumianowe
        self.head = self.merge_trees(self.head, heap.head)

    def merge_trees(self, tree1, tree2):
        # Metoda rekurencyjnie łącząca dwa drzewa kopca dwumianowego
        if tree1 is None:
            return tree2
        elif tree2 is None:
            return tree1
        else:
            if tree1.value <= tree2.value:
                tree1.sibling = self.merge_trees(tree1.sibling, tree2)
                tree1.sibling.parent = tree1
                return tree1
            else:
                tree2.sibling = self.merge_trees(tree1, tree2.sibling)
                tree2.sibling.parent = tree2
                return tree2

    def insert(self, value):
        # Wstawianie elementu do kopca dwumianowego
        new_heap = BinomialHeap()
        new_heap.head = BinomialHeapNode(value)
        self.merge(new_heap)

    def get_min(self):
        # Znajdowanie najmniejszego elementu w kopcu dwumianowym
        if self.head is None:
            return None

        min_node = self.head
        current_node = self.head.sibling
        while current_node is not None:
            if current_node.value < min_node.value:
                min_node = current_node
            current_node = current_node.sibling

        return min_node.value

    def extract_min(self):
        # Usuwanie najmniejszego elementu z kopca dwumianowego
        if self.head is None:
            return None

        min_node = self.head
        prev_node = None
        current_node = self.head
        next_node = current_node.sibling

        while next_node is not None:
            if next_node.value < min_node.value:
                min_node = next_node
                prev_node = current_node
            current_node = next_node
            next_node = next_node.sibling

        if prev_node is None:
            self.head = min_node.sibling
        else:
            prev_node.sibling = min_node.sibling

        # Tworzenie nowego kopca z poddrzew usuwanego węzła
        new_heap = BinomialHeap()
        new_heap.head = min_node.child

        # Odwrócenie kolejności rodzeństwa w poddrzewach
        prev_node = None
        current_node = new_heap.head
        next_node = None
        while current_node is not None:
            next_node = current_node.sibling
            current_node.sibling = prev_node
            prev_node = current_node
            current_node = next_node

        # Łączenie kopca z odwróconym poddrzewem
        new_heap.head = prev_node

        # Łączenie nowego kopca z aktualnym kopcem
        self.merge(new_heap)

        return min_node.value
    
class FibonacciHeapNode:
    def __init__(self, value):
        self.value = value
        self.degree = 0
        self.parent = None
        self.child = None
        self.marked = False
        self.left = self
        self.right = self

class FibonacciHeap:
    def __init__(self):
        self.min_node = None
        self.num_nodes = 0

    def insert(self, value):
        new_node = FibonacciHeapNode(value)
        if self.min_node is None:
            self.min_node = new_node
        else:
            self._insert_node(new_node, self.min_node)
            if new_node.value < self.min_node.value:
                self.min_node = new_node
        self.num_nodes += 1

    def _insert_node(self, node, root):
        node.left = root
        node.right = root.right
        root.right = node
        node.right.left = node

    def get_min(self):
        if self.min_node is None:
            return None
        return self.min_node.value

    def extract_min(self):
        min_node = self.min_node
        if min_node is not None:
            if min_node.child is not None:
                # Przyłączamy dzieci do korzenia kopca
                child = min_node.child
                while True:
                    next_child = child.right
                    self._insert_node(child, self.min_node)
                    child.parent = None
                    child = next_child
                    if child == min_node.child:
                        break

            # Usuwamy min_node z korzeni kopca
            min_node.left.right = min_node.right
            min_node.right.left = min_node.left

            if min_node == min_node.right:
                self.min_node = None
            else:
                self.min_node = min_node.right
                self._consolidate()

            self.num_nodes -= 1

        return min_node.value

    def _consolidate(self):
        max_degree = int((self.num_nodes ** 0.5) + 1)
        degrees = [None] * max_degree

        current_node = self.min_node
        while True:
            degree = current_node.degree
            while degrees[degree] is not None:
                other_node = degrees[degree]
                if current_node.value > other_node.value:
                    current_node, other_node = other_node, current_node
                self._link_nodes(other_node, current_node)
                degrees[degree] = None
                degree += 1
            degrees[degree] = current_node

            current_node = current_node.right
            if current_node == self.min_node:
                break

        self.min_node = None
        for node in degrees:
            if node is not None:
                if self.min_node is None:
                    self.min_node = node
                else:
                    self._insert_node(node, self.min_node)
                    if node.value < self.min_node.value:
                        self.min_node = node

    def _link_nodes(self, child, parent):
        child.left.right = child.right
        child.right.left = child.left

        child.parent = parent
        if parent.child is None:
            parent.child = child
            child.right = child
            child.left = child
        else:
            child.left = parent.child
            child.right = parent.child.right
            parent.child.right = child
            child.right.left = child

        parent.degree += 1
        child.marked = False

    def decrease_key(self, node, new_value):
        if new_value > node.value:
            return

        node.value = new_value
        parent = node.parent
        if parent is not None and node.value < parent.value:
            self._cut(node, parent)
            self._cascading_cut(parent)
        if node.value < self.min_node.value:
            self.min_node = node

    def _cut(self, child, parent):
        child.left.right = child.right
        child.right.left = child.left
        parent.degree -= 1

        if parent.child == child:
            parent.child = child.right
        if parent.degree == 0:
            parent.child = None

        child.left = self.min_node
        child.right = self.min_node.right
        self.min_node.right = child
        child.right.left = child

        child.parent = None
        child.marked = False

    def _cascading_cut(self, node):
        parent = node.parent
        if parent is not None:
            if node.marked:
                self._cut(node, parent)
                self._cascading_cut(parent)
            else:
                node.marked = True

    def merge(self, other_heap):
        # Metoda łącząca dwa kopce Fibonacciego
        if other_heap.min_node is None:
            return

        if self.min_node is None:
            self.min_node = other_heap.min_node
        else:
            # Łączenie list korzeni
            self._concatenate_lists(self.min_node, other_heap.min_node)

            # Aktualizacja min_node
            if other_heap.min_node.value < self.min_node.value:
                self.min_node = other_heap.min_node

        self.num_nodes += other_heap.num_nodes

    def _concatenate_lists(self, node1, node2):
        # Metoda łącząca dwie listy węzłów
        temp = node1.right
        node1.right = node2.right
        node2.right.left = node1
        node2.right = temp
        temp.left = node2



heap1 = BinomialHeap()
heap1.insert(5)
heap1.insert(7)
heap1.insert(10)

heap2 = BinomialHeap()
heap2.insert(3)
heap2.insert(8)
heap2.insert(12)

heap1.merge(heap2)

min_value = heap1.get_min()
print(min_value)  # Output: 3

heap1 = FibonacciHeap()
heap1.insert(5)
heap1.insert(7)

heap2 = FibonacciHeap()
heap2.insert(3)
heap2.insert(8)

heap1.merge(heap2)

min_value = heap1.get_min()
print(min_value)  # Output: 3
