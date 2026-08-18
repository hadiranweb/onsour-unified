import math

def step_node(theta_prev, e, ec, neighbor_sum):
    """
    Core mathematical step for a single node (v0.4).
    """
    if ec == 0:
        raw = neighbor_sum
    else:
        raw = (e - ec) / ec + neighbor_sum
    
    theta_new = math.tanh(raw)
    return theta_new

def get_alpha(theta):
    return (theta + 1) / 2.0

def step_sparse(nodes, edges):
    n = len(nodes)
    neighbor_sums = [0.0] * n
    
    for edge in edges:
        src = edge['src']
        dst = edge['dst']
        weight = edge['weight']
        neighbor_sums[dst] += weight * nodes[src]['theta']
        
    new_thetas = []
    for i in range(n):
        new_theta = step_node(nodes[i]['theta'], nodes[i]['e'], nodes[i]['ec'], neighbor_sums[i])
        new_thetas.append(new_theta)
        
    for i in range(n):
        nodes[i]['theta'] = new_thetas[i]
    
    return nodes
