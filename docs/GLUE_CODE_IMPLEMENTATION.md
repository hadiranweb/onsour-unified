# راهنمای پیاده‌سازی لایه ارتباطی (Glue Code) بین Rust و TypeScript

## ۱. مقدمه
برای اتصال هسته ریاضی قدرتمند **Tanh-Brain** (پیاده‌سازی شده در زبان Rust در مسیر `backend/core/rts_core`) به بخش فرانت‌اند اکوسیستم ONSOUR (ساخته شده با Next.js / Remix)، از تکنولوژی **WebAssembly (WASM)** و کتابخانه `wasm-bindgen` استفاده می‌شود. این روش تضمین می‌کند که محاسبات پیچیده فیزیک آماری و معادلات `tanh` با سرعت بومی و بدون سربار مفسر در مرورگر اجرا شوند.

---

## ۲. تغییرات در هسته Rust (`rts_core`)

برای اینکه توابع Rust برای TypeScript قابل فراخوانی باشند، باید پکیج به صورت `cdylib` کامپایل شده و ساختارها و توابع با `#[wasm_bindgen]` مشخص شوند:

### الف) تنظیم `Cargo.toml`
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.6"
```

### ب) اکسپورت توابع گراف در `graph.rs`
```rust
use wasm_bindgen::prelude::*;
use crate::state::{Node, Edge};
use crate::math::step_node_math;

#[wasm_bindgen]
pub fn step_sparse_js(nodes: JsValue, edges: JsValue) -> JsValue {
    let mut nodes_vec: Vec<Node> = serde_wasm_bindgen::from_value(nodes).unwrap();
    let edges_vec: Vec<Edge> = serde_wasm_bindgen::from_value(edges).unwrap();
    
    // اجرای محاسبه فاز
    step_sparse_impl(&mut nodes_vec, &edges_vec);
    
    serde_wasm_bindgen::to_value(&nodes_vec).unwrap()
}
```

---

## ۳. لایه ارتباطی TypeScript (`uipt-bridge.ts`)

در سمت فرانت‌اند (مسیر `frontend/packages/uipt-bridge.ts`)، یک کلاس Wrapper برای مدیریت ماژول WASM و ارتباط با کامپوننت‌های React/Remix ایجاد شده است:

```typescript
export interface UIPTNode {
    theta: number;
    e: number;
    ec: number;
}

export interface UIPTRelation {
    src: number;
    dst: number;
    weight: number;
}

export class TanhBrainEngine {
    private wasmModule: any;

    constructor(wasmModule: any) {
        this.wasmModule = wasmModule;
    }

    public stepNode(node: UIPTNode, neighborSum: number): number {
        return this.wasmModule.step_node_math(node.theta, node.e, node.ec, neighborSum);
    }

    public updateGraph(nodes: UIPTNode[], edges: UIPTRelation[]): UIPTNode[] {
        return this.wasmModule.step_sparse_js(nodes, edges);
    }
}
```

---

## ۴. نحوه استفاده در کامپوننت‌های React / Remix

برای استفاده از این موتور در صفحات وب یا داشبوردهای مدیریتی ONSOUR:

```typescript
import { useEffect, useState } from 'react';
import { TanhBrainEngine, UIPTNode, UIPTRelation } from '@onsour/uipt-bridge';

export default function SimulationDashboard() {
    const [engine, setEngine] = useState<TanhBrainEngine | null>(null);
    const [nodes, setNodes] = useState<UIPTNode[]>([
        { theta: 0.1, e: 12.0, ec: 10.0 },
        { theta: -0.2, e: 8.0, ec: 10.0 }
    ]);
    const edges: UIPTRelation[] = [{ src: 0, dst: 1, weight: 0.5 }];

    useEffect(() => {
        // بارگذاری دینامیک ماژول WASM
        import('../pkg/rts_core').then((wasm) => {
            setEngine(new TanhBrainEngine(wasm));
        });
    }, []);

    const handleStep = () => {
        if (!engine) return;
        const nextNodes = engine.updateGraph(nodes, edges);
        setNodes([...nextNodes]);
    };

    return (
        <div className="p-6">
            <h1 className="text-2xl font-bold">Tanh-Brain Live Simulation</h1>
            <button onClick={handleStep} className="mt-4 px-4 py-2 bg-emerald-600 text-white rounded">
                Run Step
            </button>
            <pre className="mt-4 p-4 bg-slate-100 rounded">{JSON.stringify(nodes, null, 2)}</pre>
        </div>
    );
}
```
