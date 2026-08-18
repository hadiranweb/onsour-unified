/**
 * ONSOUR Ecosystem: UIPT Bridge (TypeScript Glue Code)
 * This module provides a clean interface to interact with the Tanh-Brain Rust WASM core.
 * Updated for Foundation Level 1.2 (Correctness-First).
 */

/**
 * Minimal node structure (16 bytes in Rust)
 */
export interface UIPTNode {
    theta: number;
    e: number;
    ec: number;
    _padding?: number; // Optional padding for compatibility
}

/**
 * Practical node structure (32 bytes in Rust)
 * Optimized for cache alignment and ABI stability.
 */
export interface UIPTNodePractical {
    theta: number;
    theta_prev: number;
    e: number;
    ec: number;
    alpha: number;
    flags: number;
    _pad1?: number;
    _pad2?: number;
}

/**
 * Sparse Graph Relation (16 bytes in Rust)
 */
export interface UIPTRelation {
    src: number;
    dst: number;
    weight: number;
    _padding?: number;
}

export class TanhBrainEngine {
    private wasmModule: any;

    constructor(wasmModule: any) {
        this.wasmModule = wasmModule;
    }

    /**
     * Perform a single mathematical step for a standalone node.
     */
    public stepNode(node: UIPTNode, neighborSum: number): number {
        return this.wasmModule.step_node_math(node.theta, node.e, node.ec, neighborSum);
    }

    /**
     * Compute the excitatory channel alpha from order parameter theta.
     */
    public getAlpha(theta: number): number {
        return this.wasmModule.alpha(theta);
    }

    /**
     * Execute a sparse graph update across multiple nodes and edges.
     * Uses optimized WASM implementation with Double Buffering logic internally.
     */
    public updateGraph(nodes: UIPTNode[], edges: UIPTRelation[]): UIPTNode[] {
        // The step_sparse_js function in Rust handles Serde conversion
        // and enforces the Double Buffering invariant for correctness.
        return this.wasmModule.step_sparse_js(nodes, edges);
    }
}

/**
 * Example for using Tanh-Brain in the frontend.
 */
export const runTanhSimulation = (engine: TanhBrainEngine, initialNodes: UIPTNode[], relations: UIPTRelation[], steps: number) => {
    let currentNodes = [...initialNodes];
    const history: UIPTNode[][] = [currentNodes];

    for (let i = 0; i < steps; i++) {
        currentNodes = engine.updateGraph(currentNodes, relations);
        history.push(currentNodes);
    }

    return { finalState: currentNodes, history };
};
