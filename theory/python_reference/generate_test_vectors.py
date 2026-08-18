import json
import random
import hashlib
import os
import time
from rts_reference_frozen_v0_4 import step_sparse, get_alpha

def generate_vectors():
    random.seed(42)
    scenarios = []
    
    # 1. Scalar case
    scenarios.append({
        'name': 'scalar_case',
        'nodes': [{'theta': 0.1, 'e': 12.0, 'ec': 10.0}],
        'edges': [],
        'steps': 1
    })
    
    # 2. Short trajectory
    nodes_short = [{'theta': random.uniform(-1, 1), 'e': random.uniform(5, 15), 'ec': 10.0} for _ in range(10)]
    edges_short = [{'src': random.randint(0, 9), 'dst': random.randint(0, 9), 'weight': random.uniform(-0.1, 0.1)} for _ in range(20)]
    scenarios.append({
        'name': 'short_trajectory',
        'nodes': nodes_short,
        'edges': edges_short,
        'steps': 100
    })
    
    # 3. Long trajectory (OFFICIAL v0.4)
    N_long = 100
    M_long = 400
    T_long = 10000
    nodes_long = [{'theta': random.uniform(-1, 1), 'e': random.uniform(5, 15), 'ec': 10.0} for _ in range(N_long)]
    edges_long = [{'src': random.randint(0, N_long-1), 'dst': random.randint(0, N_long-1), 'weight': random.uniform(-0.05, 0.05)} for _ in range(M_long)]
    scenarios.append({
        'name': 'long_trajectory',
        'nodes': nodes_long,
        'edges': edges_long,
        'steps': T_long
    })

    # Run reference
    for scenario in scenarios:
        print(f"Generating {scenario['name']}...")
        nodes = [dict(n) for n in scenario['nodes']]
        edges = scenario['edges']
        trajectory = []
        # For long trajectory, we only store final state to keep JSON small
        for t in range(scenario['steps']):
            nodes = step_sparse(nodes, edges)
            if scenario['name'] != 'long_trajectory' or t == scenario['steps'] - 1:
                state = [n['theta'] for n in nodes]
                alphas = [get_alpha(n['theta']) for n in nodes]
                trajectory.append({'step': t, 'thetas': state, 'alphas': alphas})
        scenario['expected_trajectory'] = trajectory

    with open('test_vectors.json', 'w') as f:
        json.dump(scenarios, f, indent=2)
    
    return 'test_vectors.json'

def create_manifest(python_file, vector_file):
    def get_sha256(fname):
        hash_sha256 = hashlib.sha256()
        with open(fname, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                hash_sha256.update(chunk)
        return hash_sha256.hexdigest()

    manifest = {
        "spec_version": "0.4",
        "frozen_date": "2026-07-17",
        "files": {
            os.path.basename(python_file): {
                "sha256": get_sha256(python_file),
                "frozen": True
            },
            os.path.basename(vector_file): {
                "sha256": get_sha256(vector_file),
                "frozen": True
            }
        },
        "invalidation_rule": "any change to frozen files requires version bump and full Track B/C rerun"
    }
    
    with open('reference_manifest.json', 'w') as f:
        json.dump(manifest, f, indent=2)

if __name__ == "__main__":
    vec_file = generate_vectors()
    create_manifest('rts_reference_frozen_v0_4.py', vec_file)
    print("Test vectors v0.4 and manifest generated.")
