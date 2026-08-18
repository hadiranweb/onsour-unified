# مرجع APIهای اکوسیستم ONSOUR (Foundation Level 1.2)

این سند مرجع رسمی APIهای هسته ران‌تایم و لایه ارتباطی در نسخه ۱.۲ است که با تمرکز بر **صحت (Correctness-First)** طراحی شده است.

---

## ۱. مرجع هسته Rust (`rts_core`)

### الف) ساختارهای داده (`state.rs`)
تمامی ساختارها از ویژگی `#[repr(C)]` برای پایداری ABI استفاده می‌کنند:
- `Node`: نود مینیمال (اندازه: ۱۶ بایت، تراز: ۱۶ بایت، شامل پدینگ صریح).
- `NodePractical`: نود کاربردی (اندازه: ۳۲ بایت، تراز: ۳۲ بایت، طراحی شده برای حذف کامل Cache-Line Straddling).
- `Edge`: یال گراف تنک (اندازه: ۱۶ بایت).

### ب) مدیریت گراف و Double Buffering (`graph.rs`)
- `step_sparse_buffered(current_nodes: &[Node], next_nodes: &mut [Node], edges: &[Edge])`: اجرای گام زمانی با مدل Double Buffering. تضمین می‌کند که `State(t)` در طول اپوک تغییر ناپذیر باقی بماند.
- `step_sparse_impl(nodes: &mut [Node], edges: &[Edge])`: نسخه کپسوله‌شده برای سازگاری تک‌بافر.
- `step_sparse_js(nodes: JsValue, edges: JsValue) -> JsValue`: تابع واسط WebAssembly با تبدیل خودکار Serde.

---

## ۲. مرجع لایه ارتباطی TypeScript (`uipt-bridge.ts`)

کلاس `TanhBrainEngine` در بسته فرانت‌اند پل ارتباطی میان رابط کاربری و ماژول WASM را برقرار می‌کند:

```typescript
export class TanhBrainEngine {
    constructor(wasmModule: any);
    public stepNode(node: UIPTNode, neighborSum: number): number;
    public getAlpha(theta: number): number;
    public updateGraph(nodes: UIPTNode[], edges: UIPTRelation[]): UIPTNode[];
}
```
