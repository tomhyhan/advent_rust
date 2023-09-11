#  Rust with linked list X.X

class Node:
    def __init__(self, label):
        self.val = int(label)
        self.next  = None

    def __repr__(self) -> str:
        return f"{self.val}"
        
def generate_nodes(label):
    nodes = {}
    for l in label:
        nodes[l] = Node(int(l))

    for i in range(len(label)):
        one = label[i]
        if i == len(label) - 1:
            nodes[one].next = nodes[label[0]]    
            break
        two = label[i+1]
        nodes[one].next = nodes[two] 
    return nodes

def circulare_move(nodes, moves, start):
    head = nodes[start]
    values = [node.val for node in nodes.values()]
    min_val = min(values)
    max_val = max(values)

    for _ in range(moves):
        first = head.next
        second = head.next.next
        third = head.next.next.next

        des = head.val - 1
        while des in (first.val, second.val, third.val) or des < min_val:
            if des < min_val:
                des = max_val
                continue
            des -= 1
        # print(first.val, second.val, third.val)
        des_node = nodes[str(des)]  
        head.next = third.next
        temp = des_node.next
        des_node.next = first
        third.next = temp        
        head = head.next

    print(nodes["1"].next.val * nodes["1"].next.next.val)
        
def main():
    label = [val for val in "784235916"] + [str(num) for num in range(10,1000001)]
    nodes = generate_nodes(label)
    circulare_move(nodes, 10000000, label[0])
    
if "__main__" == __name__:
    main()