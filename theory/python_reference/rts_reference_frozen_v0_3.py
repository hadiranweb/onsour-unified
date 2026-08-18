import math

def step_node(theta_prev, e, ec, neighbor_sum):
    """
    Core mathematical step for a single node.
    raw_i(t) = (E_i(t) - E_c,i) / E_c,i + neighbor_sum_i(t)
    theta_i(t+1) = tanh(raw_i(t))
    """
    if ec == 0:
        raw = neighbor_sum
    else:
        raw = (e - ec) / ec + neighbor_sum
    
    theta_new = math.tanh(raw)
    return theta_new

def get_alpha(theta):
    """
    alpha_i(t+1) = (theta_i(t+1) + 1) / 2
    """
    return (theta + 1) / 2.0

def step_sparse(nodes, edges):
    """
    nodes: list of dicts {'theta': f32, 'e': f32, 'ec': f32}
    edges: list of dicts {'src': int, 'dst': int, 'weight': f32}
    """
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
